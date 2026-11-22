# A1.2 — Interação orientada a eventos no núcleo Rust

A A1.2 acrescenta à CLI um **loop de comandos textual** que envia intenções ao `AranduCore`. Isso demonstra que o Rust processa sinais da Confluência e da navegação sem depender do navegador.

**Não** é ainda uma janela gráfica nativa nem compositor Wayland. A entrada é apenas textual via stdin e os resultados são resumos da cena produzida pelo núcleo com aplicativos sintéticos.

## Testar sem modificar a sessão gráfica

No checkout da branch A1.2:

```bash
cargo test --workspace --locked
cargo run --locked -p arandu-cli -- --interactive-core
```

Comandos disponíveis: `1` (recolher), `2` (expandir), `3` (alternar fixação), `memory`, `open`, `frequent`, `next`, `previous`, `tick` e `quit`.

Também é possível fazer teste determinístico:

```bash
printf '2\nmemory\nnext\n1\nquit\n' | cargo run --locked --quiet -p arandu-cli -- --interactive-core
```

## Invariantes

- Somente aplicativos sintéticos.
- Nenhum acesso à Semente viva, à ECA ou ao gerenciador de janelas do Linux.
- O tempo lógico é fornecido pelo loop, de maneira determinística; não representa relógio de sessão real.
- Eventos de navegação e sinais da Confluência passam pela API do core.
- Os estados das cenas são derivados do Rust, não inventados por um cliente JavaScript.
- A1.1 (HTML) continua sendo uma demonstração com instantâneos. A1.2 é um **loop Rust separado**, ainda sem vínculo vivo com os botões HTML.

## Caminho para uma janela nativa

1. Instrumentar entradas reais do teclado/mouse de forma controlada.
2. Adicionar adaptador de janela nativa em processo comum, sem substituir o desktop.
3. Ligar o loop de eventos ao renderer mantendo testes de regressão.
4. Medir CPU, RAM, latência e falhas.
5. Investigar Wayland/Smithay somente após confiabilidade do processo isolado.

A prioridade é validar a interação e preservar a máquina do usuário.
