use std::time::Duration;

use arandu_confluence::ConfluenceSignal;
use arandu_core::AranduCore;
use arandu_model::{ApplicationId, ApplicationNode, RiverKind};
use arandu_navigation::NavigationIntent;

fn main() {
    let mut core = AranduCore::default();

    for (id, name) in [
        ("firefox", "Firefox"),
        ("terminal", "Terminal"),
        ("files", "Arquivos"),
        ("editor", "Editor"),
        ("music", "Música"),
        ("calculator", "Calculadora"),
    ] {
        core.register_application(ApplicationNode::new(
            ApplicationId::new(id),
            name,
            format!("{id}.desktop"),
        ));
    }

    for id in [
        "firefox",
        "terminal",
        "files",
        "editor",
        "music",
        "calculator",
    ] {
        let _ = core.observe_focus(&ApplicationId::new(id));
    }
    let _ = core.observe_launch(&ApplicationId::new("firefox"));
    let _ = core.observe_launch(&ApplicationId::new("terminal"));
    let _ = core.observe_active_duration(&ApplicationId::new("firefox"), Duration::from_hours(1));
    let _ = core.observe_active_duration(&ApplicationId::new("terminal"), Duration::from_mins(20));

    println!("ARANDU CORE 0.2 — Semente da Confluência + Três Rios\n");
    for kind in RiverKind::ALL {
        let names: Vec<_> = core
            .river(kind)
            .nodes
            .iter()
            .map(|id| {
                core.application(id)
                    .map_or(id.as_str(), |app| app.name.as_str())
            })
            .collect();
        println!("{}: {}", kind.label_pt_br(), names.join(" → "));
    }

    println!("\nIntenção: selecionar MEMÓRIA");
    for event in core.handle_intent(NavigationIntent::SelectRiver(RiverKind::Memory)) {
        println!("  {event:?}");
    }

    println!("Intenção: próximo nó");
    for event in core.handle_intent(NavigationIntent::NextNode) {
        println!("  {event:?}");
    }

    println!("\nSemente da Confluência:");
    confluence_demo(core);
}

// Demonstração determinística da Semente da Confluência, sem depender do SO.
fn confluence_demo(mut core: AranduCore) {
    for (label, signal, at) in [
        (
            "AWARE",
            ConfluenceSignal::Proximity(0.40),
            Duration::from_millis(100),
        ),
        (
            "NEAR",
            ConfluenceSignal::Proximity(0.75),
            Duration::from_millis(200),
        ),
        (
            "EXPANDED",
            ConfluenceSignal::HoverEntered,
            Duration::from_millis(300),
        ),
    ] {
        let events = core.handle_confluence_signal(signal, at);
        println!(
            "Semente {label}: {:?} / eventos={events:?}",
            core.confluence_state()
        );
    }
}
