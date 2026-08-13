# ARANDU CORE 0.1.0

**Núcleo universal de navegação em três rios para o ecossistema ARANDU.**

ARANDU CORE separa a lógica de navegação da interface gráfica e do sistema operacional. O GNOME passa a ser um adaptador/laboratório visual, enquanto o núcleo define conceitos universais que poderão ser usados em Linux, Windows e macOS.

## Três rios

- **MEMÓRIA** — os aplicativos usados mais recentemente.
- **ABERTOS** — os aplicativos atualmente abertos.
- **FREQUENTES** — os aplicativos mais usados segundo estatística local.

Os três rios encontram-se em uma **Confluência**. A forma de entrada é independente do rio: teclado, mouse, touchpad, voz, sons tonais, movimento de cabeça ou dispositivos assistivos podem produzir as mesmas intenções de navegação.

## Princípio central

> O ARANDU não exige que a pessoa se adapte ao dispositivo de entrada; cada modalidade traduz sinais humanos para intenções universais de navegação.

## Workspace

```text
ARANDU-CORE_v0.1.0/
├── crates/
│   ├── arandu-model/
│   ├── arandu-navigation/
│   ├── arandu-platform-api/
│   ├── arandu-input-api/
│   └── arandu-core/
├── apps/
│   └── arandu-cli/
├── ontology/
├── docs/
└── scripts/
```

## Build

```bash
cargo test --workspace
cargo run -p arandu-cli
```

## Estado 0.1

Esta primeira baseline implementa:

- tipos universais de aplicativo e nó;
- os três rios;
- memória recente limitada;
- estatística de frequência baseada em observações brutas;
- intents de navegação;
- perfis e mapeamentos de entrada;
- traits para adaptadores de plataforma;
- motor central ARANDU;
- eventos de navegação;
- CLI demonstrativa;
- ontologia inicial RDF/Turtle;
- testes unitários sem dependências externas.

## Privacidade

O núcleo 0.1 não usa rede, nuvem, câmera ou microfone. Módulos futuros de voz, som e movimento deverão processar localmente por padrão e entregar somente `NavigationIntent` ao core.


## v0.2.0 — Semente da Confluência

A interface pode permanecer recolhida em uma única Semente central e despertar progressivamente por proximidade, expandir os três rios, ser fixada (`Pinned`) e recolher após inatividade. Toda a máquina de estados está em Rust e é independente de Linux, macOS e Windows. Veja `docs/CONFLUENCE_SEED.md`.


### Toolchain mínima

O workspace declara Rust 1.91 ou superior.

## v0.3.0 — Geometria Adaptativa

O Rust agora calcula a posição da Semente e dos três rios antes da renderização. O motor recebe apenas o tamanho lógico da área disponível, a posição desejada da Semente e a quantidade de nós. Ele pode inverter rios nas bordas, comprimir espaçamentos e manter os nós dentro da área segura. Nenhuma regra geométrica depende de GNOME, Windows ou macOS.

Novo crate: `crates/arandu-geometry`.
