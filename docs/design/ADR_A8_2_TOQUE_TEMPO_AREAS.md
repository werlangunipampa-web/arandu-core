# ADR A8.2 — Intenção de toque: aplicativos, tempo e áreas de trabalho

**23/11/2026 · protótipo codificado, testes locais aprovados; ainda não validado na máquina GNOME real**

## Contexto

Após a revisão A8.1, o usuário estabeleceu duas expectativas adicionais: **tocar no centro da Semente deve mostrar aplicativos imediatamente** e deve haver uma maneira visível de **alternar entre as áreas de trabalho**. Em seguida, pediu que **tocar no horário** abrisse um painel semelhante à visão GNOME de calendário, agenda e notificações.

A Semente precisa expressar **previsibilidade por intenção**: um comando deve ter um resultado claro, sem exigir várias etapas escondidas ou trocar de modo por hover.

## Decisão A8.2

| Gesto | Intenção do usuário | Efeito do protótipo |
| --- | --- | --- |
| Tocar no centro | Encontrar um aplicativo | Abrir Opy na aba **Aplicativos**, com pesquisa e lista de apps instalados e ícones reais |
| Tocar diretamente na hora | Consultar o tempo | Abrir a Opy na página **Calendário**, com mês local, dias, anterior/próximo e Hoje |
| Tocar em uma área numerada | Retomar outro espaço de trabalho | Ativar a área **real** do gerenciador GNOME, por gesto explícito |
| Tocar em Visão geral | Entender a organização dos espaços | Abrir Overview do GNOME |
| Tocar em Agenda e notificações | Consultar dados do GNOME | Abrir o **menu nativo de data** do GNOME, com eventos/notificações reais |
| Tocar em Energia | Encerrar ou suspender a sessão por decisão humana | Preservar o painel A8.1 com confirmação GNOME (não forçada) |

A Opy permanece **não modal**, recolhível com Escape/clique externo, com posicionamento adaptado ao monitor da Semente, e não bloqueia outras janelas.

## Limites honestos do protótipo

- O calendário A8.2 gera somente **datas reais**, por `GLib.DateTime.new_now_local()`. **Não duplica compromissos e notificações** na interface ARANDU: para isso utiliza o menu dateMenu do GNOME, enquanto a barra permanece visível.
- A lista inicial é limitada para não sobrecarregar o painel; a busca e a visão completa GNOME permitem acesso ao catálogo restante.
- As áreas de trabalho ativadas vêm de `global.workspace_manager` e obedecem à configuração GNOME — não são áreas inventadas por monitor.
- A função de desligamento mantém diálogo de confirmação GNOME, sem `--force` ou `--no-prompt`.
- O núcleo Rust A5, a Confluência e a TOPBAR **não são alterados** por esta revisão.
- A interface ainda requer validação em uso real e nos dois monitores antes de considerar a A8.2 concluída.

## Continuidade ontológica

**Diferenciação progressiva:** a Semente em repouso é simples, mas abre diretamente a dimensão desejada com um toque. O usuário decide aprofundar o detalhe; o sistema não impõe uma expansão automática.

**Reconciliação integradora:** Aplicativos, Abertos, Memória, Frequentes, Sistema, Tempo e Áreas são diferentes perspectivas sobre o mesmo contexto de trabalho. A Opy reúne essas perspectivas sem confundi-las com os Três Rios.

**Territorialidade e agência:** a Semente móvel preserva seu vínculo ao monitor; as áreas de trabalho ajudam a pessoa a decidir o que fica em foco. A troca de área só acontece por ação humana.

**Responsabilidade técnica:** não ocultar a barra superior antes de ter implementado calendário, agenda, notificações, rede, áudio, privacidade e energia independentemente dos menus do GNOME.

## Código e validação

Artefato experimental da conversa: `arandu_a82_toque_tempo_espacos.py`, criado a partir da A8.1 com instalador reversível e hashes. Testes locais: **7 autotestes**, sintaxe JavaScript com Node, geometria do painel e restauração byte a byte em instalação sintética. **Nenhuma instalação na máquina do usuário foi realizada por esta decisão.** O código experimental não foi enviado a este repositório; este ADR registra as regras e o estado técnico, não faz declaração de entrega em produção.

## Roteiro futuro

1. Validar leitura e alvos de clique do centro e do relógio no GNOME 46 X11.
2. Melhorar apresentação da lista de apps (ícones, busca, acessibilidade, paginação).
3. Integrar agenda e notificações reais à Opy por contratos seguros, sem repassar dados sensíveis ao Rust.
4. Validar alternância e descrição de áreas de trabalho com dois monitores.
5. Migrar os demais controles e indicadores de privacidade, e **somente depois** oferecer ocultação reversível da barra superior.

Veja também [Registro Filosófico Vivo](../filosofia/REGISTRO_FILOSOFICO_VIVO.md), [Semente Adaptativa](../arquitetura/SEMENTE_ADAPTATIVA.md), [Espaços Vivos](../filosofia/ARANDU_ESPACOS_VIVOS.md) e [ADR A8.1](ADR_A8_1_SEMENTE_NAVEGAVEL.md).
