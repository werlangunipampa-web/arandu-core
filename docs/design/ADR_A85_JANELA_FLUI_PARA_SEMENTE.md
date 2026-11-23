# ADR A8.5 — A janela flui para a Semente

**23/11/2026 — decisão de design; ainda não implementada no GNOME.**

## Correção de entendimento

O usuário quer mover **a janela real do aplicativo** até o Rio/Semente, não arrastar um cartão na Opy nem apenas substituir os botões `—`, `□` e `X` por outros botões. O objetivo é substituir progressivamente a tríade de controles no canto da janela por um paradigma espacial de atividades.

## Recolher ao Rio

1. A pessoa segura e move a janela real pela região de arraste disponível.
2. Somente durante o movimento, a Semente revela uma pequena **zona de acolhimento — Abertos / Recolher**, no monitor onde está posicionada. Não surgem faixas grandes decorativas.
3. Ao aproximar o ponteiro, a zona destaca o destino e mostra texto acessível; proximidade sozinha não age.
4. Ao soltar dentro da zona, o adaptador GNOME verifica novamente a janela específica e solicita o plano Rust `WindowIntent::RecolherAoRio`.
5. O GNOME minimiza somente aquela janela. Seu medalhão permanece em Abertos, distinguindo múltiplas janelas de um mesmo app.
6. Tocar no medalhão ou acessá-lo por teclado/Alt+Tab executa Retomar para a janela exata.

**Arrastar a Semente e arrastar uma janela são dois gestos diferentes.** A posição livre da Semente em qualquer monitor continua valendo.

## Imersão, Confluência, Encerrar

- Imersão: movimentar para uma região de expansão, com maximização reversível na área útil, não fullscreen obrigatório.
- Confluência: aproximação ou região de composição sugere lado a lado, exigindo seleção da segunda janela e opção Desfazer.
- Encerrar: **nunca** será consequência automática de arrastar a janela perto da Semente, para fora da tela ou para um canto. Exige comando inequívoco e respeita avisos de documentos não salvos.

## Acessibilidade e atalhos

Gestos são um caminho, não o único caminho. Preservar Escape, Tab/Shift+Tab, setas, Enter/Espaço, atalhos de Rios e Alt+Tab. Ações devem ser alcançáveis também por menu contextual, teclado remapeável e tecnologias assistivas; futuro acesso por voz e acionadores será explícito, com consentimento. Nenhum conflito com atalhos GNOME deve ser imposto.

## Viabilidade e limites no GNOME 46

O Mutter emite sinais `grab-op-begin` e `grab-op-end` quando uma janela é arrastada. O adaptador deverá aceitar apenas movimentação verdadeira, e não resize ou movimento do próprio medalhão. Ocultar os controles tradicionais pode funcionar para muitas decorações, mas nem todos os aplicativos obedecem às preferências GNOME, especialmente barras desenhadas pelos próprios aplicativos (CSD).

**A8.5 — piloto de observação:** exibir a zona quando uma janela começa a mover; nenhum efeito mutante, testar GNOME real.
**A8.6 — piloto de ação:** drop explícito aciona Recolher/Retomar via plano Rust; conferir referências, permissões e cancelamento.
**Etapa posterior:** ocultar opcionalmente `— □ X` apenas na sessão ARANDU e somente onde compatível, após alternativas completas de teclado, confirmação, recuperação e testes com diferentes aplicações. A sessão GNOME original permanece inalterada.

## Requisitos de validação

- Testar janela perto da zona, mas solta fora: **nenhuma ação**.
- Testar janela solta dentro: somente a janela escolhida é recolhida; outras janelas do mesmo app permanecem.
- Confirmar retorno de monitor, workspace, geometria e foco.
- Testar dois monitores, Semente nos cantos e bordas, escala e movimento reduzido.
- Nenhuma sobreposição invisível bloqueia aplicativos; sem polling contínuo em repouso.
- Verificar cancelamento por Escape e alternativas assistivas.
- Manter backup e restauração integral da extensão anterior.

## Filosofia

Rios não são faixas obrigatórias no papel de parede: são lugares de continuidade. O usuário escolhe para onde levar uma atividade, e a Semente torna visível o destino somente quando isso ajuda a agir. Esse é o vínculo entre territorialidade, diferenciação progressiva e autonomia humana.

Fontes técnicas: https://mutter.gnome.org/meta/signal.Display.grab-op-begin.html ; https://mutter.gnome.org/meta/signal.Display.grab-op-end.html ; https://docs.gtk.org/gtk4/property.Settings.gtk-decoration-layout.html

**Pergunta fundadora:** não 'qual botão clico?', mas 'para onde quero levar esta atividade?'.