//! A1.5: line-oriented SVG scene protocol for a local window adapter.
//!
//! The Rust core owns state and geometry. The process only reads commands on
//! stdin and returns one static SVG scene per accepted command on stdout.
//! No global hooks, filesystem writes, network, or compositor integration.

use arandu_core::AranduCore;
use arandu_geometry::{Point, Viewport};
use arandu_input_api::desktop_keyboard::{DesktopKey, KeyTransition};
use arandu_model::{ApplicationId, ApplicationNode};
use std::io::{self, BufRead, Write};
use std::time::Duration;

use crate::native_input_bridge::dispatch;
use crate::svg_preview::render_svg;

fn sample_core() -> AranduCore {
    let mut core = AranduCore::new(5);
    for (id, name) in [
        ("browser", "Navegador"),
        ("terminal", "Terminal"),
        ("files", "Arquivos"),
        ("editor", "Editor"),
        ("notes", "Notas"),
    ] {
        let key = ApplicationId::new(id);
        core.register_application(ApplicationNode::new(
            key.clone(),
            name,
            format!("{id}.desktop"),
        ));
        let _ = core.observe_launch(&key);
        let _ = core.observe_focus(&key);
    }
    core
}

fn parse_key(input: &str) -> Option<DesktopKey> {
    match input {
        "1" | "escape" => Some(DesktopKey::Escape),
        "2" => Some(DesktopKey::Digit2),
        "3" => Some(DesktopKey::Digit3),
        "m" => Some(DesktopKey::KeyM),
        "o" => Some(DesktopKey::KeyO),
        "f" => Some(DesktopKey::KeyF),
        "left" => Some(DesktopKey::ArrowLeft),
        "right" => Some(DesktopKey::ArrowRight),
        "up" => Some(DesktopKey::ArrowUp),
        "down" => Some(DesktopKey::ArrowDown),
        _ => None,
    }
}

fn output_scene<W: Write>(core: &AranduCore, output: &mut W) -> io::Result<()> {
    let viewport = Viewport::new(1100.0, 720.0);
    let scene = core.render_scene(viewport, Point::new(550.0, 360.0));
    let svg = render_svg(&scene, 1100, 720);
    writeln!(output, "SCENE {}", svg.replace('\n', ""))?;
    output.flush()
}

/// Processes normalized keyboard commands, returning an SVG scene after each
/// accepted event. The `quit` command stops the process.
///
/// # Errors
///
/// Returns I/O errors from stdin/stdout. Unrecognized commands are ignored.
pub fn run<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut core = sample_core();
    let mut now = Duration::ZERO;
    output_scene(&core, &mut output)?;
    for line in input.lines() {
        let line = line?;
        let command = line.trim();
        if command == "quit" {
            break;
        }
        let Some(key) = parse_key(command) else {
            continue;
        };
        now += Duration::from_millis(100);
        if dispatch(&mut core, key, KeyTransition::Press, now).is_some() {
            output_scene(&core, &mut output)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn initial_and_expanded_scenes_are_generated_by_rust() {
        let mut out = Vec::new();
        run(Cursor::new("2\nescape\nquit\n"), &mut out).expect("synthetic protocol");
        let text = String::from_utf8(out).expect("UTF-8");
        let frames: Vec<_> = text.lines().collect();
        assert_eq!(frames.len(), 3);
        assert!(frames.iter().all(|line| line.starts_with("SCENE <svg ")));
        assert!(frames[1].contains("MEMÓRIA"));
        assert!(!frames[0].contains("MEMÓRIA"));
        assert!(!frames[2].contains("MEMÓRIA"));
    }

    #[test]
    fn invalid_commands_do_not_emit_new_frames() {
        let mut out = Vec::new();
        run(Cursor::new("invalid\nquit\n"), &mut out).expect("synthetic protocol");
        assert_eq!(String::from_utf8(out).expect("UTF-8").lines().count(), 1);
    }

    #[test]
    fn newline_delimited_frames_are_complete_svg() {
        let mut out = Vec::new();
        run(Cursor::new("3\nquit\n"), &mut out).expect("synthetic protocol");
        let text = String::from_utf8(out).expect("UTF-8");
        assert_eq!(text.matches("</svg>").count(), 2);
    }
}
