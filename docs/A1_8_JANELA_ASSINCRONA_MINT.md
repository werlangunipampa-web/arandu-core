# A1.8 — Janela responsiva do ARANDU sobre Linux Mint

## Entrega

A A1.8 adiciona `scripts/arandu_window_async_mint.py`, uma **segunda janela de laboratório**, agora com comunicação por uma thread de trabalho e filas limitadas. A interface Tk mantém seu próprio loop de eventos, enquanto a thread de fundo controla stdin/stdout do núcleo Rust.

O estado da Confluência, navegação e geometria **continuam em Rust**; Python recebe SVGs e os apresenta num Canvas. A janela não instala uma nova sessão, não registra atalhos globais e não consulta a Semente viva.

```text
Linux Mint: janela Tk (thread da UI)
    │  teclas/botões locais
    ▼
fila de comandos (limite 24)
    │
thread única de comunicação
    │  stdin / stdout, sem rede
    ▼
arandu-cli --window-protocol (Rust)
    │
    └── quadros SVG → fila de eventos (limite 32)
                       │
                       └── Tk.after(40 ms) → atualização da janela
```

## Como testar

No checkout atualizado da branch `a0/validacao-rust-ci`:

```bash
cargo build --locked -p arandu-cli
python3 scripts/arandu_window_async_mint.py
```

O comando abaixo executa a suíte completa e, quando `tkinter` e `xvfb-run` estiverem disponíveis, também testa a nova janela virtual com expansão, fixação, recolhimento e fechamento:

```bash
bash scripts/testar_tudo.sh
```

Para exigir os testes de GUI virtual, use `bash scripts/testar_tudo.sh --require-gui`. O uso de Xvfb não altera a sessão gráfica real. **Nenhuma dependência é instalada automaticamente.**

## Ganhos e limites

- O loop principal Tk **não realiza leitura bloqueante** de stdout Rust.
- Fila de entrada limitada a 24 eventos e fila de respostas limitada a 32; quando a fila satura, a interface apresenta aviso e não aceita comandos adicionais.
- Um watchdog detecta ausência de quadro inicial ou ausência de resposta a comandos (limite de 6 segundos) e sinaliza o problema; o usuário consegue fechar a janela.
- O fechamento solicita a terminação do subprocesso sem esperar o processo no loop de interface.
- O modo `ARANDU_GUI_SMOKE=1` executa interação sintética, exige quatro quadros e encerra a janela; não é ativado automaticamente durante o uso comum.

**Ainda é protótipo:** a thread pode ficar momentaneamente bloqueada em `readline`, apesar de a janela continuar responsiva; não há reconexão do core, classificação formal de eventos, métricas de entrada-na-tela nem garantia de recuperação de todos os estados de erro. O controlador da interface é Python/Tk provisório, não um compositor Wayland em Rust.

## Próximas etapas

A1.9: medir latência real, melhorar protocolo com sequências, respostas de erro e reconexão controlada; A2: estudar uma janela integralmente Rust no Mint, mantendo segurança, acessibilidade, ontologia e independência do ambiente gráfico existente.

**Critério de aceite:** CI verde no commit final e teste humano no Mint. Até lá, manter PR em rascunho.
