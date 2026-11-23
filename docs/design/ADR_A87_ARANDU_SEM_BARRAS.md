# ADR A8.7 — O ARANDU sem barras

**23/11/2026 · decisão de produto e arquitetura; implementação pendente, sem alterar o ambiente instalado.**

## Ruptura de paradigma

O ARANDU abandona **barra superior**, dock ou barra de tarefas como organização canônica. Não se trata de uma barra escondida que reaparece com hover: **ela não pertence à linguagem do ambiente**. Na superfície em repouso, apenas o trabalho e, se desejado, a Semente; o usuário também pode guardar a própria Semente no Rio para uma tela inteiramente livre.

GNOME/Mutter permanece como base técnica madura durante a transição. Remover barras **da experiência visível** não exige remover o subsistema interno do GNOME; o compositor pode manter mecanismos técnicos invisíveis para recuperação. A sessão GNOME original segue intacta.

## Onde vão as funções da barra?

| Função anteriormente na barra | Ponto natural ARANDU |
| --- | --- |
| Hora e data | Toque no horário da Semente; Opy Tempo/Calendário |
| Notificações e eventos | Opy Tempo/Avisos, com alerta não intrusivo e histórico consultável |
| Wi-Fi, rede, Bluetooth | Opy Sistema; estado e controles reais |
| Volume, saída/entrada de áudio | Opy Sistema; comando rápido, ajuste completo por contexto |
| Câmera, microfone, tela compartilhada | Indicadores de privacidade inequívocos e persistentes durante uso, visíveis mesmo com Semente guardada |
| Aplicativos instalados | Toque no centro da Semente ou atalho → Opy grade grande |
| Janelas ativas e recolhidas | Rio Abertos; toque/atalho para retomar uma janela exata |
| Aplicativos anteriores e frequentes | Rios Memória e Frequentes |
| Áreas de trabalho | Seletor visual na Opy e atalhos |
| Bloquear, suspender, encerrar sessão, reiniciar, desligar | Entrada Energia sempre encontrável por Semente/Opy e atalho, com confirmação GNOME apropriada |
| Configurações e acessibilidade | Opy Sistema; configurações nativas disponíveis enquanto migramos controles |

### Indicadores de privacidade: exceção necessária ao 'vazio absoluto'

A ausência de barras **não** autoriza esconder uso ativo do microfone, câmera, gravação ou compartilhamento de tela. Um elemento de status mínimo e acessível deve permanecer perceptível enquanto há atividade sensível, inclusive com a Semente recolhida e em múltiplos monitores. Sua posição não constitui uma barra tradicional.

## Estrutura de experiência

**Tela livre:** fundo e janelas, nenhuma barra, Semente pequena opcional. **Semente:** movimento livre entre monitores; aproximação revela comandos úteis sem botões minúsculos. **Opy:** grade de aplicativos, Rios, áreas, tempo, sistema, energia e acessibilidade. **Espaços Vivos:** arrastar janela para zona contextual de Recolher, Retomar por Rio, Imersão e Confluência sob intenção. **Teclado:** atalhos históricos dos Rios preservados, mais invocação e resgate inequívocos da Semente.

Não substituir uma barra horizontal por outra barra circular abarrotada de botões. Diferenciação progressiva: poucos sinais em repouso, navegação ampliada apenas na Opy quando solicitada.

## Plano de implantação sem regressão

**Etapa 1 — Fundação sem barra (protótipo de sessão ARANDU):** ocultar visualmente o painel GNOME apenas na sessão ARANDU, preservar infraestrutura nativa e manter atalho técnico de resgate enquanto a paridade não está validada. Verificar em uso real relógio/agenda, controle de energia, rede, áudio, acessibilidade, notificações e visibilidade de indicadores críticos.

**Etapa 2 — Independência funcional:** implementar equivalentes ARANDU para todas as funções necessárias, sem links que apenas abram um menu escondido ancorado na barra. Alertas, indicação de estados e energia devem funcionar com barra ausente.

**Etapa 3 — Padrão definitivo:** a sessão ARANDU inicia sem barras e nunca precisa revelar uma barra para operar. O atalho de recuperação permanece como salvaguarda técnica opcional, não como recurso cotidiano. GNOME padrão mantém configuração de origem.

**Não ativar a etapa 3** se o usuário não conseguir encerrar sessão, controlar som/rede, perceber avisos de privacidade ou recuperar a interface por teclado.

## Responsabilidades técnicas

- **Rust:** ontologia, Rios, navegação, estados de Semente, intenções de janelas, regras de alternância. Não depender de GNOME.
- **Adaptador GNOME Shell:** apresentar Opy, executar ações permitidas e integrar serviços reais. A barra de GNOME não deve determinar o desenho ARANDU.
- **Mutter e serviços do sistema:** monitores, janelas, som, rede, sessão e permissões; não reinventar componentes maduros sem necessidade.
- **ECA/Tupã:** opcionais e não exigidos para operações críticas. Sem controle automático não consentido.

## Validação e recuperação

Testes com dois monitores, diferentes escalas, atividade com documentos abertos, travamento da extensão, GNOME overview e ativação do atalho de retorno. Teclas dos Rios `Super+Alt+1/2/3` têm sua função mantida ou migrada sem conflitos. Nenhum mecanismo de autoescuta ou sensor é necessário.

## Ontologia ARANDU

Os Rios não são decoração permanente, e a Semente não é uma barra alternativa. A área de trabalho é um território aberto de atividades, em que a navegação nasce da intenção. A ausência de barra é consequência da filosofia de continuidade, territorialidade escolhida e aprendizagem significativa — não um truque de tema visual.

Relacionados: [Registro Filosófico Vivo](../filosofia/REGISTRO_FILOSOFICO_VIVO.md), [Semente recolhida e atalhos](ADR_A86_RIOS_SEMENTE_RECOLHIDA_ATALHOS.md), [Janelas que fluem](ADR_A85_JANELA_FLUI_PARA_SEMENTE.md).