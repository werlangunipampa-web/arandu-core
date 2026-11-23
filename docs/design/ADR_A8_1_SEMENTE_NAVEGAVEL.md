# ADR A8.1 — Semente legível, navegação não bloqueante e energia acessível

**23/11/2026 · status: protótipo A8.1 codificado e testado localmente; ainda não validado no GNOME real e não incorporado à linha principal**

## Contexto e evidências

A pessoa que está experimentando a A8 no GNOME 46 com dois monitores relatou: Semente pequena demais para ver funções e reconhecer ícones; expansão que atrapalha a navegação por outras partes do ambiente; desejo de tocar/clicar na Semente e abrir uma interface ampliada semelhante à referência visual fornecida; dificuldade de localizar **Desligar** e **Encerrar sessão**.

A inspeção do código do instalador `arandu_a8_semente_adaptativa.py` indica causas prováveis e verificáveis:

- `DIAMETER = 118` e escala de repouso `0.66` produzem aproximadamente **78 px** visíveis.
- O centro ocupa `64 × 64 px` e o relógio usa fonte de `10 px`.
- Os microcontroles têm `25 × 25 px`, ainda menores na escala de repouso/aproximação; a área clicável efetiva pode ficar abaixo do recomendado para usuários com restrição motora.
- No `_presenceTick()`, `stage === 'HOVER'` muda automaticamente `_viewMode = 'rivers'`; ao sair, `d > 160` pode mudar novamente para `system`. Isso pode fazer controles desaparecerem enquanto se tenta acioná-los.
- `energy` chama `_panelMenu('quickSettings')`, que abre um menu ancorado na barra superior, **não um menu de energia junto à Semente**.
- A expansão atual desenha medalhões independentes sobre a área de trabalho, sem contrato explícito de fechamento por Escape, clique externo e transferência de foco ao aplicativo.

Esses pontos explicam a necessidade de redesenho. **Não foi feita inspeção remota da execução na máquina; o diagnóstico combina relato e leitura do código**, e os sintomas devem ser confirmados por teste.

## Decisão de experiência

A Semente terá **três escalas previsíveis**, não microbotões ilegíveis:

| Estado | Dimensão visual inicial proposta | Conteúdo |
| --- | --- | --- |
| Repouso | **~104–112 px** | Identidade reconhecível, hora clara e indicação discreta de energia; sem textos minúsculos |
| Aproximação | **~136–152 px** | Aro com ícones visíveis para Sistema, Busca, Opy e Energia; funções reveladas, **sem troca inesperada de modo** |
| Clique/tecla (Opy compacta) | **~320–380 px** de largura útil, ajustável ao monitor | Área ampliada com busca, Três Rios e funções do sistema, ícones + texto em área confortável |

Valores são hipótese de design e precisam ser validados com escala DPI, área de trabalho, baixa visão e coordenação motora. **Um ícone desenhado com 20 px não deve ter alvo interativo menor que 44 × 44 px** nos controles de uso frequente. É aceitável mostrar só os ícones principais no estado de proximidade e expandir rótulos na Opy.

### Regras de interação

1. **Aproximação revela; clique/tecla escolhe.** Nenhuma função de Sistema desaparece porque o ponteiro cruza o centro da Semente.
2. **Um clique no centro** abre ou recolhe uma área ampliada **não modal** (Opy compacta); teclado e atalho global terão ação equivalente.
3. **Rios são abas/áreas identificáveis** na Opy compacta. Selecionar Memória/Abertos/Frequentes não elimina as funções de Sistema e Energia.
4. **Os nós dos aplicativos** aparecem de maneira progressiva e podem ser acessados por teclado. Ao abrir ou focar um aplicativo, o painel recolhe e devolve o foco ao trabalho.
5. **Não bloquear o resto da área de trabalho:** somente os controles efetivos devem receber eventos de ponteiro. A interface não pode criar uma camada transparente que impeça usar as outras janelas.
6. **Esc ou clicar fora** recolhe a área ampliada; navegação pelos nós mantém o estado durante o uso e não deve ser fechada por aproximação/histerese.
7. **Semente móvel e multimonitor**: a Opy se posiciona no quadrante de espaço disponível, sem recortar controles; lembrar posição relativa e retornar ao monitor ativo se outro for desconectado.
8. **Se a expansão for fixada**, deixar visível um comando inequívoco **Recolher** e um atalho acessível. Nunca aprisionar o usuário num estado.
9. **Energia** é visível na primeira camada, com rótulo **Energia / Sessão** na Opy compacta; ações **Bloquear, Encerrar sessão, Reiniciar, Desligar e Cancelar**, sem executar diretamente no primeiro toque.
10. **Confirmação nativa** para encerrar sessão, reiniciar e desligar. Comando futuro pode usar `gnome-session-quit --logout`, `--reboot` ou `--power-off` **sem `--force` e sem `--no-prompt`**, após interação explícita e verificação de disponibilidade. A confirmação deve proteger documentos não salvos; não criar desligamento silencioso.
11. **Barra superior não será ocultada** antes de os equivalentes reais de som, rede, calendário, notificações, energia e indicadores de privacidade serem utilizáveis sem ela.
12. **Opy plena** poderá ampliar a busca por aplicativos e comandos de voz sob ativação explícita, sem escuta contínua. Nome guarani só com validação linguística.

## Motivo da decisão

**Diferenciação progressiva** requer crescimento da informação conforme uma intenção inteligível; ela não permite tornar funções pequenas e instáveis. **Reconciliação integradora** requer continuidade entre Sistema, Rios, Opy e trabalho já em curso. **Autonomia da pessoa** requer controle de foco, acesso rápido a saída, possibilidade de recolhimento e respeito às janelas que continuam abertas.

**Minimalismo não é miniaturização.** Uma Semente visualmente pequena pode resultar num ambiente mais confuso se o usuário não conseguir localizar comandos.

## Limites técnicos e segurança

- Não mudar, na primeira correção, o esquema GNOME/Mutter de janelas nem as operações de fechar/minimizar/maximizar.
- Não modificar Confluência, TOPBAR, A5 Rust, sessão GNOME original, serviço de sessão ou permissões.
- Fazer instalador experimental reversível com backup integral da A8 e restauração byte a byte.
- O protótipo deverá passar testes de estados, hitboxes, Escape, clique externo, foco, 2 monitores, cantos, desconexão e redução de movimento; testes automatizados **não substituem** uso humano.
- Se a integração gráfica falhar, manter `Super`, `Alt+Tab`, os atalhos GNOME e a barra superior.

## Critérios para aceitar a A8.1

- O usuário localiza **Energia / Sessão** sem procurar a barra superior e consegue escolher o comando correto com confirmação.
- A Semente permanece legível em repouso e os botões frequentes apresentam alvos de interação confortáveis.
- A Opy compacta abre no clique e fecha por Escape, clique externo, novo clique e seleção de aplicativo, sem bloquear outras janelas.
- Sistema continua acessível durante navegação por Memória, Abertos e Frequentes.
- A Semente não muda arbitrariamente de modo quando o ponteiro se desloca.
- A posição funciona nos dois monitores e em todas as bordas.
- Nenhum controle de IA ou microfone é ativado automaticamente.
- A versão A8 anterior é restaurável integralmente.

## Relação documental

- [Registro Filosófico Vivo](../filosofia/REGISTRO_FILOSOFICO_VIVO.md)
- [Espaços Vivos](../filosofia/ARANDU_ESPACOS_VIVOS.md)
- [Manifesto Visual](MANIFESTO_VISUAL_ARANDU.md)
- [Semente Adaptativa](../arquitetura/SEMENTE_ADAPTATIVA.md)

**Decisão-síntese:** a Semente deve ser menor que um painel de controle, não menor que a capacidade humana de perceber, compreender e agir.

## Implementação experimental preparada em 23/11/2026

A primeira implementação local da A8.1 foi gerada a partir do código real da A8: instalador reversível `arandu_a81_semente_navegavel.py` e componentes `extension.js`, `opy-placement.js`, `stylesheet.css`. A versão de ensaio traz uma Opy não modal ao clicar no centro, pesquisa textual de aplicativos, Rio Abertos via ponte Rust A5, microícones de Sistema maiores, Energia/Sessão com confirmação GNOME e recolhimento da sobreposição visual histórica via comando D-Bus explícito `seed:collapse`. Rios Memória e Frequentes ainda não têm dados reais no Rust. **Testes de contrato, sintaxe e geometria passaram; não há teste da interface executada na máquina do usuário.** Os arquivos gerados estão disponíveis na conversa como artefatos locais; **o código da A8.1 ainda não foi enviado ao repositório**. É proibido representar a A8.1 como instalada, validada no hardware ou mesclada ao ramo principal antes de evidência real.
