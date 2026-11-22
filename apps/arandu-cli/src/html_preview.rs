//! A1.1 standalone interactive browser preview from precomputed Rust scenes.
//!
//! The browser switches between three Rust-rendered snapshots. This is a
//! presentation experiment, not a live Rust event loop or Linux compositor.
use arandu_confluence::ConfluenceSignal;
use arandu_core::AranduCore;
use arandu_geometry::{Point, Viewport};
use arandu_model::{ApplicationId, ApplicationNode};
use std::time::Duration;

use crate::svg_preview::render_svg;

fn synthetic_core() -> AranduCore {
    let mut core = AranduCore::default();
    for (id, label) in [
        ("browser", "Navegador"),
        ("editor", "Editor"),
        ("files", "Arquivos"),
        ("terminal", "Terminal"),
        ("notes", "Notas"),
    ] {
        core.register_application(ApplicationNode::new(
            ApplicationId::new(id),
            label,
            format!("{id}.desktop"),
        ));
        let key = ApplicationId::new(id);
        let _ = core.observe_launch(&key);
        let _ = core.observe_focus(&key);
    }
    core
}

fn svg_snapshot(core: &AranduCore) -> String {
    let scene = core.render_scene(Viewport::new(1100.0, 720.0), Point::new(550.0, 360.0));
    render_svg(&scene, 1100, 720)
}

#[must_use]
pub fn preview() -> String {
    let mut core = synthetic_core();
    let collapsed = svg_snapshot(&core);
    let _ = core.handle_confluence_signal(ConfluenceSignal::Expand, Duration::from_secs(1));
    let expanded = svg_snapshot(&core);
    let _ = core.handle_confluence_signal(ConfluenceSignal::TogglePin, Duration::from_secs(2));
    let pinned = svg_snapshot(&core);

    let header = r#"<!doctype html>
<html lang="pt-BR">
<head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="color-scheme" content="dark">
<title>ARANDU A1.1 — Prévia local</title>
<style>
:root { font-family: system-ui, sans-serif; color: #dceee6; background: #09171a; }
body { margin: 0; padding: 1rem; }
main { max-width: 1100px; margin: auto; }
h1 { font-size: 1.2rem; margin: .5rem 0; }
p { color: #b1c7bd; line-height: 1.5; }
nav { display: flex; flex-wrap: wrap; gap: .7rem; margin: 1rem 0; }
button { background: #15372e; color: #e5f2e8; border: 1px solid #78b79d;
  border-radius: .65rem; padding: .75rem 1rem; font: inherit; cursor: pointer; }
button:focus-visible { outline: 3px solid #e5d795; outline-offset: 2px; }
button[aria-pressed="true"] { background: #386957; border-color: #dce9b9; }
.scene { display: none; }
.scene.active { display: block; }
.scene svg { width: 100%; height: auto; border-radius: .8rem; }
small { color: #a8bcb4; }
</style></head><body><main>
<h1>🌱 ARANDU — Semente da Confluência</h1>
<p>Prévia interativa local com cenas pré-calculadas pelo núcleo Rust.
Nenhum aplicativo real é iniciado. Nenhuma informação da Semente oficial é acessada.</p>
<nav aria-label="Estados da Confluência">
<button type="button" data-state="collapsed" aria-pressed="true">1 · Semente</button>
<button type="button" data-state="expanded" aria-pressed="false">2 · Três Rios</button>
<button type="button" data-state="pinned" aria-pressed="false">3 · Fixados</button>
</nav>
<p id="status" role="status" aria-live="polite">Estado visual: Semente recolhida</p>
"#;
    let footer = r#"<p><small>Teclas: 1, 2, 3 ou Escape para recolher. Esta é uma
demonstração de estados; o navegador não executa a máquina de estados Rust ao vivo.</small></p>
</main>
<script>
(() => {
  const labels = {
    collapsed: 'Semente recolhida',
    expanded: 'Três Rios expandidos',
    pinned: 'Rios fixados'
  };
  function show(state) {
    if (!Object.hasOwn(labels, state)) return;
    for (const el of document.querySelectorAll('.scene')) {
      el.classList.toggle('active', el.dataset.scene === state);
    }
    for (const button of document.querySelectorAll('button[data-state]')) {
      button.setAttribute('aria-pressed', String(button.dataset.state === state));
    }
    document.getElementById('status').textContent = 'Estado visual: ' + labels[state];
  }
  document.querySelectorAll('button[data-state]').forEach(button =>
    button.addEventListener('click', () => show(button.dataset.state))
  );
  document.addEventListener('keydown', event => {
    const target = {'1':'collapsed', '2':'expanded', '3':'pinned', Escape:'collapsed'}[event.key];
    if (target) show(target);
  });
})();
</script></body></html>
"#;
    let mut output = String::from(header);
    for (state, svg) in [
        ("collapsed", collapsed),
        ("expanded", expanded),
        ("pinned", pinned),
    ] {
        let active = if state == "collapsed" { " active" } else { "" };
        output.push_str(&format!(
            "<section class=\"scene{active}\" data-scene=\"{state}\" aria-label=\"Prévia {state}\">\n{svg}</section>\n"
        ));
    }
    output.push_str(footer);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_contains_three_rust_generated_snapshots() {
        let html = preview();
        assert!(html.starts_with("<!doctype html>"));
        assert_eq!(html.matches("<svg ").count(), 3);
        assert!(html.contains("data-scene=\"collapsed\""));
        assert!(html.contains("data-scene=\"expanded\""));
        assert!(html.contains("data-scene=\"pinned\""));
        assert!(html.contains("aria-pressed=\"true\""));
        assert!(html.ends_with("</html>\n"));
    }

    #[test]
    fn snapshots_are_deterministic() {
        assert_eq!(preview(), preview());
    }
}
