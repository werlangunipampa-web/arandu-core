# Changelog

## 0.2.0 — Semente da Confluência
- máquina de estados universal `Collapsed/Aware/Near/Expanded/Pinned`;
- proximidade normalizada e independente de hardware;
- recolhimento automático determinístico por inatividade;
- modo `Pinned`;
- eventos para renderer/adapters;
- integração no `AranduCore`;
- novos testes e gate de qualidade.


## 0.1.2 — Clippy Gate Completion

- documenta `# Errors` em todas as operações do `PlatformAdapter`;
- documenta `# Panics` dos acessores de rios que protegem invariantes internas;
- ajusta durações de testes/demonstração para evitar lints pedânticos sem elevar o MSRV;
- preserva integralmente a lógica dos três rios e das intenções;
- adiciona gate v0.1.2 para `check`, `test` e `clippy -D warnings`.


## 0.1.1 — Clippy Gate Hardening

- preserva a lógica validada do ARANDU CORE 0.1.0;
- adiciona `#[must_use]` às APIs cujo resultado não deve ser descartado silenciosamente;
- documenta os `# Panics` dos construtores que rejeitam capacidade zero;
- documenta `# Errors` da sincronização com adaptadores de plataforma;
- ajusta CLI e testes para consumir explicitamente eventos intencionalmente ignorados;
- prepara o workspace para o gate `cargo clippy --workspace --all-targets -- -D warnings`.

## 0.1.0 — Three Rivers Core Foundation

- núcleo universal em Rust;
- rios ABERTOS, MEMÓRIA e FREQUENTES;
- confluência e motor de intenções;
- abstrações de plataforma e modalidades de entrada;
- ontologia inicial e CLI demonstrativa.

## 0.3.0

- Novo `arandu-geometry`.
- `Viewport`, `Point`, `RiverCounts` e `AdaptiveLayout` independentes de plataforma.
- Inversão automática de rios em bordas.
- Compactação adaptativa de espaçamento.
- Clamping da Semente e dos nós em margem segura.
- Integração da geometria ao `AranduCore` usando contagens reais dos três rios.
- Seis testes do motor geométrico e um teste de integração no Core.
