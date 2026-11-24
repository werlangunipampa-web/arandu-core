# ADR A9.1 — Sistema Rust sobre A8.8.2 e correção do instalador

**Data:** 23/11/2026  
**Estado:** código e instalador offline produzidos e ensaiados; instalação na máquina real ainda não validada.

## Motivação

O usuário validou a instalação A8.8.2 com rolagem dos Rios, depois de recuperar uma instalação A9.0 interrompida. A falha A9.0 apresentou `ARANDU_A90_ERROR: Arquivo ausente ou inesperado: .../a90-sistema/bin/arandu-sistema` e o recuperador identificou `arandu-sistema: parcial_a90`, enquanto os arquivos GNOME principais permaneciam originais.

A revisão do instalador A9.0 encontrou uma provável causa: `safe()` limitava a **4.000.000 bytes** qualquer arquivo existente, inclusive o executável Rust compilado. Um executável válido maior que esse limite seria rejeitado após a gravação, ocasionando interrupção. O diagnóstico concreto no PC não incluiu o tamanho real do binário; por isso a explicação é uma causa técnica consistente, não uma medição daquele arquivo.

## Solução A9.1

1. Integrar **A8.8.2** (scroll nas três colunas, grade de aplicativos paginada e seleção por teclado) ao Sistema Rust prototipado na A9.0 por merge de três vias sobre a A8.8.1.
2. Instalar binário exclusivo em `~/.local/share/arandu/a91-sistema/bin/arandu-sistema`, sem reutilizar o nome da pasta da A9.0 recuperada.
3. Separar o limite de arquivos: fontes GNOME até 1 MiB, binário até **32 MiB**, com SHA-256 calculado em streaming para inspeção de binários.
4. Compilar e executar os **testes Rust antes de modificar qualquer arquivo GNOME**, com `-C debuginfo=0 -C strip=symbols`.
5. Guardar hash anterior e posterior por arquivo no manifesto, arquivos originais byte a byte, restaurar somente estados reconhecidos e **recusar divergências externas**.
6. Preservar GNOME original, Confluência e keybindings: `Super+Alt+1/2/3`, setas, Enter, cancelar, `Super+Alt+9` continuam na base atual.
7. **Não esconder a barra** ainda: faltam equivalentes para avisos críticos de privacidade e notificações.

## Escopo funcional

- **Opy > Sistema** consulta status geral da rede e rádio Wi-Fi por `nmcli`.
- A pessoa liga/desliga Wi-Fi somente clicando explicitamente.
- Consulta volume e mute padrão por `wpctl` com fallback para `pactl`.
- Ajusta +/- 5% ou alterna mudo, sem shell arbitrário e com lista fechada de argumentos.
- Energia, calendário e acesso à acessibilidade permanecem por controles existentes; não alegar que foram reimplementados no Rust.
- A apresentação ainda usa JavaScript GNOME Shell como adaptador. A lógica de controle está em Rust, sem reinventar NetworkManager ou PipeWire.

## Validação feita fora da máquina do usuário

- 9 autotestes offline do instalador, incluindo payload, JS, proteção de symlink, limites e resgate.
- Ensaio sintético: `--check → --apply → --check → --restore → --check → --apply → --restore`.
- Simulação de binário ELF com mais de **5 MiB**, recuperável sem falso bloqueio de tamanho.
- Simulação de instalação interrompida e restauração de arquivos preservados.
- Simulação de arquivo modificado por outra fonte: rollback **recusado**, sem sobrescrever.
- Sintaxe do JS combinada conferida com Node.
- Executável Rust real ainda será compilado e testado pelo instalador na máquina do usuário (rustc indisponível no ambiente de empacotamento). CI do GitHub para a implementação Rust de referência é uma checagem adicional, não substitui o teste gráfico real.

## Caminho no computador

Baixar `arandu_a91_sistema_rust.py` em `~/Downloads`, executar `--test`, `--check`. **Somente** quando aparecer `ARANDU_A91_COMPATIBLE`, executar `--apply` sem sudo, salvar documentos e recarregar GNOME X11 com `Alt+F2 → r → Enter`. Após confirmar as funções da Opy, testar rolagem e atalhos dos Rios.

Se `ARANDU_A91_BLOCKED` aparecer, não sobrescrever a base; investigar primeiro. Para voltar, `--restore` restaura A8.8.2 apenas em estados reconhecidos e arquiva backups.

## Próximo marco

A9.2, não A9.1, deverá abordar a migração de avisos e indicadores de privacidade em múltiplos monitores, antes de estabelecer sessão ARANDU sem barra por padrão.

[Plano Diretor](../arquitetura/PLANO_DIRETOR_SUBSTITUICAO_GNOME_69_FUNCOES.md) · [Rolagem A8.8.2](ADR_A882_ROLAGEM_TOTAL_RIOS.md) · [Sistema A9.0](ADR_A90_OPY_SISTEMA_RUST.md) · [Sem barras](ADR_A87_ARANDU_SEM_BARRAS.md)
