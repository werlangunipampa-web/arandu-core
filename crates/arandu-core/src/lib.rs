use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use arandu_confluence::{ConfluenceController, ConfluenceEvent, ConfluenceSignal, ConfluenceState};
use arandu_geometry::{AdaptiveLayout, GeometryEngine, Point, RiverCounts, Viewport};
use arandu_model::{
    ApplicationId, ApplicationNode, Confluence, RecentMemory, RiverKind, RiverState,
};
use arandu_navigation::{AranduEvent, NavigationIntent, NavigationState, UsageStatistics};
use arandu_platform_api::{PlatformAdapter, PlatformError};

#[derive(Debug)]
pub struct AranduCore {
    applications: HashMap<ApplicationId, ApplicationNode>,
    recent: RecentMemory,
    usage: UsageStatistics,
    open_ids: Vec<ApplicationId>,
    navigation: NavigationState,
    confluence: Confluence,
    confluence_controller: ConfluenceController,
    geometry: GeometryEngine,
    frequent_limit: usize,
}

impl Default for AranduCore {
    fn default() -> Self {
        Self::new(5)
    }
}

impl AranduCore {
    /// Creates the universal navigation core with the same capacity for each river.
    ///
    /// # Panics
    ///
    /// Panics when `river_capacity` is zero.
    #[must_use]
    pub fn new(river_capacity: usize) -> Self {
        Self {
            applications: HashMap::new(),
            recent: RecentMemory::new(river_capacity),
            usage: UsageStatistics::default(),
            open_ids: Vec::new(),
            navigation: NavigationState::new(Vec::new(), Vec::new(), Vec::new()),
            confluence: Confluence::default(),
            confluence_controller: ConfluenceController::default(),
            geometry: GeometryEngine::default(),
            frequent_limit: river_capacity,
        }
    }

    #[must_use]
    pub fn confluence(&self) -> &Confluence {
        &self.confluence
    }

    #[must_use]
    pub fn confluence_state(&self) -> ConfluenceState {
        self.confluence_controller.state()
    }

    #[must_use]
    pub fn confluence_reveal_factor(&self) -> f32 {
        self.confluence_controller.reveal_factor()
    }

    #[must_use]
    pub fn handle_confluence_signal(
        &mut self,
        signal: ConfluenceSignal,
        now: Duration,
    ) -> Vec<ConfluenceEvent> {
        self.confluence_controller.handle(signal, now)
    }

    /// Computes renderer-independent positions for the Confluence Seed and all
    /// currently populated rivers. Platform adapters only provide viewport size
    /// and the requested seed position; the Rust core decides the geometry.
    #[must_use]
    pub fn adaptive_layout(&self, viewport: Viewport, requested_seed: Point) -> AdaptiveLayout {
        self.geometry.layout(
            viewport,
            requested_seed,
            RiverCounts {
                memory: self.river(RiverKind::Memory).nodes.len(),
                open: self.river(RiverKind::Open).nodes.len(),
                frequent: self.river(RiverKind::Frequent).nodes.len(),
            },
        )
    }

    pub fn register_application(&mut self, app: ApplicationNode) {
        self.applications.insert(app.id.clone(), app);
    }

    #[must_use]
    pub fn application(&self, id: &ApplicationId) -> Option<&ApplicationNode> {
        self.applications.get(id)
    }

    #[must_use]
    pub fn river(&self, kind: RiverKind) -> &RiverState {
        self.navigation.river(kind)
    }

    #[must_use]
    pub fn observe_focus(&mut self, id: &ApplicationId) -> Vec<AranduEvent> {
        if !self.applications.contains_key(id) {
            return Vec::new();
        }

        for app in self.applications.values_mut() {
            app.is_focused = false;
        }
        if let Some(app) = self.applications.get_mut(id) {
            app.is_focused = true;
            app.last_used_at = Some(SystemTime::now());
            app.usage_count = app.usage_count.saturating_add(1);
        }

        self.recent.observe(id.clone());
        self.usage.record_focus(id);
        self.refresh_derived_rivers();
        vec![AranduEvent::MemoryUpdated, AranduEvent::FrequencyUpdated]
    }

    #[must_use]
    pub fn observe_launch(&mut self, id: &ApplicationId) -> Vec<AranduEvent> {
        if let Some(app) = self.applications.get_mut(id) {
            app.is_open = true;
            self.usage.record_launch(id);
            if !self.open_ids.contains(id) {
                self.open_ids.push(id.clone());
            }
            self.refresh_derived_rivers();
            vec![
                AranduEvent::OpenApplicationsUpdated,
                AranduEvent::FrequencyUpdated,
            ]
        } else {
            Vec::new()
        }
    }

    #[must_use]
    pub fn observe_closed(&mut self, id: &ApplicationId) -> Vec<AranduEvent> {
        if let Some(app) = self.applications.get_mut(id) {
            app.is_open = false;
            app.is_focused = false;
        }
        self.open_ids.retain(|open_id| open_id != id);
        self.refresh_derived_rivers();
        vec![AranduEvent::OpenApplicationsUpdated]
    }

    #[must_use]
    pub fn observe_active_duration(
        &mut self,
        id: &ApplicationId,
        duration: Duration,
    ) -> Vec<AranduEvent> {
        if let Some(app) = self.applications.get_mut(id) {
            app.accumulated_active += duration;
            self.usage.record_active_duration(id, duration);
            self.refresh_derived_rivers();
            vec![AranduEvent::FrequencyUpdated]
        } else {
            Vec::new()
        }
    }

    #[must_use]
    pub fn handle_intent(&mut self, intent: NavigationIntent) -> Vec<AranduEvent> {
        self.navigation.handle(intent)
    }

    /// Synchronizes open/focused application state from a platform adapter.
    ///
    /// # Errors
    ///
    /// Returns the platform adapter error if the open-application or focused-application
    /// query fails.
    pub fn synchronize_platform<P: PlatformAdapter>(
        &mut self,
        platform: &P,
    ) -> Result<Vec<AranduEvent>, PlatformError> {
        let open = platform.list_open_applications()?;
        let focused = platform.focused_application()?;

        for app in self.applications.values_mut() {
            app.is_open = false;
            app.is_focused = false;
        }

        self.open_ids.clear();
        for mut app in open {
            app.is_open = true;
            self.open_ids.push(app.id.clone());
            self.applications.insert(app.id.clone(), app);
        }

        let mut events = vec![AranduEvent::OpenApplicationsUpdated];
        if let Some(mut app) = focused {
            app.is_open = true;
            app.is_focused = true;
            let id = app.id.clone();
            self.applications.insert(id.clone(), app);
            events.extend(self.observe_focus(&id));
        } else {
            self.refresh_derived_rivers();
        }

        Ok(events)
    }

    fn refresh_derived_rivers(&mut self) {
        self.navigation
            .replace_river(RiverKind::Memory, self.recent.as_vec());
        self.navigation
            .replace_river(RiverKind::Open, self.open_ids.clone());
        self.navigation
            .replace_river(RiverKind::Frequent, self.usage.top(self.frequent_limit));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arandu_model::ApplicationNode;
    use arandu_navigation::NavigationIntent;

    fn app(id: &str) -> ApplicationNode {
        ApplicationNode::new(ApplicationId::new(id), id, format!("{id}.desktop"))
    }

    #[test]
    fn core_builds_three_rivers_from_observations() {
        let mut core = AranduCore::new(5);
        for id in ["firefox", "terminal", "files"] {
            core.register_application(app(id));
        }

        let _ = core.observe_launch(&ApplicationId::new("firefox"));
        let _ = core.observe_focus(&ApplicationId::new("firefox"));
        let _ = core.observe_focus(&ApplicationId::new("terminal"));
        let _ = core.observe_focus(&ApplicationId::new("files"));
        let _ =
            core.observe_active_duration(&ApplicationId::new("firefox"), Duration::from_mins(10));

        assert_eq!(
            core.river(RiverKind::Memory).nodes[0],
            ApplicationId::new("files")
        );
        assert!(core
            .river(RiverKind::Open)
            .nodes
            .contains(&ApplicationId::new("firefox")));
        assert_eq!(
            core.river(RiverKind::Frequent).nodes[0],
            ApplicationId::new("firefox")
        );
    }

    #[test]
    fn intents_are_independent_from_input_modality() {
        let mut core = AranduCore::default();
        core.register_application(app("firefox"));
        let _ = core.observe_focus(&ApplicationId::new("firefox"));
        let events = core.handle_intent(NavigationIntent::SelectRiver(RiverKind::Memory));
        assert!(matches!(
            events[0],
            AranduEvent::RiverChanged(RiverKind::Memory)
        ));
    }
    #[test]
    fn core_exposes_device_independent_confluence_seed() {
        let mut core = AranduCore::default();
        let _ = core.handle_confluence_signal(
            ConfluenceSignal::Proximity(0.80),
            Duration::from_millis(100),
        );
        assert_eq!(core.confluence_state(), ConfluenceState::Near);

        let _ = core
            .handle_confluence_signal(ConfluenceSignal::HoverEntered, Duration::from_millis(200));
        assert_eq!(core.confluence_state(), ConfluenceState::Expanded);
        assert!((core.confluence_reveal_factor() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn core_geometry_uses_live_river_counts_and_stays_in_viewport() {
        let mut core = AranduCore::new(5);
        for id in ["firefox", "terminal", "files", "editor", "notes"] {
            core.register_application(app(id));
            let _ = core.observe_launch(&ApplicationId::new(id));
            let _ = core.observe_focus(&ApplicationId::new(id));
        }

        let viewport = Viewport::new(800.0, 600.0);
        let layout = core.adaptive_layout(viewport, Point::new(60.0, 550.0));

        assert_eq!(layout.memory.nodes.len(), 5);
        assert_eq!(layout.open.nodes.len(), 5);
        assert_eq!(layout.frequent.nodes.len(), 5);
        for kind in RiverKind::ALL {
            assert!(layout.river(kind).nodes.iter().all(|point| {
                point.x >= 44.0
                    && point.x <= viewport.width - 44.0
                    && point.y >= 44.0
                    && point.y <= viewport.height - 44.0
            }));
        }
    }
}
