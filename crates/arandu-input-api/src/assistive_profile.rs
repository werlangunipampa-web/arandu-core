//! A2: optional, device-independent intent mappings for assistive input.
//!
//! No sensors are started here. Physical head trackers, tonal controllers,
//! adaptive mice, and switches must be configured and explicitly enabled by
//! the user in future platform adapters. The GNOME renderer never consumes
//! raw audio/video; it receives only normalized navigation intentions.

use arandu_model::RiverKind;
use arandu_navigation::NavigationIntent;

use crate::{
    HeadGesture, InputAction, InputMapping, InputModality, InteractionProfile, TonalBand,
};

/// Baseline assistive mapping, independent of GNOME, Sway and input hardware.
///
/// An adapter must implement calibration, consent, dwell/debounce and explicit
/// confirmation before taking an action that launches an application.
/// This function only maps sanitized, already-recognized signals to intents.
#[must_use]
pub fn baseline_profile() -> InteractionProfile {
    let mut profile = InteractionProfile::new("arandu-multimodal-baseline");
    let mappings = [
        (
            InputModality::HeadMovement,
            InputAction::HeadGesture(HeadGesture::Right),
            NavigationIntent::NextNode,
        ),
        (
            InputModality::HeadMovement,
            InputAction::HeadGesture(HeadGesture::Left),
            NavigationIntent::PreviousNode,
        ),
        (
            InputModality::TonalSound,
            InputAction::TonalBand(TonalBand::High),
            NavigationIntent::NextNode,
        ),
        (
            InputModality::TonalSound,
            InputAction::TonalBand(TonalBand::Low),
            NavigationIntent::PreviousNode,
        ),
        (
            InputModality::AssistiveSwitch,
            InputAction::Button(1),
            NavigationIntent::NextNode,
        ),
        (
            InputModality::AssistiveSwitch,
            InputAction::Button(2),
            NavigationIntent::PreviousNode,
        ),
        (
            InputModality::ExternalDevice,
            InputAction::Button(1),
            NavigationIntent::NextNode,
        ),
        (
            InputModality::Voice,
            InputAction::VoiceCommand("MEMÓRIA".to_owned()),
            NavigationIntent::SelectRiver(RiverKind::Memory),
        ),
        (
            InputModality::Voice,
            InputAction::VoiceCommand("ABERTOS".to_owned()),
            NavigationIntent::SelectRiver(RiverKind::Open),
        ),
        (
            InputModality::Voice,
            InputAction::VoiceCommand("FREQUENTES".to_owned()),
            NavigationIntent::SelectRiver(RiverKind::Frequent),
        ),
    ];
    for (modality, action, intent) in mappings {
        profile.map(InputMapping {
            modality,
            action,
            intent,
        });
    }
    profile
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InputSignal;

    #[test]
    fn head_sound_and_switch_request_the_same_next_node_intent() {
        let profile = baseline_profile();
        let choices = [
            (
                InputModality::HeadMovement,
                InputAction::HeadGesture(HeadGesture::Right),
            ),
            (
                InputModality::TonalSound,
                InputAction::TonalBand(TonalBand::High),
            ),
            (InputModality::AssistiveSwitch, InputAction::Button(1)),
            (InputModality::ExternalDevice, InputAction::Button(1)),
        ];
        for (modality, action) in choices {
            let signal = InputSignal {
                modality,
                action,
                confidence: Some(0.95),
            };
            assert_eq!(
                profile.resolve(&signal, 0.70),
                Some(NavigationIntent::NextNode)
            );
        }
    }

    #[test]
    fn uncertain_head_gesture_does_not_navigate() {
        let profile = baseline_profile();
        let signal = InputSignal {
            modality: InputModality::HeadMovement,
            action: InputAction::HeadGesture(HeadGesture::Right),
            confidence: Some(0.45),
        };
        assert_eq!(profile.resolve(&signal, 0.70), None);
    }

    #[test]
    fn baseline_never_maps_an_unconfirmed_signal_to_app_activation() {
        let profile = baseline_profile();
        assert!(profile.mappings.iter().all(|mapping| {
            mapping.intent != NavigationIntent::ActivateNode
        }));
    }
}
