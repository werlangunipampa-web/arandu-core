use std::collections::VecDeque;
use std::fmt;
use std::time::{Duration, SystemTime};

pub const DEFAULT_RIVER_CAPACITY: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ApplicationId(String);

impl ApplicationId {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApplicationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiverKind {
    Open,
    Memory,
    Frequent,
}

impl RiverKind {
    pub const ALL: [Self; 3] = [Self::Open, Self::Memory, Self::Frequent];

    #[must_use]
    pub fn label_pt_br(self) -> &'static str {
        match self {
            Self::Open => "ABERTOS",
            Self::Memory => "MEMÓRIA",
            Self::Frequent => "FREQUENTES",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppCategory {
    Browser,
    Terminal,
    Files,
    Development,
    Document,
    Media,
    Communication,
    Utility,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccentFamily {
    Teal,
    Silver,
    Ochre,
    Olive,
    LightBlue,
    Violet,
    Amber,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisualIdentity {
    pub category: AppCategory,
    pub arandu_symbol: String,
    pub original_icon_reference: Option<String>,
    pub accent: AccentFamily,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationNode {
    pub id: ApplicationId,
    pub name: String,
    pub executable: Option<String>,
    pub platform_id: String,
    pub is_open: bool,
    pub is_focused: bool,
    pub last_used_at: Option<SystemTime>,
    pub usage_count: u64,
    pub accumulated_active: Duration,
    pub visual: VisualIdentity,
}

impl ApplicationNode {
    #[must_use]
    pub fn new(id: ApplicationId, name: impl Into<String>, platform_id: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            executable: None,
            platform_id: platform_id.into(),
            is_open: false,
            is_focused: false,
            last_used_at: None,
            usage_count: 0,
            accumulated_active: Duration::ZERO,
            visual: VisualIdentity {
                category: AppCategory::Other,
                arandu_symbol: "generic".to_string(),
                original_icon_reference: None,
                accent: AccentFamily::Neutral,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiverState {
    pub kind: RiverKind,
    pub nodes: Vec<ApplicationId>,
    pub selected_index: Option<usize>,
}

impl RiverState {
    #[must_use]
    pub fn new(kind: RiverKind, nodes: Vec<ApplicationId>) -> Self {
        let selected_index = (!nodes.is_empty()).then_some(0);
        Self {
            kind,
            nodes,
            selected_index,
        }
    }

    #[must_use]
    pub fn selected(&self) -> Option<&ApplicationId> {
        self.selected_index.and_then(|index| self.nodes.get(index))
    }

    pub fn select_next(&mut self) {
        if self.nodes.is_empty() {
            self.selected_index = None;
            return;
        }
        let next = self
            .selected_index
            .map_or(0, |index| (index + 1) % self.nodes.len());
        self.selected_index = Some(next);
    }

    pub fn select_previous(&mut self) {
        if self.nodes.is_empty() {
            self.selected_index = None;
            return;
        }
        let previous = self.selected_index.map_or(0, |index| {
            if index == 0 {
                self.nodes.len() - 1
            } else {
                index - 1
            }
        });
        self.selected_index = Some(previous);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confluence {
    pub rivers: [RiverKind; 3],
}

impl Default for Confluence {
    fn default() -> Self {
        Self {
            rivers: RiverKind::ALL,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecentMemory {
    capacity: usize,
    entries: VecDeque<ApplicationId>,
}

impl RecentMemory {
    /// Creates a recent-memory buffer with a fixed positive capacity.
    ///
    /// # Panics
    ///
    /// Panics when `capacity` is zero.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "recent memory capacity must be positive");
        Self {
            capacity,
            entries: VecDeque::with_capacity(capacity),
        }
    }

    #[must_use]
    pub fn default_five() -> Self {
        Self::new(DEFAULT_RIVER_CAPACITY)
    }

    pub fn observe(&mut self, id: ApplicationId) {
        if let Some(index) = self.entries.iter().position(|existing| existing == &id) {
            self.entries.remove(index);
        }
        self.entries.push_front(id);
        while self.entries.len() > self.capacity {
            self.entries.pop_back();
        }
    }

    #[must_use]
    pub fn as_vec(&self) -> Vec<ApplicationId> {
        self.entries.iter().cloned().collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_memory_keeps_unique_mru_order() {
        let mut memory = RecentMemory::new(3);
        memory.observe(ApplicationId::new("a"));
        memory.observe(ApplicationId::new("b"));
        memory.observe(ApplicationId::new("c"));
        memory.observe(ApplicationId::new("b"));
        memory.observe(ApplicationId::new("d"));

        let ids: Vec<_> = memory
            .as_vec()
            .into_iter()
            .map(|id| id.to_string())
            .collect();
        assert_eq!(ids, vec!["d", "b", "c"]);
    }

    #[test]
    fn river_selection_wraps() {
        let mut river = RiverState::new(
            RiverKind::Memory,
            vec![ApplicationId::new("a"), ApplicationId::new("b")],
        );
        assert_eq!(river.selected().map(ApplicationId::as_str), Some("a"));
        river.select_next();
        assert_eq!(river.selected().map(ApplicationId::as_str), Some("b"));
        river.select_next();
        assert_eq!(river.selected().map(ApplicationId::as_str), Some("a"));
        river.select_previous();
        assert_eq!(river.selected().map(ApplicationId::as_str), Some("b"));
    }
}
