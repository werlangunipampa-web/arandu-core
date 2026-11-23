# ADR A8.8 — Três Rios sob demanda e Semente recolhível

**Data:** 23/11/2026.  
**Estado:** piloto implementado no adaptador GNOME Shell 46, com fontes e instalador experimentais entregues na conversa; **ainda não validado na máquina GNOME real**.

## A experiência

O desktop não precisa desenhar faixas extensas de Rios. A pessoa solicita **Confluência** e a Opy mostra um painel único de três colunas:

1. **Memória:** aplicações usadas antes, incluindo as já encerradas, se o histórico foi consentido; abrir de novo o aplicativo não restaura documentos não salvos.
2. **Abertos:** janelas individuais em execução, inclusive recolhidas (minimizadas); múltiplas janelas do mesmo app continuam distintas. Toque no cartão foca/retoma, e a tecla R (com cartão focado) ou clique direito solicita Recolher por Rust A8.4.
3. **Frequentes:** aplicações com contadores de frequência observados localmente, quando permitidos; não inferir listas fictícias.

Sem Opy aberta, nenhum dos três Rios é desenhado permanentemente. O painel não é uma barra de tarefas ou dock. Uma solicitação pode destacar uma coluna, mas preservar a visão das três.

## Semente recolhível

A própria Semente pode ser guardada, deixando a tela livre. Isso não pode depender de hover nem de descobrir um ícone invisível: a A8.8 registra **Super+Alt+9** como atalho configurável via schema próprio da extensão e guarda o estado em arquivo local JSON. Se o registro do atalho não for confirmado pelo adaptador, a interface recusa esconder a Semente. O retorno mostra a Semente e, opcionalmente, a Opy. Há alternativa técnica via D-Bus para recuperação em falha.

## Teclas históricas sem conflito

A Confluência legada já detém **Super+Alt+1, 2, 3** para Memória, Abertos e Frequentes, e **Super+Shift+T** para mapa. Em vez de duplicar esses registros, o piloto adiciona uma ponte **assíncrona D-Bus** na extensão Confluência que solicita `Show('memory'|'open'|'frequent'|'all')` ao adaptador A8.8. Se a A8.8 estiver ausente ou indisponível, a Confluência conserva suas rotinas originais. No GNOME regular, o comportamento histórico é preservado.

**Condições:** a ponte exige código local compatível e hash verificável. O instalador recusa substituições quando a Confluência diverge. O GNOME Shell X11 precisa ser recarregado após o patch.

## Separação Rust e plataforma

Novo módulo Rust `crates/arandu-core/src/rios_sob_demanda.rs` modela `SurfaceState` (oculto, Semente somente, três colunas), `ActivityState` (visível, recolhida, encerrada), `RiverProjection` com três Rios sem dependência GNOME, e valida duplicação e estado por janela. O modelo evita confundir janelas com aplicações.

**Limite:** o novo módulo Rust ainda não está ligado diretamente à atualização visual da Opy. A integração visual inicial permanece **GNOME Shell JavaScript**, os dados históricos são consultados pelo `Snapshot` existente da Confluência, e o planejamento Rust da A8.4 segue controlando Recolher/Retomar. A próxima ponte deverá encaminhar a projeção das três colunas ao Rust, preservando IDs individuais e privacidade.

## Segurança e recuperação

- Instalador `arandu_a88_tres_rios_semente.py` com `--test --check --apply --restore`, sem sudo, restrito à sessão ARANDU sobre GNOME 46/X11.
- Verifica hashes exatos da A8.4, schema e identidade da Confluência. Recusa versões não reconhecidas.
- Salva byte a byte `extension.js`, `stylesheet.css`, `metadata.json`, código da Confluência e cópias anteriores dos schemas.
- Compila GSettings em área temporária antes da aplicação; recusa arquivos de esquema sem origem.
- Instala somente após validação; restauração integral dos arquivos existentes, removendo apenas arquivos que não existiam antes.
- O atalho de resgate da Semente não é compartilhado com a versão GNOME regular.
- **Ainda não elimina a barra superior GNOME** porque avisos de privacidade e controles de rede, áudio, agenda e sessão precisam ter alternativas testadas. A ausência definitiva da barra permanece objetivo A9.1, não uma funcionalidade anunciada falsamente nesta fase.
- Gesto de arrastar a **janela real** para o Rio (ADR A8.5) ainda é futuro e não é apresentado como implementado.

## Testes e aceitação

O instalador passou **sete autotestes offline locais**: payload e sintaxe JS, modelo visual, Semente/atalho, ponte sem duplicação de combinações, ausência de caminhos pessoais nos dados, backup/restore sintético e recusa de sobrescrita desconhecida.

Ainda pendentes: compilação do schema no GNOME real, revisão da integração D-Bus no GNOME 46, atalho Super+Alt+9 efetivo, recuperação após guardar Semente, duas telas, modo assistivo, ritmo de navegação, consistência com privacidade de GNOME e medições de CPU e memória.

Relacionados: [ADR A8.6](ADR_A86_RIOS_SEMENTE_RECOLHIDA_ATALHOS.md), [ADR A8.7](ADR_A87_ARANDU_SEM_BARRAS.md), [Plano Diretor](../arquitetura/PLANO_DIRETOR_SUBSTITUICAO_GNOME_69_FUNCOES.md).
