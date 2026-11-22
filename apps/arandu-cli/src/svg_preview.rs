//! A1: isolated SVG preview adapter driven exclusively by ARANDU Core scenes.
//! Synthetic apps only; no GNOME/Wayland integration or filesystem access.

use arandu_confluence::ConfluenceSignal;
use arandu_core::AranduCore;
use arandu_geometry::{Point, Viewport};
use arandu_model::{ApplicationId, ApplicationNode, RiverKind};
use std::time::Duration;

pub fn preview() -> String {
    let mut core = AranduCore::default();
    for (id, name) in [
        ("browser", "Navegador"),
        ("editor", "Editor"),
        ("files", "Arquivos"),
        ("terminal", "Terminal"),
        ("notes", "Notas"),
    ] {
        core.register_application(ApplicationNode::new(
            ApplicationId::new(id),
            name,
            format!("{id}.desktop"),
        ));
    }
    for id in ["browser", "editor", "files", "terminal", "notes"] {
        let app = ApplicationId::new(id);
        let _ = core.observe_launch(&app);
        let _ = core.observe_focus(&app);
    }
    let _ = core.handle_confluence_signal(ConfluenceSignal::Expand, Duration::ZERO);
    let viewport = Viewport::new(1100.0, 720.0);
    let scene = core.render_scene(viewport, Point::new(550.0, 360.0));
    render_svg(&scene, 1100, 720)
}

fn escape_xml(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(ch),
        }
    }
    output
}

pub fn render_svg(scene: &arandu_render_api::RenderScene, width: u32, height: u32) -> String {
    use std::fmt::Write as _;

    let mut out = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" role=\"img\" aria-label=\"ARANDU Confluência, prévia sintética\">\n"
    );
    out.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#0b110e\"/>\n");
    out.push_str("<text x=\"42\" y=\"68\" fill=\"#f5ead6\" font-size=\"25\" font-family=\"sans-serif\">ARANDU • Confluência</text>\n");
    out.push_str("<text x=\"42\" y=\"96\" fill=\"#b9a77e\" font-size=\"13\" font-family=\"sans-serif\">Protótipo A1 • dados sintéticos • nenhum sistema alterado</text>\n");
    for connector in &scene.connectors {
        if connector.opacity <= 0.0 {
            continue;
        }
        let _ = writeln!(
            out,
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#8b9e7a\" stroke-opacity=\"{:.2}\" stroke-width=\"2\"/>",
            connector.from.x,
            connector.from.y,
            connector.to.x,
            connector.to.y,
            connector.opacity
        );
    }
    for node in &scene.nodes {
        if node.opacity <= 0.0 {
            continue;
        }
        let stroke = if node.focused { "#e6d5a6" } else { "#a7864e" };
        let _ = writeln!(
            out,
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#20231d\" stroke=\"{stroke}\" stroke-width=\"2\" opacity=\"{:.2}\"/>",
            node.center.x,
            node.center.y,
            node.radius * node.scale,
            node.opacity
        );
        let _ = writeln!(
            out,
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"12\" fill=\"#f5ead6\" opacity=\"{:.2}\">{}</text>",
            node.center.x,
            node.center.y + node.radius * node.scale + 18.0,
            node.opacity,
            escape_xml(&node.name)
        );
    }
    // A1.10 — lightweight concentric rings inspired by the existing GNOME
    // ARANDU Semente: gold outer rim, dark face, contrasting inner relief.
    // Keep every primitive within the original Tk SVG subset.
    for (factor, fill, stroke, width) in [
        (1.35_f32, "#0b110e", "#a7864e", 1.0_f32),
        (1.22, "#20231d", "#c6a66e", 2.0),
        (1.08, "#252824", "#f5dd98", 2.0),
        (0.92, "#101411", "#876735", 1.5),
    ] {
        let _ = writeln!(
            out,
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{width}\" opacity=\"{:.2}\"/>",
            scene.seed.center.x,
            scene.seed.center.y,
            scene.seed.radius * scene.seed.scale * factor,
            scene.seed.opacity
        );
    }
    let _ = writeln!(
        out,
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#c6a66e\" stroke=\"#f7e6ae\" stroke-width=\"1\" opacity=\"{:.2}\"/>",
        scene.seed.center.x,
        scene.seed.center.y,
        scene.seed.radius * scene.seed.scale * 0.18,
        scene.seed.opacity
    );
    for label in &scene.labels {
        let label_text = match label.river {
            RiverKind::Memory => "MEMÓRIA",
            RiverKind::Open => "ABERTOS",
            RiverKind::Frequent => "FREQUENTES",
        };
        let _ = writeln!(
            out,
            "<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#d6bb79\" font-family=\"sans-serif\" font-size=\"14\" opacity=\"{:.2}\">{label_text}</text>",
            label.anchor.x + 30.0,
            label.anchor.y - 12.0,
            label.opacity
        );
    }
    out.push_str("</svg>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_an_svg_from_the_real_core_scene() {
        let result = preview();
        assert!(result.starts_with("<svg "));
        assert!(result.contains("MEMÓRIA"));
        assert!(result.contains("ABERTOS"));
        assert!(result.contains("FREQUENTES"));
        assert!(result.contains("<circle "));
        assert!(result.ends_with("</svg>\n"));
    }

    #[test]
    fn escapes_external_text() {
        assert_eq!(
            escape_xml("<script>&\"'"),
            "&lt;script&gt;&amp;&quot;&apos;"
        );
    }

    #[test]
    fn collapsed_scene_does_not_display_hidden_nodes() {
        let core = AranduCore::new(5);
        let scene = core.render_scene(Viewport::new(800.0, 600.0), Point::new(400.0, 300.0));
        let result = render_svg(&scene, 800, 600);
        assert!(!result.contains("<line "));
        assert!(result.contains("<circle "));
    }
}
