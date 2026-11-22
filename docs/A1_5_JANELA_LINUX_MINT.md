# A1.5 — Primeira janela de laboratório no Linux Mint

## O que foi construído

Esta etapa permite abrir **uma janela comum do sistema**, isolada da sessão gráfica existente, para exibir a Semente e os Três Rios. Um adaptador temporário usa **Python Tkinter** somente para criar a janela, desenhar primitivas e capturar teclas **dentro dela**. O núcleo **Rust** continua responsável pelos estados da Confluência, intenções de navegação e geometria dos medalhões.

Isso não é ainda um compositor Wayland nem o ambiente ARANDU completo. Tkinter é um laboratório de apresentação nativa no Linux Mint, não uma escolha definitiva de toolkit.

### Fluxo

```text
Janela local (Tkinter no Linux Mint)
  → tecla/botão da própria janela
  → stdin do processo Rust --window-protocol
  → arandu-input-api → native_input_bridge → AranduCore
  → RenderScene (núcleo Rust) → SVG (adaptador Rust)
  → stdout, quadro SCENE
  → Canvas da janela local
```

O canal de texto usa uma linha por cena, marcada por `SCENE `. O Rust emite o quadro inicial e novo quadro para cada comando reconhecido. Comandos inválidos não disparam atualizações. Os aplicativos são sempre sintéticos.

## Teste explícito no Linux Mint

Requisitos locais: `git`, `cargo` com Rust compatível com o workspace, Python 3 e módulo `tkinter` instalado, além de sessão gráfica funcionando.

```bash
git clone https://github.com/werlangunipampa-web/arandu-core.git
cd arandu-core
git switch a0/validacao-rust-ci
cargo test --workspace --locked
python3 scripts/arandu_window_mint.py
```

Para checar se o Mint possui Tkinter: `python3 -c 'import tkinter; print("Tk disponível")'`. Em Linux Mint baseado em Ubuntu, o pacote costuma ser `python3-tk`; a instalação é uma decisão manual do usuário, não é realizada pelo projeto.

**Não execute `git clone` por cima de um checkout existente com alterações locais.** Se já possui o clone, utilize um diretório de trabalho separado ou verifique o estado de suas alterações antes de trocar de branch.

## Controles

- `1` e `Escape`: recolher.
- `2`: expandir.
- `3`: fixar/desafixar.
- `M`, `O`, `F`: selecionar rio.
- Setas: navegar entre medalhões.
- Fechar a janela encerra somente o processo de laboratório.

## Limitações conhecidas

- A renderização Tk reproduz apenas círculos, linhas e texto do SVG; não possui animações, fidelidade de tema, ícones ou composição de janelas.
- A comunicação síncrona via subprocesso é apropriada apenas para protótipo local; uma falha do processo Rust pode tornar a interface temporariamente não responsiva até a detecção. A próxima arquitetura exigirá protocolo assíncrono com limites de tempo e controle de processos.
- A versão atual não desenha o destaque de seleção além das informações que já existem nos primitivos renderizados.
- A janela pode funcionar via X11 ou, dependendo da sessão, XWayland. Isso **não** demonstra suporte Wayland nativo.
- Não foram medidos ainda CPU, RAM, latência, consumo de energia ou acessibilidade em hardware real.
- Não há integração com aplicativos reais, Semente oficial, ECA ou ontologia canônica completa.

## Segurança

O programa não instala sessões, não modifica gerenciadores de login, não registra atalhos globais e não altera o desktop padrão do Linux Mint. Sempre mantenha a sessão original disponível.

## Critério de promoção

CI Rust verde; verificação manual no Linux Mint; revisão de segurança do adaptador e posterior substituição por uma janela nativa com backend Rust apropriado.
