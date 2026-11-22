# A1.7 — Autoteste completo por um comando Bash

No checkout da branch `a0/validacao-rust-ci` do projeto, execute:

```bash
bash scripts/testar_tudo.sh
```

O comando verifica, em sequência: versões, `rustfmt`, `cargo check`, testes Rust, Clippy com avisos tratados como erros, sintaxe Python, testes unitários e de integração Python, compilação da CLI, cenas SVG e HTML, estados de Confluência, protocolo local de quadros, benchmark sintético de 100 comandos e, **se** `xvfb-run` e `tkinter` estiverem presentes, abertura e fechamento automáticos de uma janela Tk numa tela virtual.

```bash
bash scripts/testar_tudo.sh --no-gui
```

Executa apenas testes sem interface gráfica. Para exigir a janela virtual e falhar caso suas dependências não estejam disponíveis:

```bash
bash scripts/testar_tudo.sh --require-gui
```

## Resultados e segurança

- Cada etapa escreve um arquivo de log numa pasta temporária privada em `/tmp/arandu-autoteste.*`. O caminho aparece ao final, inclusive quando há falha.
- O script para na primeira etapa que falhar, sem declarar aprovação antecipada.
- A GUI virtual é um teste automatizado de laboratório; **não** substitui nem valida integralmente a sessão Linux Mint/Cinnamon real.
- Nenhum pacote é instalado; nenhum gerenciador de sessão é configurado.
- Nenhum dado da Semente oficial é carregado; os testes Rust usam aplicativos sintéticos.
- O benchmark mede *throughput do lote* e não FPS nem latência de percepção humana.

## Limitações e próximas etapas

A janela A1.5 ainda usa IPC síncrono. A próxima etapa A1.8 deverá investigar leitura não bloqueante e encerramento robusto do subprocesso sem afetar o desktop atual. Testes de estabilidade, CPU, RAM e acessibilidade em hardware Linux Mint continuam necessários.

A aprovação só vale para o commit exato em que a CI passa.
