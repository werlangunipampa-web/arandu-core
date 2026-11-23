//! A8.3 — Espaços Vivos: planos de ações sobre janelas, sem executar ações.
//!
//! Um ID de aplicativo não identifica uma janela. O adaptador GNOME é responsável
//! por emitir identificadores opacos efêmeros, validar o gesto humano e resolver
//! novamente as janelas imediatamente antes de aplicar qualquer operação.
//! Este módulo não importa GNOME, não lê títulos e não fecha janelas.

use std::collections::HashSet;

use arandu_model::ApplicationId;

const MAX_WINDOWS: usize = 128;
const MAX_TOKEN_BYTES: usize = 64;

/// Referência opaca e local à sessão. Nunca é ID do aplicativo ou título.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WindowRef {
    token: String,
    generation: u64,
}

impl WindowRef {
    /// Cria referência somente com token opaco seguro e geração positiva.
    ///
    /// # Errors
    ///
    /// Rejeita token vazio, não ASCII, excessivo e geração zero.
    pub fn new(token: impl Into<String>, generation: u64) -> Result<Self, PlanError> {
        let token = token.into();
        if generation == 0
            || token.is_empty()
            || token.len() > MAX_TOKEN_BYTES
            || !token
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
        {
            return Err(PlanError::InvalidReference);
        }
        Ok(Self { token, generation })
    }

    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}

/// Estado mínimo de uma janela; nomes de arquivos e títulos são excluídos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowObservation {
    pub reference: WindowRef,
    pub application: ApplicationId,
    pub monitor: u32,
    pub workspace: u32,
    pub minimized: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub can_minimize: bool,
    pub can_maximize: bool,
    pub can_close: bool,
    pub can_tile: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSnapshot {
    generation: u64,
    windows: Vec<WindowObservation>,
}

impl WindowSnapshot {
    /// Valida um conjunto de referências locais de janelas observadas.
    ///
    /// # Errors
    ///
    /// Rejeita gerações inválidas, referências duplicadas e excesso de janelas.
    pub fn new(generation: u64, windows: Vec<WindowObservation>) -> Result<Self, PlanError> {
        if generation == 0 || windows.len() > MAX_WINDOWS {
            return Err(PlanError::InvalidSnapshot);
        }
        let mut seen = HashSet::new();
        for window in &windows {
            if window.reference.generation != generation
                || window.application.as_str().is_empty()
                || !seen.insert(&window.reference)
            {
                return Err(PlanError::InvalidSnapshot);
            }
        }
        Ok(Self {
            generation,
            windows,
        })
    }

    #[must_use]
    pub fn windows(&self) -> &[WindowObservation] {
        &self.windows
    }

    fn locate(&self, reference: &WindowRef) -> Result<&WindowObservation, PlanError> {
        if reference.generation != self.generation {
            return Err(PlanError::StaleReference);
        }
        self.windows
            .iter()
            .find(|window| window.reference == *reference)
            .ok_or(PlanError::WindowMissing)
    }
}

/// Apenas origens deliberadas; o adaptador precisa provar que o gesto ocorreu.
/// O enum não é, isoladamente, um mecanismo de autenticação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmedGesture {
    Click,
    Keyboard,
    Assistive,
}

/// Intenções são universais e não representam chamadas ao GNOME.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowIntent {
    RecolherAoRio,
    TrazerAoFoco,
    Imersao,
    Confluencia { partner: WindowRef },
    Encerrar,
}

/// Operações declarativas. Nenhuma é executada pelo núcleo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEffect {
    Minimize(WindowRef),
    Unminimize(WindowRef),
    Activate(WindowRef),
    Maximize(WindowRef),
    TileSideBySide {
        primary: WindowRef,
        secondary: WindowRef,
    },
    RequestClose(WindowRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowPlan {
    pub steps: Vec<WindowEffect>,
    /// Fechar deve ser confirmado na interface; a aplicação controla salvamento.
    pub needs_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanOutcome {
    NoChange,
    Ready(WindowPlan),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanError {
    InvalidReference,
    InvalidSnapshot,
    StaleReference,
    WindowMissing,
    OperationNotSupported,
    IncompatibleSurfaces,
}

/// Planeja efeitos após gesto humano confirmado, mas nunca toca o compositor.
///
/// A validade do plano depende da observação atual. O adaptador deve conferir
/// novamente referências, estado e capacidades no momento da execução.
///
/// # Errors
///
/// Rejeita janelas ausentes, obsoletas, sem capacidades ou com espaços
/// incompatíveis. Não substitui confirmação de desligamento ou salvamento.
pub fn plan_window_action(
    snapshot: &WindowSnapshot,
    target: &WindowRef,
    intent: &WindowIntent,
    _gesture: ConfirmedGesture,
) -> Result<PlanOutcome, PlanError> {
    let current = snapshot.locate(target)?;

    let plan = match intent {
        WindowIntent::RecolherAoRio => {
            if !current.can_minimize {
                return Err(PlanError::OperationNotSupported);
            }
            if current.minimized {
                return Ok(PlanOutcome::NoChange);
            }
            WindowPlan {
                steps: vec![WindowEffect::Minimize(target.clone())],
                needs_confirmation: false,
            }
        }
        WindowIntent::TrazerAoFoco => {
            let mut steps = Vec::with_capacity(2);
            if current.minimized {
                steps.push(WindowEffect::Unminimize(target.clone()));
            }
            steps.push(WindowEffect::Activate(target.clone()));
            WindowPlan {
                steps,
                needs_confirmation: false,
            }
        }
        WindowIntent::Imersao => {
            if !current.can_maximize || current.fullscreen {
                return Err(PlanError::OperationNotSupported);
            }
            if current.maximized && !current.minimized {
                return Ok(PlanOutcome::NoChange);
            }
            let mut steps = Vec::with_capacity(2);
            if current.minimized {
                steps.push(WindowEffect::Unminimize(target.clone()));
            }
            steps.push(WindowEffect::Maximize(target.clone()));
            WindowPlan {
                steps,
                needs_confirmation: false,
            }
        }
        WindowIntent::Confluencia { partner } => {
            let companion = snapshot.locate(partner)?;
            if partner == target
                || current.monitor != companion.monitor
                || current.workspace != companion.workspace
                || current.fullscreen
                || companion.fullscreen
            {
                return Err(PlanError::IncompatibleSurfaces);
            }
            if !current.can_tile || !companion.can_tile {
                return Err(PlanError::OperationNotSupported);
            }
            let mut steps = Vec::with_capacity(3);
            if current.minimized {
                steps.push(WindowEffect::Unminimize(target.clone()));
            }
            if companion.minimized {
                steps.push(WindowEffect::Unminimize(partner.clone()));
            }
            steps.push(WindowEffect::TileSideBySide {
                primary: target.clone(),
                secondary: partner.clone(),
            });
            WindowPlan {
                steps,
                needs_confirmation: false,
            }
        }
        WindowIntent::Encerrar => {
            if !current.can_close {
                return Err(PlanError::OperationNotSupported);
            }
            WindowPlan {
                steps: vec![WindowEffect::RequestClose(target.clone())],
                needs_confirmation: true,
            }
        }
    };
    Ok(PlanOutcome::Ready(plan))
}

/// A metáfora de Rio não implica trilhas permanentes no desktop.
/// Opy pode oferecer os três Rios sem poluir o espaço de trabalho.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RiverPresentation {
    #[default]
    Discreet,
    Requested,
}

impl RiverPresentation {
    #[must_use]
    pub fn show_trails(self) -> bool {
        matches!(self, Self::Requested)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(token: &str, generation: u64) -> WindowRef {
        WindowRef::new(token, generation).unwrap()
    }

    fn window(token: &str, app: &str, monitor: u32, workspace: u32) -> WindowObservation {
        WindowObservation {
            reference: reference(token, 42),
            application: ApplicationId::new(app),
            monitor,
            workspace,
            minimized: false,
            maximized: false,
            fullscreen: false,
            can_minimize: true,
            can_maximize: true,
            can_close: true,
            can_tile: true,
        }
    }

    fn snapshot() -> WindowSnapshot {
        WindowSnapshot::new(
            42,
            vec![
                window("editor1", "org.editor.desktop", 0, 0),
                window("editor2", "org.editor.desktop", 0, 0),
                window("browser", "org.browser.desktop", 1, 1),
            ],
        )
        .unwrap()
    }

    fn decide(
        source: &WindowSnapshot,
        target: &str,
        intent: WindowIntent,
    ) -> Result<PlanOutcome, PlanError> {
        plan_window_action(
            source,
            &reference(target, 42),
            &intent,
            ConfirmedGesture::Keyboard,
        )
    }

    #[test]
    fn two_windows_from_same_app_stay_distinct() {
        let source = snapshot();
        assert_eq!(source.windows()[0].application, source.windows()[1].application);
        assert_ne!(source.windows()[0].reference, source.windows()[1].reference);
    }

    #[test]
    fn rejects_invalid_and_duplicate_references() {
        assert_eq!(WindowRef::new("../file", 42), Err(PlanError::InvalidReference));
        assert_eq!(WindowRef::new("valid", 0), Err(PlanError::InvalidReference));
        assert_eq!(WindowRef::new("á", 1), Err(PlanError::InvalidReference));
        assert_eq!(
            WindowSnapshot::new(42, vec![window("same", "a", 0, 0), window("same", "b", 0, 0)]),
            Err(PlanError::InvalidSnapshot)
        );
    }

    #[test]
    fn rejects_stale_and_missing_windows() {
        let source = snapshot();
        assert_eq!(
            plan_window_action(
                &source,
                &reference("editor1", 41),
                &WindowIntent::RecolherAoRio,
                ConfirmedGesture::Click,
            ),
            Err(PlanError::StaleReference)
        );
        assert_eq!(
            decide(&source, "unknown", WindowIntent::Encerrar),
            Err(PlanError::WindowMissing)
        );
    }

    #[test]
    fn recolher_preserves_window_and_does_not_close() {
        let source = snapshot();
        assert_eq!(
            decide(&source, "editor1", WindowIntent::RecolherAoRio).unwrap(),
            PlanOutcome::Ready(WindowPlan {
                steps: vec![WindowEffect::Minimize(reference("editor1", 42))],
                needs_confirmation: false,
            })
        );
        assert_eq!(source.windows().len(), 3);
    }

    #[test]
    fn recolher_twice_is_no_change() {
        let mut source = snapshot();
        source.windows[0].minimized = true;
        assert_eq!(
            decide(&source, "editor1", WindowIntent::RecolherAoRio).unwrap(),
            PlanOutcome::NoChange
        );
    }

    #[test]
    fn retomar_restores_then_activates_exact_window() {
        let mut source = snapshot();
        source.windows[1].minimized = true;
        assert_eq!(
            decide(&source, "editor2", WindowIntent::TrazerAoFoco).unwrap(),
            PlanOutcome::Ready(WindowPlan {
                steps: vec![
                    WindowEffect::Unminimize(reference("editor2", 42)),
                    WindowEffect::Activate(reference("editor2", 42))
                ],
                needs_confirmation: false,
            })
        );
    }

    #[test]
    fn imersao_is_maximize_not_exclusive_fullscreen() {
        let source = snapshot();
        let result = decide(&source, "editor1", WindowIntent::Imersao).unwrap();
        assert_eq!(
            result,
            PlanOutcome::Ready(WindowPlan {
                steps: vec![WindowEffect::Maximize(reference("editor1", 42))],
                needs_confirmation: false,
            })
        );
    }

    #[test]
    fn confluencia_requires_two_compatible_windows() {
        let source = snapshot();
        let decision = decide(
            &source,
            "editor1",
            WindowIntent::Confluencia {
                partner: reference("editor2", 42),
            },
        )
        .unwrap();
        assert!(matches!(
            decision,
            PlanOutcome::Ready(WindowPlan {
                steps,
                needs_confirmation: false
            }) if steps == vec![WindowEffect::TileSideBySide {
                primary: reference("editor1", 42),
                secondary: reference("editor2", 42)
            }]
        ));

        assert_eq!(
            decide(
                &source,
                "editor1",
                WindowIntent::Confluencia {
                    partner: reference("browser", 42)
                }
            ),
            Err(PlanError::IncompatibleSurfaces)
        );
        assert_eq!(
            decide(
                &source,
                "editor1",
                WindowIntent::Confluencia {
                    partner: reference("editor1", 42)
                }
            ),
            Err(PlanError::IncompatibleSurfaces)
        );
    }

    #[test]
    fn close_never_runs_and_requires_confirmation() {
        let source = snapshot();
        assert_eq!(
            decide(&source, "editor1", WindowIntent::Encerrar).unwrap(),
            PlanOutcome::Ready(WindowPlan {
                steps: vec![WindowEffect::RequestClose(reference("editor1", 42))],
                needs_confirmation: true,
            })
        );
        assert_eq!(source.windows().len(), 3);
    }

    #[test]
    fn denies_unsupported_actions() {
        let mut source = snapshot();
        source.windows[0].can_close = false;
        source.windows[0].can_maximize = false;
        assert_eq!(
            decide(&source, "editor1", WindowIntent::Encerrar),
            Err(PlanError::OperationNotSupported)
        );
        assert_eq!(
            decide(&source, "editor1", WindowIntent::Imersao),
            Err(PlanError::OperationNotSupported)
        );
    }

    #[test]
    fn river_presentation_defaults_to_no_trails() {
        assert!(!RiverPresentation::default().show_trails());
        assert!(RiverPresentation::Requested.show_trails());
    }
}
