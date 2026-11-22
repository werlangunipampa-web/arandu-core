//! A1.4 — isolated bridge from normalized keyboard events to Rust core.
//!
//! This is not a native window, input hook, display server or Wayland compositor.
//! It runs only when explicitly called by an application.

use arandu_confluence::ConfluenceEvent;
use arandu_core::AranduCore;
use arandu_input_api::desktop_keyboard::{DesktopAction, DesktopKey, KeyTransition, map_key};
use arandu_navigation::AranduEvent;
use std::time::Duration;

/// Events returned to a future renderer after a single accepted keypress.
#[derive(Debug, Clone, PartialEq)]
pub struct DispatchEffects {
    pub confluence: Vec<ConfluenceEvent>,
    pub navigation: Vec<AranduEvent>,
}

/// Dispatches one normalized key event through the existing Rust core.
/// An unrecognized key or a key release does not mutate core state.
#[must_use]
pub fn dispatch(
    core: &mut AranduCore,
    key: DesktopKey,
    transition: KeyTransition,
    now: Duration,
) -> Option<DispatchEffects> {
    let action = map_key(key, transition)?;
    let mut effects = DispatchEffects {
        confluence: Vec::new(),
        navigation: Vec::new(),
    };
    match action {
        DesktopAction::Confluence(signal) => {
            effects.confluence = core.handle_confluence_signal(signal, now);
        }
        DesktopAction::Navigation(intent) => {
            effects.navigation = core.handle_intent(intent);
            effects.confluence = core.handle_confluence_signal(
                arandu_confluence::ConfluenceSignal::Activity,
                now,
            );
        }
    }
    Some(effects)
}

/// Prints a synthetic sample; no real OS input is captured.
pub fn demo() -> String {
    let mut core = AranduCore::default();
    let _ = dispatch(
        &mut core,
        DesktopKey::Digit2,
        KeyTransition::Press,
        Duration::from_secs(1),
    );
    format!(
        "ARANDU A1.4 • entrada sintética • Confluência: {:?}\n",
        core.confluence_state()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use arandu_confluence::ConfluenceState;
    use arandu_model::{ApplicationId, ApplicationNode, RiverKind};

    #[test]
    fn keyboard_expands_and_collapses_via_real_rust_state_machine() {
        let mut core = AranduCore::default();
        let expanded = dispatch(
            &mut core,
            DesktopKey::Digit2,
            KeyTransition::Press,
            Duration::from_secs(1),
        );
        assert!(expanded.is_some());
        assert_eq!(core.confluence_state(), ConfluenceState::Expanded);
        let collapsed = dispatch(
            &mut core,
            DesktopKey::Escape,
            KeyTransition::Press,
            Duration::from_secs(2),
        );
        assert!(collapsed.is_some());
        assert_eq!(core.confluence_state(), ConfluenceState::Collapsed);
    }

    #[test]
    fn key_release_and_unknown_input_have_no_side_effects() {
        let mut core = AranduCore::default();
        assert!(dispatch(
            &mut core,
            DesktopKey::Digit2,
            KeyTransition::Release,
            Duration::ZERO
        )
        .is_none());
        assert!(dispatch(
            &mut core,
            DesktopKey::Other,
            KeyTransition::Press,
            Duration::ZERO
        )
        .is_none());
        assert_eq!(core.confluence_state(), ConfluenceState::Collapsed);
    }

    #[test]
    fn selecting_memory_routes_to_existing_navigation_engine() {
        let mut core = AranduCore::default();
        let id = ApplicationId::new("synthetic-editor");
        core.register_application(ApplicationNode::new(
            id.clone(),
            "Editor",
            "synthetic-editor.desktop",
        ));
        let _ = core.observe_focus(&id);
        let effects = dispatch(
            &mut core,
            DesktopKey::KeyM,
            KeyTransition::Press,
            Duration::from_secs(1),
        )
        .expect("mapped input");
        assert!(effects.navigation.contains(&AranduEvent::RiverChanged(RiverKind::Memory)));
    }

    #[test]
    fn pin_remains_until_explicit_escape() {
        let mut core = AranduCore::default();
        let _ = dispatch(
            &mut core,
            DesktopKey::Digit3,
            KeyTransition::Press,
            Duration::ZERO,
        );
        assert_eq!(core.confluence_state(), ConfluenceState::Pinned);
        let _ = core.handle_confluence_signal(
            arandu_confluence::ConfluenceSignal::Tick,
            Duration::from_secs(60),
        );
        assert_eq!(core.confluence_state(), ConfluenceState::Pinned);
        let _ = dispatch(
            &mut core,
            DesktopKey::Escape,
            KeyTransition::Press,
            Duration::from_secs(61),
        );
        assert_eq!(core.confluence_state(), ConfluenceState::Collapsed);
    }
}
