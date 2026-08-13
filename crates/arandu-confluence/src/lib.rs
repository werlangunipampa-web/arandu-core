use std::time::Duration;

/// Visual/interaction state of the ARANDU Confluence Seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfluenceState {
    /// Only the central seed is visible.
    #[default]
    Collapsed,
    /// The pointer or another proximity source entered the awareness zone.
    Aware,
    /// The interaction source is close enough for stronger visual anticipation.
    Near,
    /// The three rivers are fully revealed temporarily.
    Expanded,
    /// The three rivers remain open until explicitly unpinned or collapsed.
    Pinned,
}

/// Device-independent signals understood by the Confluence Seed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfluenceSignal {
    /// Normalized proximity where `0.0` is far and `1.0` is at the seed.
    Proximity(f32),
    /// Direct hover/contact with the central seed.
    HoverEntered,
    /// Requests a temporary full expansion.
    Expand,
    /// Toggles persistent expansion.
    TogglePin,
    /// Explicitly collapses the seed, even when pinned.
    Collapse,
    /// Refreshes the inactivity timer while the user interacts with an open river.
    Activity,
    /// Advances inactivity evaluation without imposing a device or event loop.
    Tick,
}

/// State-machine events for renderers and platform adapters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfluenceEvent {
    StateChanged {
        from: ConfluenceState,
        to: ConfluenceState,
    },
    ProximityChanged {
        normalized: f32,
    },
    PinChanged {
        pinned: bool,
    },
    AutoCollapsed,
}

/// Configuration for progressive awakening and automatic recollection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfluenceConfig {
    pub aware_proximity: f32,
    pub near_proximity: f32,
    pub expand_proximity: f32,
    pub auto_collapse_after: Duration,
}

impl Default for ConfluenceConfig {
    fn default() -> Self {
        Self {
            aware_proximity: 0.35,
            near_proximity: 0.70,
            expand_proximity: 0.95,
            auto_collapse_after: Duration::from_secs(4),
        }
    }
}

/// Deterministic, platform-independent state machine for the Confluence Seed.
#[derive(Debug, Clone)]
pub struct ConfluenceController {
    state: ConfluenceState,
    proximity: f32,
    last_activity_at: Duration,
    config: ConfluenceConfig,
}

impl Default for ConfluenceController {
    fn default() -> Self {
        Self::new(ConfluenceConfig::default())
    }
}

impl ConfluenceController {
    #[must_use]
    pub fn new(config: ConfluenceConfig) -> Self {
        Self {
            state: ConfluenceState::Collapsed,
            proximity: 0.0,
            last_activity_at: Duration::ZERO,
            config: normalize_config(config),
        }
    }

    #[must_use]
    pub fn state(&self) -> ConfluenceState {
        self.state
    }

    #[must_use]
    pub fn proximity(&self) -> f32 {
        self.proximity
    }

    /// Returns a renderer-friendly reveal factor without choosing any toolkit.
    #[must_use]
    pub fn reveal_factor(&self) -> f32 {
        match self.state {
            ConfluenceState::Collapsed => 0.0,
            ConfluenceState::Aware | ConfluenceState::Near => self.proximity,
            ConfluenceState::Expanded | ConfluenceState::Pinned => 1.0,
        }
    }

    /// Processes a signal at a caller-supplied monotonic logical time.
    ///
    /// Passing time from the host instead of reading a system clock keeps the
    /// state machine deterministic and testable on Linux, macOS and Windows.
    #[must_use]
    pub fn handle(&mut self, signal: ConfluenceSignal, now: Duration) -> Vec<ConfluenceEvent> {
        match signal {
            ConfluenceSignal::Proximity(value) => self.handle_proximity(value, now),
            ConfluenceSignal::HoverEntered | ConfluenceSignal::Expand => {
                self.last_activity_at = now;
                self.transition_to(ConfluenceState::Expanded)
            }
            ConfluenceSignal::TogglePin => {
                self.last_activity_at = now;
                if self.state == ConfluenceState::Pinned {
                    let mut events = self.transition_to(ConfluenceState::Expanded);
                    events.push(ConfluenceEvent::PinChanged { pinned: false });
                    events
                } else {
                    let mut events = self.transition_to(ConfluenceState::Pinned);
                    events.push(ConfluenceEvent::PinChanged { pinned: true });
                    events
                }
            }
            ConfluenceSignal::Collapse => {
                self.proximity = 0.0;
                self.last_activity_at = now;
                let was_pinned = self.state == ConfluenceState::Pinned;
                let mut events = self.transition_to(ConfluenceState::Collapsed);
                if was_pinned {
                    events.push(ConfluenceEvent::PinChanged { pinned: false });
                }
                events
            }
            ConfluenceSignal::Activity => {
                self.last_activity_at = now;
                Vec::new()
            }
            ConfluenceSignal::Tick => self.handle_tick(now),
        }
    }

    fn handle_proximity(&mut self, value: f32, now: Duration) -> Vec<ConfluenceEvent> {
        let normalized = normalize_unit(value);
        self.proximity = normalized;
        self.last_activity_at = now;

        let mut events = vec![ConfluenceEvent::ProximityChanged { normalized }];
        if self.state == ConfluenceState::Pinned || self.state == ConfluenceState::Expanded {
            return events;
        }

        let target = if normalized >= self.config.expand_proximity {
            ConfluenceState::Expanded
        } else if normalized >= self.config.near_proximity {
            ConfluenceState::Near
        } else if normalized >= self.config.aware_proximity {
            ConfluenceState::Aware
        } else {
            ConfluenceState::Collapsed
        };
        events.extend(self.transition_to(target));
        events
    }

    fn handle_tick(&mut self, now: Duration) -> Vec<ConfluenceEvent> {
        if matches!(
            self.state,
            ConfluenceState::Collapsed | ConfluenceState::Pinned
        ) {
            return Vec::new();
        }

        if now.saturating_sub(self.last_activity_at) < self.config.auto_collapse_after {
            return Vec::new();
        }

        self.proximity = 0.0;
        let mut events = self.transition_to(ConfluenceState::Collapsed);
        events.push(ConfluenceEvent::AutoCollapsed);
        events
    }

    fn transition_to(&mut self, target: ConfluenceState) -> Vec<ConfluenceEvent> {
        if self.state == target {
            return Vec::new();
        }
        let from = self.state;
        self.state = target;
        vec![ConfluenceEvent::StateChanged { from, to: target }]
    }
}

fn normalize_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn normalize_config(config: ConfluenceConfig) -> ConfluenceConfig {
    let aware = if config.aware_proximity.is_finite() {
        config.aware_proximity.clamp(0.0, 1.0)
    } else {
        0.35
    };
    let near = if config.near_proximity.is_finite() {
        config.near_proximity.clamp(aware, 1.0)
    } else {
        0.70_f32.max(aware)
    };
    let expand = if config.expand_proximity.is_finite() {
        config.expand_proximity.clamp(near, 1.0)
    } else {
        0.95_f32.max(near)
    };

    ConfluenceConfig {
        aware_proximity: aware,
        near_proximity: near,
        expand_proximity: expand,
        auto_collapse_after: config.auto_collapse_after,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proximity_wakes_seed_progressively() {
        let mut controller = ConfluenceController::default();

        let _ = controller.handle(
            ConfluenceSignal::Proximity(0.40),
            Duration::from_millis(100),
        );
        assert_eq!(controller.state(), ConfluenceState::Aware);

        let _ = controller.handle(
            ConfluenceSignal::Proximity(0.75),
            Duration::from_millis(200),
        );
        assert_eq!(controller.state(), ConfluenceState::Near);

        let _ = controller.handle(
            ConfluenceSignal::Proximity(0.98),
            Duration::from_millis(300),
        );
        assert_eq!(controller.state(), ConfluenceState::Expanded);
    }

    #[test]
    fn expanded_seed_auto_collapses_after_inactivity() {
        let mut controller = ConfluenceController::default();
        let _ = controller.handle(ConfluenceSignal::HoverEntered, Duration::ZERO);
        assert_eq!(controller.state(), ConfluenceState::Expanded);

        let events = controller.handle(ConfluenceSignal::Tick, Duration::from_secs(4));
        assert_eq!(controller.state(), ConfluenceState::Collapsed);
        assert!(events.contains(&ConfluenceEvent::AutoCollapsed));
    }

    #[test]
    fn activity_keeps_expanded_rivers_alive() {
        let mut controller = ConfluenceController::default();
        let _ = controller.handle(ConfluenceSignal::HoverEntered, Duration::ZERO);
        let _ = controller.handle(ConfluenceSignal::Activity, Duration::from_secs(3));
        let _ = controller.handle(ConfluenceSignal::Tick, Duration::from_secs(6));
        assert_eq!(controller.state(), ConfluenceState::Expanded);
    }

    #[test]
    fn pinned_seed_ignores_auto_collapse() {
        let mut controller = ConfluenceController::default();
        let _ = controller.handle(ConfluenceSignal::TogglePin, Duration::ZERO);
        let _ = controller.handle(ConfluenceSignal::Tick, Duration::from_hours(1));
        assert_eq!(controller.state(), ConfluenceState::Pinned);
    }

    #[test]
    fn explicit_collapse_overrides_pinned_state() {
        let mut controller = ConfluenceController::default();
        let _ = controller.handle(ConfluenceSignal::TogglePin, Duration::ZERO);
        let events = controller.handle(ConfluenceSignal::Collapse, Duration::from_secs(1));
        assert_eq!(controller.state(), ConfluenceState::Collapsed);
        assert!(events.contains(&ConfluenceEvent::PinChanged { pinned: false }));
    }
}
