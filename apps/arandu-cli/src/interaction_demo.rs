//! A1.2 — deterministic interaction loop in Rust (stdin/stdout only).
//!
//! Prototype of the event contract, not a Wayland compositor, and it does
//! not read OS window state, user profiles or any live Seed data.

use arandu_confluence::ConfluenceSignal;
use arandu_core::AranduCore;
use arandu_geometry::{Point, Viewport};
use arandu_model::{ApplicationId, ApplicationNode, RiverKind};
use arandu_navigation::NavigationIntent;
use std::io::{self, BufRead, Write};
use std::time::Duration;

fn synthetic_core() -> AranduCore {
    let mut core = AranduCore::new(5);
    for (id, name) in [
        ("browser", "Navegador"),
        ("terminal", "Terminal"),
        ("editor", "Editor"),
    ] {
        core.register_application(ApplicationNode::new(
            ApplicationId::new(id),
            name,
            format!("{id}.desktop"),
        ));
        let key = ApplicationId::new(id);
        let _ = core.observe_launch(&key);
        let _ = core.observe_focus(&key);
    }
    core
}

fn apply(core: &mut AranduCore, command: &str, at: Duration) -> bool {
    match command {
        "1" | "collapse" | "esc" => {
            let _ = core.handle_confluence_signal(ConfluenceSignal::Collapse, at);
        }
        "2" | "expand" => {
            let _ = core.handle_confluence_signal(ConfluenceSignal::Expand, at);
        }
        "3" | "pin" => {
            let _ = core.handle_confluence_signal(ConfluenceSignal::TogglePin, at);
        }
        "memory" => {
            let _ = core.handle_intent(NavigationIntent::SelectRiver(RiverKind::Memory));
            let _ = core.handle_confluence_signal(ConfluenceSignal::Activity, at);
        }
        "open" => {
            let _ = core.handle_intent(NavigationIntent::SelectRiver(RiverKind::Open));
            let _ = core.handle_confluence_signal(ConfluenceSignal::Activity, at);
        }
        "frequent" => {
            let _ = core.handle_intent(NavigationIntent::SelectRiver(RiverKind::Frequent));
            let _ = core.handle_confluence_signal(ConfluenceSignal::Activity, at);
        }
        "next" => {
            let _ = core.handle_intent(NavigationIntent::NextNode);
            let _ = core.handle_confluence_signal(ConfluenceSignal::Activity, at);
        }
        "previous" => {
            let _ = core.handle_intent(NavigationIntent::PreviousNode);
            let _ = core.handle_confluence_signal(ConfluenceSignal::Activity, at);
        }
        "tick" => {
            let _ = core.handle_confluence_signal(ConfluenceSignal::Tick, at);
        }
        _ => return false,
    }
    true
}

/// Processes command lines in Rust, without external side effects.
///
/// # Errors
///
/// Returns an I/O error if input cannot be read or output cannot be written.
pub fn run<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut core = synthetic_core();
    writeln!(
        output,
        "ARANDU A1.2 | 1/collapse | 2/expand | 3/pin | memory/open/frequent | next/previous | tick | quit"
    )?;
    let mut elapsed = Duration::ZERO;
    for line in input.lines() {
        let line = line?;
        let command = line.trim();
        if command == "quit" {
            break;
        }
        elapsed += Duration::from_secs(1);
        if !apply(&mut core, command, elapsed) {
            writeln!(output, "Comando desconhecido: {command}")?;
            continue;
        }
        let scene = core.render_scene(Viewport::new(1100.0, 720.0), Point::new(550.0, 360.0));
        writeln!(
            output,
            "state={:?} reveal={:.2} nodes={} labels={}",
            scene.confluence_state,
            scene.reveal_factor,
            scene.nodes.len(),
            scene.labels.len()
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn expansion_and_collapse_are_processed_by_rust_core() {
        let input = Cursor::new("2\n1\nquit\n");
        let mut output = Vec::new();
        run(input, &mut output).expect("synthetic commands succeed");
        let result = String::from_utf8(output).expect("valid UTF-8");
        assert!(result.contains("state=Expanded reveal=1.00"));
        assert!(result.contains("state=Collapsed reveal=0.00"));
    }

    #[test]
    fn pin_persists_until_explicit_collapse() {
        let input = Cursor::new("3\ntick\n1\nquit\n");
        let mut output = Vec::new();
        run(input, &mut output).expect("synthetic commands succeed");
        let result = String::from_utf8(output).expect("valid UTF-8");
        assert_eq!(result.matches("state=Pinned").count(), 2);
        assert!(result.contains("state=Collapsed reveal=0.00"));
    }

    #[test]
    fn unknown_command_does_not_trigger_navigation() {
        let input = Cursor::new("unknown\nquit\n");
        let mut output = Vec::new();
        run(input, &mut output).expect("synthetic commands succeed");
        let result = String::from_utf8(output).expect("valid UTF-8");
        assert!(result.contains("Comando desconhecido: unknown"));
        assert!(!result.contains("state=Expanded"));
    }
}
