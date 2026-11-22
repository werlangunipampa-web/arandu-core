# A1 — Prévia SVG do ARANDU Desktop (Rust)

Este é o primeiro **adaptador visual reproduzível** da cena fornecida pelo ARANDU CORE. Ele não é um compositor Wayland, não substitui GNOME/Cinnamon e não inicia serviços.

## Uso no checkout do repositório

```bash
cargo test --workspace --locked
cargo run --locked -p arandu-cli -- --preview-svg > /tmp/arandu-a1-preview.svg
```

Abra `/tmp/arandu-a1-preview.svg` num visualizador SVG de sua confiança. A saída contém somente elementos SVG estáticos e dados **sintéticos** embutidos. Não usa rede, não coleta dados do ambiente, não consulta a Semente viva e não executa JavaScript.

## Contrato técnico

1. `AranduCore` recebe observações sintéticas de aplicativos.
2. A máquina de estados expande a Confluência por sinal explícito.
3. `AranduCore::render_scene` computa a geometria e os elementos gráficos sem conhecer SVG.
4. O adaptador `apps/arandu-cli/src/svg_preview.rs` traduz a cena para SVG estático.
5. O renderer usa codificação de entidades XML para nomes variáveis e omite nós ocultos.

A prévia não oferece interação, animação, compositor ou gerenciamento real de janelas. A versão A1 deve permanecer experimental até os testes de plataforma.

## Próximos critérios

- CI verde para a versão da branch com o exportador.
- Conferir visualmente disposição dos três rios em tamanhos diferentes.
- Medir desempenho do renderizador com dados sintéticos e sem motor cognitivo.
- Definir um adaptador gráfico para Wayland/X11 com isolamento e sem instalar por padrão.
- Preservar um login alternativo íntegro; nunca substituir a sessão atual antes dos testes.

**Limite ontológico:** o SVG demonstra navegação, não cognição da Semente nem aprendizagem significativa da ECA.
