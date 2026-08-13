use std::collections::HashMap;
use std::time::Duration;

use arandu_model::{ApplicationId, RiverKind, RiverState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationIntent {
    SelectRiver(RiverKind),
    NextNode,
    PreviousNode,
    ActivateNode,
    ReturnToConfluence,
    OpenControlCenter,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AranduEvent {
    RiverChanged(RiverKind),
    NodeSelected {
        river: RiverKind,
        app: ApplicationId,
    },
    NodeActivated {
        river: RiverKind,
        app: ApplicationId,
    },
    ReturnedToConfluence,
    ControlCenterRequested,
    Cancelled,
    MemoryUpdated,
    FrequencyUpdated,
    OpenApplicationsUpdated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UsageCounters {
    pub launch_count: u64,
    pub focus_count: u64,
    pub active_seconds: u64,
}

impl UsageCounters {
    #[must_use]
    pub fn score(self) -> u128 {
        u128::from(self.launch_count)
            + u128::from(self.focus_count)
            + u128::from(self.active_seconds / 60)
    }
}

#[derive(Debug, Default, Clone)]
pub struct UsageStatistics {
    counters: HashMap<ApplicationId, UsageCounters>,
}

impl UsageStatistics {
    pub fn record_launch(&mut self, id: &ApplicationId) {
        self.counters.entry(id.clone()).or_default().launch_count += 1;
    }

    pub fn record_focus(&mut self, id: &ApplicationId) {
        self.counters.entry(id.clone()).or_default().focus_count += 1;
    }

    pub fn record_active_duration(&mut self, id: &ApplicationId, duration: Duration) {
        self.counters.entry(id.clone()).or_default().active_seconds += duration.as_secs();
    }

    #[must_use]
    pub fn counters(&self, id: &ApplicationId) -> UsageCounters {
        self.counters.get(id).copied().unwrap_or_default()
    }

    #[must_use]
    pub fn top(&self, limit: usize) -> Vec<ApplicationId> {
        let mut ranked: Vec<_> = self.counters.iter().collect();
        ranked.sort_by(|(id_a, counters_a), (id_b, counters_b)| {
            counters_b
                .score()
                .cmp(&counters_a.score())
                .then_with(|| id_a.cmp(id_b))
        });
        ranked
            .into_iter()
            .take(limit)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct NavigationState {
    pub active_river: Option<RiverKind>,
    rivers: HashMap<RiverKind, RiverState>,
}

impl NavigationState {
    #[must_use]
    pub fn new(
        memory: Vec<ApplicationId>,
        open: Vec<ApplicationId>,
        frequent: Vec<ApplicationId>,
    ) -> Self {
        let mut rivers = HashMap::new();
        rivers.insert(
            RiverKind::Memory,
            RiverState::new(RiverKind::Memory, memory),
        );
        rivers.insert(RiverKind::Open, RiverState::new(RiverKind::Open, open));
        rivers.insert(
            RiverKind::Frequent,
            RiverState::new(RiverKind::Frequent, frequent),
        );
        Self {
            active_river: None,
            rivers,
        }
    }

    /// Returns the state for one of the three ARANDU rivers.
    ///
    /// # Panics
    ///
    /// Panics only if the internal invariant is broken and a canonical ARANDU river
    /// is missing from the state map. Every constructor initializes all three rivers.
    #[must_use]
    pub fn river(&self, kind: RiverKind) -> &RiverState {
        self.rivers
            .get(&kind)
            .expect("all ARANDU rivers are initialized")
    }

    /// Returns mutable state for one of the three ARANDU rivers.
    ///
    /// # Panics
    ///
    /// Panics only if the internal invariant is broken and a canonical ARANDU river
    /// is missing from the state map. Every constructor initializes all three rivers.
    #[must_use]
    pub fn river_mut(&mut self, kind: RiverKind) -> &mut RiverState {
        self.rivers
            .get_mut(&kind)
            .expect("all ARANDU rivers are initialized")
    }

    pub fn replace_river(&mut self, kind: RiverKind, nodes: Vec<ApplicationId>) {
        self.rivers.insert(kind, RiverState::new(kind, nodes));
    }

    #[must_use]
    pub fn handle(&mut self, intent: NavigationIntent) -> Vec<AranduEvent> {
        match intent {
            NavigationIntent::SelectRiver(kind) => {
                self.active_river = Some(kind);
                let mut events = vec![AranduEvent::RiverChanged(kind)];
                if let Some(app) = self.river(kind).selected().cloned() {
                    events.push(AranduEvent::NodeSelected { river: kind, app });
                }
                events
            }
            NavigationIntent::NextNode => self.move_selection(true),
            NavigationIntent::PreviousNode => self.move_selection(false),
            NavigationIntent::ActivateNode => {
                let Some(kind) = self.active_river else {
                    return Vec::new();
                };
                self.river(kind)
                    .selected()
                    .cloned()
                    .map(|app| vec![AranduEvent::NodeActivated { river: kind, app }])
                    .unwrap_or_default()
            }
            NavigationIntent::ReturnToConfluence => {
                self.active_river = None;
                vec![AranduEvent::ReturnedToConfluence]
            }
            NavigationIntent::OpenControlCenter => vec![AranduEvent::ControlCenterRequested],
            NavigationIntent::Cancel => vec![AranduEvent::Cancelled],
        }
    }

    fn move_selection(&mut self, next: bool) -> Vec<AranduEvent> {
        let Some(kind) = self.active_river else {
            return Vec::new();
        };
        let river = self.river_mut(kind);
        if next {
            river.select_next();
        } else {
            river.select_previous();
        }
        river
            .selected()
            .cloned()
            .map(|app| vec![AranduEvent::NodeSelected { river: kind, app }])
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_ranking_uses_raw_counters_deterministically() {
        let a = ApplicationId::new("a");
        let b = ApplicationId::new("b");
        let mut stats = UsageStatistics::default();
        stats.record_launch(&a);
        stats.record_focus(&a);
        stats.record_active_duration(&a, Duration::from_mins(2));
        stats.record_launch(&b);
        stats.record_launch(&b);
        stats.record_launch(&b);
        assert_eq!(stats.top(2), vec![a, b]);
    }

    #[test]
    fn selecting_a_river_then_next_emits_selection_event() {
        let mut nav = NavigationState::new(
            vec![ApplicationId::new("a"), ApplicationId::new("b")],
            Vec::new(),
            Vec::new(),
        );
        let events = nav.handle(NavigationIntent::SelectRiver(RiverKind::Memory));
        assert!(matches!(
            events[0],
            AranduEvent::RiverChanged(RiverKind::Memory)
        ));
        let events = nav.handle(NavigationIntent::NextNode);
        assert_eq!(
            events,
            vec![AranduEvent::NodeSelected {
                river: RiverKind::Memory,
                app: ApplicationId::new("b"),
            }]
        );
    }
}
