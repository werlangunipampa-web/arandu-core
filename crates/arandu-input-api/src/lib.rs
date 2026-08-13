use arandu_navigation::NavigationIntent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputModality {
    Keyboard,
    Mouse,
    Touchpad,
    Voice,
    TonalSound,
    HeadMovement,
    AssistiveSwitch,
    ExternalDevice,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    Named(String),
    Button(u16),
    KeyChord(String),
    VoiceCommand(String),
    TonalBand(TonalBand),
    HeadGesture(HeadGesture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TonalBand {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadGesture {
    Left,
    Right,
    Up,
    Down,
    Diagonal,
    Hold,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputSignal {
    pub modality: InputModality,
    pub action: InputAction,
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputMapping {
    pub modality: InputModality,
    pub action: InputAction,
    pub intent: NavigationIntent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionProfile {
    pub name: String,
    pub mappings: Vec<InputMapping>,
}

impl InteractionProfile {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mappings: Vec::new(),
        }
    }

    pub fn map(&mut self, mapping: InputMapping) {
        self.mappings.push(mapping);
    }

    #[must_use]
    pub fn resolve(
        &self,
        signal: &InputSignal,
        minimum_confidence: f32,
    ) -> Option<NavigationIntent> {
        if signal
            .confidence
            .is_some_and(|confidence| confidence < minimum_confidence)
        {
            return None;
        }
        self.mappings
            .iter()
            .find(|mapping| mapping.modality == signal.modality && mapping.action == signal.action)
            .map(|mapping| mapping.intent)
    }
}

pub trait IntentSource {
    fn poll_intents(&mut self) -> Vec<NavigationIntent>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccessibilityPolicy {
    pub minimum_confidence: f32,
    pub dwell_ms: u64,
    pub debounce_ms: u64,
    pub require_confirmation_for_launch: bool,
}

impl Default for AccessibilityPolicy {
    fn default() -> Self {
        Self {
            minimum_confidence: 0.70,
            dwell_ms: 800,
            debounce_ms: 250,
            require_confirmation_for_launch: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use arandu_model::RiverKind;
    use arandu_navigation::NavigationIntent;

    use super::*;

    #[test]
    fn voice_and_tone_can_map_to_same_intent() {
        let mut profile = InteractionProfile::new("multimodal");
        profile.map(InputMapping {
            modality: InputModality::Voice,
            action: InputAction::VoiceCommand("MEMÓRIA".into()),
            intent: NavigationIntent::SelectRiver(RiverKind::Memory),
        });
        profile.map(InputMapping {
            modality: InputModality::TonalSound,
            action: InputAction::TonalBand(TonalBand::Medium),
            intent: NavigationIntent::SelectRiver(RiverKind::Memory),
        });

        let voice = InputSignal {
            modality: InputModality::Voice,
            action: InputAction::VoiceCommand("MEMÓRIA".into()),
            confidence: Some(0.95),
        };
        let tone = InputSignal {
            modality: InputModality::TonalSound,
            action: InputAction::TonalBand(TonalBand::Medium),
            confidence: Some(0.90),
        };

        assert_eq!(profile.resolve(&voice, 0.70), profile.resolve(&tone, 0.70));
    }
}
