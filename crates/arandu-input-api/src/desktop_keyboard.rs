//! A1.3: portable keyboard-intent mapping for future native window adapters.
//!
//! No OS hooks are registered here. A future window backend supplies a
//! normalized key event; this layer maps it to an explicit ARANDU intent.

use arandu_confluence::ConfluenceSignal;
use arandu_model::RiverKind;
use arandu_navigation::NavigationIntent;

/// Physical input is normalized by the future platform adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopKey {
    Escape,
    Digit1,
    Digit2,
    Digit3,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    KeyM,
    KeyO,
    KeyF,
    Other,
}

/// Explicit ARANDU action, independent of X11, Wayland or toolkit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesktopAction {
    Confluence(ConfluenceSignal),
    Navigation(NavigationIntent),
}

/// Only press transitions produce actions; repeated keypresses and releases
/// must be handled deliberately by the native adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyTransition {
    Press,
    Release,
}

/// Converts one normalized physical event to at most one core action.
#[must_use]
pub fn map_key(key: DesktopKey, transition: KeyTransition) -> Option<DesktopAction> {
    if transition != KeyTransition::Press {
        return None;
    }
    match key {
        DesktopKey::Escape | DesktopKey::Digit1 => {
            Some(DesktopAction::Confluence(ConfluenceSignal::Collapse))
        }
        DesktopKey::Digit2 => Some(DesktopAction::Confluence(ConfluenceSignal::Expand)),
        DesktopKey::Digit3 => Some(DesktopAction::Confluence(ConfluenceSignal::TogglePin)),
        DesktopKey::ArrowLeft | DesktopKey::ArrowUp => {
            Some(DesktopAction::Navigation(NavigationIntent::PreviousNode))
        }
        DesktopKey::ArrowRight | DesktopKey::ArrowDown => {
            Some(DesktopAction::Navigation(NavigationIntent::NextNode))
        }
        DesktopKey::KeyM => Some(DesktopAction::Navigation(
            NavigationIntent::SelectRiver(RiverKind::Memory),
        )),
        DesktopKey::KeyO => Some(DesktopAction::Navigation(
            NavigationIntent::SelectRiver(RiverKind::Open),
        )),
        DesktopKey::KeyF => Some(DesktopAction::Navigation(
            NavigationIntent::SelectRiver(RiverKind::Frequent),
        )),
        DesktopKey::Other => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_always_requests_explicit_collapse() {
        assert_eq!(
            map_key(DesktopKey::Escape, KeyTransition::Press),
            Some(DesktopAction::Confluence(ConfluenceSignal::Collapse))
        );
    }

    #[test]
    fn released_and_unmapped_keys_are_ignored() {
        assert_eq!(map_key(DesktopKey::Digit2, KeyTransition::Release), None);
        assert_eq!(map_key(DesktopKey::Other, KeyTransition::Press), None);
    }

    #[test]
    fn navigation_uses_portable_core_intents() {
        assert_eq!(
            map_key(DesktopKey::KeyM, KeyTransition::Press),
            Some(DesktopAction::Navigation(NavigationIntent::SelectRiver(
                RiverKind::Memory
            )))
        );
        assert_eq!(
            map_key(DesktopKey::ArrowRight, KeyTransition::Press),
            Some(DesktopAction::Navigation(NavigationIntent::NextNode))
        );
    }

    #[test]
    fn pin_and_expand_have_distinct_intents() {
        assert_eq!(
            map_key(DesktopKey::Digit2, KeyTransition::Press),
            Some(DesktopAction::Confluence(ConfluenceSignal::Expand))
        );
        assert_eq!(
            map_key(DesktopKey::Digit3, KeyTransition::Press),
            Some(DesktopAction::Confluence(ConfluenceSignal::TogglePin))
        );
    }
}
