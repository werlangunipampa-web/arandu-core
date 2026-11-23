# ARANDU A8.8.1 — Navegação real de medalhões por teclado

**Data:** 23/11/2026 — correção experimental para GNOME 46/X11.

## Problema observado

A A8.8 já abriu a Confluência em três colunas por `Super+Alt+1/2/3`, mas os atalhos históricos `Super+Alt+←/→`, `Super+Alt+Enter` e `Super+Alt+0` continuaram selecionando medalhões no ator gráfico da **Confluência antiga**, que está invisível. Portanto, o usuário não via a seleção e as teclas pareciam não fazer nada.

## Correção incremental, sem novos atalhos globais

A Confluência histórica continua dona dos atalhos do GSettings. O patch A8.8.1 altera seus callbacks para encaminhar por D-Bus às operações `Navigate(direction)`, `Activate()` e `Cancel()` exportadas pela extensão A8.8.1 em `org.gnome.Shell/org/arandulab/Tape/A88`. A nova Opy gerencia explicitamente o **Rio selecionado** e o **índice do medalhão**, destaca a seleção e transfere o foco de teclado.

| Gesto | Efeito esperado no ARANDU |
| --- | --- |
| Super + Alt + 1 | Opy: Memória |
| Super + Alt + 2 | Opy: Abertos |
| Super + Alt + 3 | Opy: Frequentes |
| Super + Alt + ← | Selecionar medalhão anterior na coluna ativa |
| Super + Alt + → | Selecionar medalhão seguinte na coluna ativa |
| Super + Alt + Enter | Ativar ou retomar exatamente o selecionado |
| Super + Alt + 0 | Cancelar e recolher a Opy |
| Esc | Recolher a Opy |
| Super + Shift + T | Confluência completa, três colunas |
| Super + Alt + 9 | Guardar ou recuperar a Semente, quando A8.8 está ativa |

**A navegação não cria novos keybindings globais**, preservando os da Confluência. Ao entrar no GNOME regular, a ponte retorna ao comportamento histórico original. Em Rio vazio, a interface não inventa itens; a seta pode não ter o que selecionar.

## Protótipo de teste sem barra (não é retirada definitiva)

O controlador `updateBar()` da Confluência legada passa a respeitar estritamente `ARANDU_SESSION=1`. Quando a sessão é GNOME regular, ele restaura o painel em vez de aplicar qualquer preferência `panel-mode` persistida na conta.

Na sessão ARANDU:
- `Super+Alt+H` alterna visível → automático → oculto → visível (conforme ponto de partida);
- `Super+Alt+B` restaura o painel no ato e ajusta o modo para visível.

A8.8.1 **não oculta automaticamente a barra**. A experiência sem barra é um teste manual e reversível, não a arquitetura A9.1 definitiva: avisos de câmera/microfone/compartilhamento, energia, som e rede ainda precisam de equivalentes acessíveis antes de remover o painel do caminho cotidiano. O painel poderá reaparecer em contextos de visão geral ou em casos de segurança do GNOME.

## Arquivos e recuperação

Instalador local `arandu_a881_navegacao_medalhoes.py` com `--test`, `--check`, `--apply`, `--restore`. Sem `sudo`. Exige identidade A8.8 exata e manifesto íntegro. Atualiza **somente** JS/CSS da extensão Semente e JS da Confluência; nenhuma configuração global de tecla, painel ou serviço é alterada na instalação. O instalador compara hashes antes/depois e conserva backup byte a byte. A restauração arquiva seu manifesto em pasta histórica e permite reaplicar o patch, com restauração novamente.

Validações locais: 7 autotestes, verificação de sintaxe JS com Node, ensaio sintético completo `--apply → --check → --restore → --check → --apply → --restore` comprovando bytes idênticos e proteção da preferência de barra da sessão GNOME. **Não se trata de teste em GNOME real**; esse teste depende da execução voluntária na máquina do usuário.

## Preservação da ontologia

Os Rios não passam a ser uma nova barra. São visões contextuais que se manifestam só quando convocadas. O teclado é modo de navegação de primeira classe, compatível com futuros acionadores e dispositivos assistivos. A operação de retomar depende do plano de janela da A8.4 em Rust; a seleção visual continua temporariamente no adaptador GNOME Shell.

Relacionados: [A8.8](ADR_A88_TRES_RIOS_SOB_DEMANDA.md), [A8.7 Sem Barra](ADR_A87_ARANDU_SEM_BARRAS.md), [Plano Diretor](../arquitetura/PLANO_DIRETOR_SUBSTITUICAO_GNOME_69_FUNCOES.md).
