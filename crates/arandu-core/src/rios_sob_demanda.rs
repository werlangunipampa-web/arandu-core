//! ARANDU A8.8 — Três Rios sob demanda e estados sem dependência GNOME.
//!
//! Não desenha faixas permanentes, não armazena títulos de janelas e não
//! decide por conta própria quando o usuário deve enxergar os Rios.

use std::collections::HashSet;
use arandu_model::ApplicationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum River {
    Memory,
    Open,
    Frequent,
}

impl River {
    pub const ALL: [Self; 3] = [Self::Memory, Self::Open, Self::Frequent];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityState {
    Visible,
    Recolhida,
    Encerrada,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiverEntry {
    pub app: ApplicationId,
    pub window_token: Option<String>,
    pub state: ActivityState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiverColumn {
    pub river: River,
    pub entries: Vec<RiverEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceState {
    Hidden,
    SeedOnly,
    ThreeColumns { focus: Option<River> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiverProjection {
    pub state: SurfaceState,
    pub columns: [RiverColumn; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectionError {
    DuplicateWindow,
    InvalidWindow,
    InvalidHistoricEntry,
    TooManyItems,
}

impl RiverProjection {
    /// Contrato de visualização: Memória inclui encerrados, Abertos janelas reais,
    /// Frequentes apenas IDs que tenham uso observado.
    ///
    /// # Errors
    ///
    /// Rejeita janela duplicada, ID malformado ou itens em excesso.
    pub fn build(
        state: SurfaceState,
        memory: Vec<ApplicationId>,
        open: Vec<RiverEntry>,
        frequent: Vec<ApplicationId>,
    ) -> Result<Self, ProjectionError> {
        if memory.len() > 128 || open.len() > 128 || frequent.len() > 128 {
            return Err(ProjectionError::TooManyItems);
        }
        let mut seen = HashSet::new();
        for entry in &open {
            let Some(token) = &entry.window_token else {
                return Err(ProjectionError::InvalidWindow);
            };
            if token.is_empty() || token.len() > 64 ||
                !token.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-') ||
                entry.state == ActivityState::Encerrada
            {
                return Err(ProjectionError::InvalidWindow);
            }
            if !seen.insert(token) {
                return Err(ProjectionError::DuplicateWindow);
            }
        }
        if memory.iter().chain(frequent.iter()).any(|id| id.as_str().is_empty()) {
            return Err(ProjectionError::InvalidHistoricEntry);
        }
        let history = memory.into_iter().map(|app| RiverEntry {
            app, window_token: None, state: ActivityState::Encerrada,
        }).collect();
        let frequent = frequent.into_iter().map(|app| RiverEntry {
            app, window_token: None, state: ActivityState::Encerrada,
        }).collect();
        Ok(Self { state, columns: [
            RiverColumn { river: River::Memory, entries: history },
            RiverColumn { river: River::Open, entries: open },
            RiverColumn { river: River::Frequent, entries: frequent },
        ]})
    }

    #[must_use]
    pub fn visible_columns(&self) -> Option<&[RiverColumn; 3]> {
        match self.state {
            SurfaceState::ThreeColumns { .. } => Some(&self.columns),
            SurfaceState::Hidden | SurfaceState::SeedOnly => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ApplicationId {
        ApplicationId::new(value)
    }

    fn example(state: SurfaceState) -> RiverProjection {
        RiverProjection::build(state, vec![id("editor.desktop")], vec![
            RiverEntry {
                app: id("browser.desktop"),
                window_token: Some("w11".into()),
                state: ActivityState::Visible,
            },
            RiverEntry {
                app: id("browser.desktop"),
                window_token: Some("w12".into()),
                state: ActivityState::Recolhida,
            }
        ], vec![id("browser.desktop")]).expect("projection valid")
    }

    #[test]
    fn surface_is_invisible_by_default_intent() {
        let projection = example(SurfaceState::Hidden);
        assert!(projection.visible_columns().is_none());
        let seed = example(SurfaceState::SeedOnly);
        assert!(seed.visible_columns().is_none());
    }

    #[test]
    fn all_three_rivers_appear_only_when_requested() {
        let projection = example(SurfaceState::ThreeColumns { focus: Some(River::Open) });
        let cols = projection.visible_columns().expect("visible");
        assert_eq!(cols.len(), 3);
        assert_eq!(cols[0].river, River::Memory);
        assert_eq!(cols[1].river, River::Open);
        assert_eq!(cols[2].river, River::Frequent);
    }

    #[test]
    fn two_windows_same_app_are_distinct_and_minimized_remains_open() {
        let projection = example(SurfaceState::ThreeColumns { focus: None });
        let entries = &projection.columns[1].entries;
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].app, entries[1].app);
        assert_ne!(entries[0].window_token, entries[1].window_token);
        assert_eq!(entries[1].state, ActivityState::Recolhida);
    }

    #[test]
    fn memory_is_not_a_live_window() {
        let projection = example(SurfaceState::ThreeColumns { focus: None });
        assert_eq!(projection.columns[0].entries[0].state, ActivityState::Encerrada);
        assert!(projection.columns[0].entries[0].window_token.is_none());
    }

    #[test]
    fn invalid_open_entries_are_rejected() {
        let invalid = RiverProjection::build(
            SurfaceState::SeedOnly,
            Vec::new(),
            vec![RiverEntry {
                app: id("editor.desktop"), window_token: None, state: ActivityState::Visible,
            }],
            Vec::new(),
        );
        assert_eq!(invalid, Err(ProjectionError::InvalidWindow));
    }

    #[test]
    fn duplicate_window_rejected_even_if_app_id_different() {
        let entries = vec![
            RiverEntry {app:id("a"),window_token:Some("w1".into()),state:ActivityState::Visible},
            RiverEntry {app:id("b"),window_token:Some("w1".into()),state:ActivityState::Recolhida},
        ];
        assert_eq!(RiverProjection::build(SurfaceState::Hidden,vec![],entries,vec![]),
            Err(ProjectionError::DuplicateWindow));
    }
}
