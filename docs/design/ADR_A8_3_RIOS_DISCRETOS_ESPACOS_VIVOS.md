# ADR A8.3 — Rios discretos e ações de janela como relações

**Data:** 23/11/2026  
**Status:** proposta de experiência, **não implementada**; exige prototipação e validação humana.

## Situação observada

As capturas de uso real do ARANDU sobre GNOME 46 exibem duas apresentações concorrentes: a visão GNOME de aplicativos (grade ampla, busca, áreas de trabalho, dock) e a antiga Confluência de faixas espessas, losangos repetidos, medalhões pequenos e rios extensos sobre o papel de parede. A segunda representa corretamente o conceito de Três Rios, mas prejudica **leveza, reconhecimento dos ícones e espaço útil**.

A nova Semente Adaptativa e a Opy já demonstraram outra via: a navegação pode ser contextual e progressiva, sem desenho permanente das conexões.

## Decisão visual: os Rios são funções, não faixas

1. **Repouso:** somente Semente pequena, legível, deslocável para qualquer ponto de qualquer monitor, com hora e indicação inequívoca de acesso.
2. **Toque no centro:** abre Opy ampla com **grade grande de aplicativos e pesquisa**, como referência à boa usabilidade GNOME, com identidade visual autoral ARANDU e acesso a áreas de trabalho.
3. **Três Rios como perspectivas/filtros** na Opy:
   - **Memória:** recentes, quando realmente observados e consentidos;
   - **Abertos:** janelas/atividades realmente em andamento; considerar múltiplas janelas do mesmo app;
   - **Frequentes:** baseados em métricas locais observadas, não inferidos por um snapshot.
4. **Visualização contextual de percurso:** linhas/trilhas muito finas e temporárias só quando a pessoa solicita observar relações. Não produzir faixas largas de losangos sobre toda a área de trabalho.
5. **Medalhões dos aplicativos:** ícones reais, rótulos acessíveis, sem duplicação decorativa nem perda de alvo tátil.
6. **Sistema e energia:** camadas utilitárias acessíveis diretamente na Semente; não são um quarto Rio.
7. **Na Opy e nos Rios não há camada modal bloqueando outros aplicativos**; Escape e clique externo devem recolher; ações do usuário dão prioridade ao foco adequado.

**Princípio:** invisível não significa indisponível. A metáfora dos Rios permanece como **estrutura semântica**, não como ornamentação constante.

## Mudança de paradigma: Espaços Vivos

Em vez de espelhar três botões tradicionais no topo de cada janela, tornar ações sobre *atividades* disponíveis por um **controle contextual leve**:

| Intenção ARANDU | Efeito técnico inicial previsto | Cuidados |
| --- | --- | --- |
| **Recolher ao Rio** | Minimizar a janela, mantendo item em Abertos | Recuperar por clique/teclado; não equivale a salvar documento |
| **Trazer ao foco** | Desminimizar/ativar uma janela específica | Preservar foco correto e ordem de janelas |
| **Imersão** | Maximizar na área útil do monitor | Não confundir com fullscreen exclusivo; manter Semente acessível |
| **Confluência** | Propor duas ou mais janelas lado a lado | Consentimento e desfazer layout anterior |
| **Encerrar** | Pedir fechamento à aplicação | Honrar diálogo de documentos não salvos e cancelamentos |

**Descoberta da ação:** ao selecionar a atividade em Abertos, por menu contextual junto à própria janela (se disponível) ou por atalho de teclado. Não exigir arrastar uma janela fisicamente até a Semente, especialmente em múltiplos monitores ou mobilidade reduzida.

**Teclas convencionais GNOME continuam disponíveis** (Alt+Tab, Super, atalhos de maximização e fechar), e botões tradicionais não são removidos antes de testar as alternativas. O término de processo e o fechamento forçado não são ações automáticas da Semente.

### Rascunho de estados

- **Semente**: presença compacta; a pessoa não está obrigada a navegar.
- **Opy / Aplicativos**: grade grande, busca, áreas e energia/tempo claramente acessíveis.
- **Opy / Abertos**: atividades reais com seletor opcional de ação: *Recolher, Focar, Imersão, Confluência, Encerrar*.
- **Janela em foco**: um gesto intencional revela o pequeno controle de ações, sem adornar permanentemente cada janela.
- **Após ação**: retirar a interface contextual e devolver controle à aplicação; a atividade persiste ou é encerrada conforme o efeito explícito.

## Governança tecnológica

- Núcleo Rust define conceitos de atividades, rios, intenção e geometria independentes de desktop.
- GNOME/Mutter detém a autoridade de janelas, monitores, foco e confirmação de sessão; integração via contratos permitidos.
- A ponte A5 só identifica apps, **não janelas individualmente**. Alterações em janelas exigem referências opacas efêmeras específicas por janela, emitidas pelo adaptador GNOME e validadas ao agir; sem comandos arbitrários e sem expor títulos pessoais.
- Sem telemetria contínua, rastreamento de cursor persistido ou necessidade de ECA/Tupã para operar.
- A acessibilidade por mouse, teclado e tecnologias assistivas é indispensável; não depender exclusivamente de hover ou de animação.
- A barra GNOME **não desaparece** antes da migração funcional completa de hora/agenda, notificações, áudio, redes, sessão/energia e indicadores de privacidade.
- Design cultural: a organização pode dialogar com valores de relações e territorialidade, mas grafismos guarani requerem procedência e participação/validação adequadas. Não usar "rio invisível" como interpretação atribuída a um povo específico.

## Experimentos verificáveis antes de implementação principal

1. Protótipo comparativo A/B: Rios persistentes vs. filtragem contextual sem trilhas, com tarefas reais de abrir, retomar, procurar e organizar janelas.
2. Taxas de sucesso para localizar aplicativo, alternar área, **Recolher** vs **Encerrar**, ativar energia e retomar após minimização.
3. Dois monitores, bordas, escalas, movimento reduzido, teclado, leitores de tela, interação assistiva, GNOME fallback.
4. Medição de CPU/RAM e latência da Semente em repouso e Opy expandida.
5. Backup e restauração integral da versão anterior, sem alterar sessão GNOME original.

## Continuidade filosófica

**Diferenciação progressiva:** relações são reveladas conforme a intenção; nenhum ornamento permanente substitui a função.  
**Reconciliação integradora:** ações da janela, estado de abertura, memória e frequência permanecem relacionados na Opy.  
**Autonomia:** a atividade é do usuário; recolher não encerra, encerrar não é recolher, imersão não implica tela cheia exclusiva.  
**Territorialidade livre:** Semente e atividades adaptam-se à composição dos monitores, sem impor centro físico.

Documentos relacionados: [Manifesto Visual](MANIFESTO_VISUAL_ARANDU.md), [Espaços Vivos](../filosofia/ARANDU_ESPACOS_VIVOS.md), [Registro Filosófico Vivo](../filosofia/REGISTRO_FILOSOFICO_VIVO.md), [ADR A8.2](ADR_A8_2_TOQUE_TEMPO_AREAS.md).
