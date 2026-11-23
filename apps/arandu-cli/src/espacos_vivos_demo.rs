//! Demonstração offline da semântica A8.3, sem consultar ou modificar GNOME.

use arandu_core::espacos_vivos::{
    ConfirmedGesture, PlanOutcome, WindowEffect, WindowIntent, WindowObservation, WindowRef,
    WindowSnapshot, plan_window_action,
};
use arandu_model::ApplicationId;
use std::fmt::Write;

fn window(token: &str, app: &str) -> WindowObservation {
    WindowObservation {
        reference: WindowRef::new(token, 1).expect("ID sintético válido"),
        application: ApplicationId::new(app),
        monitor: 0,
        workspace: 0,
        minimized: false,
        maximized: false,
        fullscreen: false,
        can_minimize: true,
        can_maximize: true,
        can_close: true,
        can_tile: true,
    }
}

#[must_use]
pub fn demo() -> String {
    let snapshot = WindowSnapshot::new(
        1,
        vec![
            window("texto", "org.editor.desktop"),
            window("artigo", "org.browser.desktop"),
        ],
    )
    .expect("snapshot sintético válido");
    let target = WindowRef::new("artigo", 1).expect("ID sintético válido");
    let mut output = String::from("ARANDU A8.3 — Espaços Vivos (simulação offline)\n");
    for (name, intent) in [
        ("Recolher ao Rio", WindowIntent::RecolherAoRio),
        ("Trazer ao foco", WindowIntent::TrazerAoFoco),
        ("Imersão", WindowIntent::Imersao),
        (
            "Confluência",
            WindowIntent::Confluencia {
                partner: WindowRef::new("texto", 1).expect("ID sintético válido"),
            },
        ),
        ("Encerrar", WindowIntent::Encerrar),
    ] {
        let planned = plan_window_action(&snapshot, &target, &intent, ConfirmedGesture::Click)
            .expect("plano sintético válido");
        match planned {
            PlanOutcome::NoChange => {
                let _ = writeln!(output, "{name}: nenhuma alteração necessária");
            }
            PlanOutcome::Ready(plan) => {
                let operations: Vec<_> = plan.steps.iter().map(effect_label).collect();
                let _ = writeln!(
                    output,
                    "{name}: {} | confirmação: {}",
                    operations.join(" → "),
                    if plan.needs_confirmation {
                        "obrigatória"
                    } else {
                        "não exigida pelo plano"
                    }
                );
            }
        }
    }
    output.push_str("Nenhuma janela real foi modificada.\n");
    output
}

fn effect_label(effect: &WindowEffect) -> &'static str {
    match effect {
        WindowEffect::Minimize(_) => "minimizar",
        WindowEffect::Unminimize(_) => "restaurar",
        WindowEffect::Activate(_) => "ativar",
        WindowEffect::Maximize(_) => "maximizar (sem fullscreen)",
        WindowEffect::TileSideBySide { .. } => "organizar lado a lado",
        WindowEffect::RequestClose(_) => "solicitar fechamento",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_explicitly_reports_no_real_actions() {
        let output = demo();
        assert!(output.contains("Recolher ao Rio"));
        assert!(output.contains("Confluência"));
        assert!(output.contains("confirmação: obrigatória"));
        assert!(output.contains("Nenhuma janela real foi modificada"));
    }
}
