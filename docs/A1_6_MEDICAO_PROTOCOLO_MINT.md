# A1.6 — Medição reproduzível do protocolo gráfico no Linux Mint

## Motivação

A janela de laboratório A1.5 abriu na máquina Linux Mint do projeto. A A1.6 passa a **medir** a comunicação local entre um adaptador e o processo Rust, em vez de afirmar que a arquitetura é rápida sem evidências.

## Entrega

- `scripts/benchmark_window_protocol.py`: envia uma sequência determinística de expansão e recolhimento ao núcleo Rust por stdin e recebe um quadro SVG completo por comando, mais o quadro inicial.
- Verifica contagem de respostas, prefixo `SCENE <svg ` e encerramento `</svg>`.
- Gera JSON com quantidade de quadros, duração total do lote e taxa média aproximada de quadros por segundo.
- O GitHub Actions executa 20 comandos sintéticos como smoke test de ponta a ponta.

## Testar no Linux Mint

No checkout da branch de desenvolvimento:

```bash
cargo test --workspace --locked
python3 scripts/benchmark_window_protocol.py --build --frames 100
python3 scripts/arandu_window_mint.py
```

O primeiro comando confirma os testes Rust. O segundo compila a CLI e executa a medição sintética, sem janela gráfica. O terceiro abre a janela de laboratório, sob demanda.

## Como interpretar

O valor `batch_frames_per_second` **não** é taxa de atualização da janela nem medição de latência entre pressionar uma tecla e ver o resultado. A medição é feita com um único processo Rust local, stdin/stdout, entradas sintéticas e sem desenho Tkinter. Ela inclui criação do subprocesso; a compilação acontece **antes** da medição.

Resultados variam conforme hardware, estado do computador, versão de Rust e carga da máquina. Não comparar esse valor diretamente com FPS de GNOME/Cinnamon ou Windows.

## Limitações e próximos critérios

1. A janela Tkinter A1.5 ainda usa leitura síncrona do subprocesso; se o núcleo Rust travar, a interface pode esperar indefinidamente. **Não é produção**.
2. Próxima etapa: IPC assíncrono, watchdog local e capacidade de fechar a janela mesmo com o backend indisponível.
3. Medir memória residente, CPU, inicialização e latência de entrada/atualização gráfica no Linux Mint com ferramentas de observação apropriadas.
4. Não coletar dados de usuário, não registrar teclas globais nem ler estado vivo da Semente.
5. Não substituir Cinnamon ou instalar um compositor na sessão atual durante os testes.

## Condição de aprovação

CI verde no último commit, revisão do protocolo e verificação manual no Linux Mint. A A1.6 não implementa um compositor ou uma nova sessão Linux.
