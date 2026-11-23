# ADR A8.6 — Rios, estados e Semente recolhida

**23/11/2026 · decisão de experiência, não implementada.**

## Três Rios são estados de atividade, não barras

- **Memória:** histórico local consentido de aplicativos e atividades usados, inclusive aplicações encerradas. Reabrir um aplicativo não garante restaurar um documento não salvo.
- **Abertos:** janelas reais atualmente existentes, distinguindo **visíveis** e **recolhidas/minimizadas**. Duas janelas do mesmo aplicativo são atividades distintas.
- **Frequentes:** aplicações com uso reiterado, conforme observação local consentida; não se restringe a janelas ainda abertas.

Os Rios são filtros e caminhos relacionais da Opy. Não exigem trilhas ou medalhões permanentemente desenhados no desktop.

## A própria Semente pode recolher-se

A pessoa poderá escolher **Guardar a Semente**, retirando completamente sua presença visual da área de trabalho, sem encerrar o serviço de navegação. A recuperação **não depende de descobrir um elemento oculto com o mouse**: haverá comando de teclado global configurável e gesto opcional de borda com alternativa assistiva. A Opy poderá continuar abrindo por teclado mesmo sem a Semente visível.

A Semente continua móvel e associada ao monitor escolhido. Quando o monitor for removido, a recuperação deve ocorrer no monitor restante. Evitar que a ocultação apague o atalho de resgate, prenda o foco ou torne a interface inacessível.

## Contrato histórico de atalhos confirmado no esquema da Confluência

| Atalho | Função histórica |
| --- | --- |
| Super + Alt + 1 | Memória |
| Super + Alt + 2 | Abertos |
| Super + Alt + 3 | Frequentes |
| Super + Alt + ← | Medalhão anterior |
| Super + Alt + → | Próximo medalhão |
| Super + Alt + Enter | Ativar medalhão |
| Super + Alt + 0 | Cancelar seleção |
| Super + Alt + Espaço | Acionador de varredura assistiva |
| Super + Shift + T | Abrir mapa ARANDU |
| Super + Alt + B | Recuperar barra GNOME (resgate técnico, não interface padrão) |
| Super + Alt + H | Alternar visibilidade da barra GNOME (legado) |

As combinações existem na extensão histórica, não estão automaticamente implementadas na nova Opy. A migração deverá preservar o significado, evitar captura duplicada entre extensões e oferecer configuração de conflitos. **O novo atalho de resgate da Semente deve ser explícito, configurável e validado**; não inferir que algum atalho anterior já o executa.

## Acessibilidade

Uma mesma intenção deverá poder vir de mouse, teclado, acionador, voz sob ativação, navegação assistiva e dispositivos futuros. A voz, sensores e ECA não são dependências das operações básicas. Estados visíveis e recolhidos precisam ser compreensíveis sem ícones puramente decorativos.

## Validação

Testar recuperação por atalho com 0, 1 e 2 monitores, em tela cheia, com GNOME overview, após erro ou reinício da extensão. Não habilitar o modo de tela completamente livre antes de garantir resgate operacional.

Relacionados: [Espaços Vivos](../filosofia/ARANDU_ESPACOS_VIVOS.md), [Semente Adaptativa](../arquitetura/SEMENTE_ADAPTATIVA.md), [Ambiente sem barra](ADR_A87_ARANDU_SEM_BARRAS.md).