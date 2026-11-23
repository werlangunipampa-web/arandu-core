# ADR A9.0 — Controles de sistema na Opy com módulo Rust

**Data:** 23/11/2026.  
**Estado:** piloto de interface/serviço Rust, sem validação ainda na sessão GNOME real.

## Decisão

O ARANDU começou a substituir o menu rápido do GNOME, não apenas sua aparência. A aba **Sistema** da Opy apresenta diretamente o estado da rede e do áudio, com controles reais para:

- estado geral da rede, sem revelar SSIDs ou dados pessoais;
- estado do rádio Wi-Fi; ativar/desativar via clique explícito;
- volume do áudio padrão; aumentar/diminuir em 5%, limitado a 100% quando suportado;
- silenciar/ativar áudio por clique;
- entrada para configuração de redes, som e acessibilidade no Centro de Controle, durante a migração;
- calendário e controles de Energia já presentes na Opy.

## Arquitetura

**Rust** em `tools/a90_sistema/arandu_sistema.rs`: código sem GNOME e sem dependências de crates externos, compilado com `rustc` local. Comandos aceitos: `status`, `wifi-on`, `wifi-off`, `vol-up`, `vol-down`, `mute-toggle`. Todos usam lista fechada de parâmetros para ferramentas Linux (`nmcli`, `wpctl`, `pactl`) por `std::process::Command`, **nunca** via `sh -c`. Retornos são pequenos e sem SSIDs.

**Adaptador GNOME**: a Opy GNOME Shell 46 (GJS) lança o executável Rust num caminho fixo com `Gio.Subprocess` de maneira assíncrona e tempo limite. Atualiza somente quando o usuário abre a aba ou solicita uma ação. Valida rigorosamente um protocolo `A90|1` com estados enum e `A90|OK|ação`. Não registra novos atalhos nem remove as funções dos Três Rios.

**Linux**: NetworkManager e PipeWire/WirePlumber/PulseAudio permanecem os serviços subjacentes. Rust é a camada de controle e protocolo, não um novo gerenciador de redes ou servidor de som. Não tenta alterar dispositivos Bluetooth, VPN, nomes de rede ou privilégios sem autorização.

## Segurança e recuperação

- O instalador `arandu_a90_sistema_rust.py` exige uma A8.8.1 identificada por hashes, manifesto e marcadores, sessão `ARANDU_SESSION=1`, GNOME 46/X11 e usuário sem sudo.
- Compila os testes Rust, executa os testes e constrói o binário **antes de modificar a extensão**.
- Faz backup byte a byte de `extension.js`, `stylesheet.css` e `metadata.json` e instala fontes/binários Rust numa pasta específica.
- `--test --check --apply --restore` são comandos distintos. A restauração compara hashes do estado A9.0 antes de escrever cópias antigas; recusa modificações externas.
- A versão A8.8.1 e a Confluência legada permanecem funcionais; não modifica os atalhos `Super+Alt+1/2/3`, `←/→`, `Enter`, `0` ou o resgate da Semente.
- **A instalação não oculta a barra**. Os indicadores de microfone/câmera/compartilhamento e as notificações ainda são GNOME, por isso não podemos declarar independência do painel nesta fase.

## Teste de aceitação no computador

1. `--test` e `--check` íntegros; executar `--apply`.
2. Recarregar GNOME X11 e abrir Semente → Opy → Sistema. A interface mostra estado real de rede e áudio ou indisponibilidade explícita.
3. Ajustar volume sem abrir o menu GNOME; alternar mute; se for seguro, alternar rádio Wi-Fi (evitar em uso remoto).
4. Testar que as funções Semente/Três Rios e atalhos não regrediram.
5. Recuperar com `--restore` e verificar hash/id original e GNOME comum.
6. Medir tempo de abertura, CPU em espera e custo dos subprocessos Rust antes de alegar ganho de desempenho.

## Evoluções

**A9.1:** calendário/avisos próprios com fontes de dados reais, status/avisos de privacidade em múltiplos monitores, áudio por dispositivo, Bluetooth, energia e rede em controle mais nativo. Apenas depois: padrão sem barra na sessão ARANDU. Sem necessidade de reescrever daemons estáveis.

[Plano Diretor de 69 funções](../arquitetura/PLANO_DIRETOR_SUBSTITUICAO_GNOME_69_FUNCOES.md) · [ADR Sem Barra](ADR_A87_ARANDU_SEM_BARRAS.md) · [Rios A8.8](ADR_A88_TRES_RIOS_SOB_DEMANDA.md).
