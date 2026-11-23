# ARANDU A8 — Semente Adaptativa e Espaços Vivos

**Documento de arquitetura e experiência · 23/11/2026 · versão 0.1**

> A atividade não deixa de existir quando deixa de ocupar o espaço. O usuário organiza presenças, não apenas janelas.

## 1. Decisão de design

O ARANDU continuará usando GNOME Shell 46 e Mutter como fundação de janelas. A nova linguagem visual, porém, passa a privilegiar **atividade, contexto, presença e relação**, em lugar de transportar diretamente os paradigmas de três controles tradicionais para outro desenho.

A interface nasce de sua ontologia de Semente, Três Rios, Confluência, Opy e aprendizagem significativa. Essa é uma **linguagem original do projeto**, em diálogo responsável com perspectivas e referências guarani, sem reivindicar ser tradução de uma cosmologia ou grafismo tradicional específico.

## 2. A Semente Adaptativa

A Semente pertence à área de trabalho, não ao centro da tela. É posicionável em qualquer monitor e persiste sua posição relativa à tela. Reage à distância do ponteiro sem câmera, microfone ou IA em segundo plano.

| Estado | Reação pretendida | Principal informação |
| --- | --- | --- |
| **IDLE** | Cerca de 78 px visíveis | Somente o núcleo e o relógio |
| **AWARE** | Aproximação suave, ~92 px | Microícones de Sistema no próprio círculo |
| **NEAR** | Expansão para ~113 px | Sistema com áreas de interação distinguíveis |
| **HOVER** | Expansão para ~132 px | Memória, Abertos e Frequentes dentro do círculo |
| **PINNED** | Permanece expandida por clique | Três Rios disponíveis até recolhimento explícito |

Os valores são **dimensionamentos de protótipo**, não resultados de teste de usabilidade. A implementação A8 consulta somente as coordenadas do ponteiro a cada 200 ms, calcula distância e atualiza o desenho **apenas quando há mudança de estado**. Em `enable-animations=false`, não aplica duração de animação. Não registra nem transmite coordenadas.

Os microícones internos do protótipo A8 encaminham acesso a **Busca, Rede/Bluetooth/Som, Calendário/Avisos, Acessibilidade, Energia e Opy**. Os menus GNOME ainda são os responsáveis por controles que não migraram. O botão Opy abre o portal A3, quando disponível; não representa a Opy definitiva. Nenhum controle inicia escuta de voz.

No centro, as três ações dos Rios aparecem no próprio círculo. Ao escolher **Abertos**, os medalhões dos aplicativos são populados pela ponte Confluência → núcleo Rust já validada na A5, sob demanda. Memória e Frequentes continuam dependentes da maturação de seus respectivos contratos de dados.

## 3. Novo paradigma de janelas: *Espaços Vivos*

### Ações propostas

| Intenção | O que a pessoa percebe | Efeito técnico previsto (posterior) |
| --- | --- | --- |
| **Recolher ao Rio** | A atividade sai do primeiro plano mas permanece no Rio Abertos | Minimizar a janela; preservá-la recuperável |
| **Trazer ao foco** | Retomar uma atividade sem procurar por bordas ou Alt-Tab | Desminimizar e ativar janela |
| **Imersão** | Dedicar todo o espaço útil do monitor a uma atividade | Maximizar; *tela cheia real* somente em opção separada |
| **Confluência** | Aproximar duas atividades em uma composição de trabalho | Propor layout lado a lado, com consentimento |
| **Encerrar** | Concluir deliberadamente a presença da janela | Solicitar fechamento ao aplicativo; respeitar diálogos de salvamento |

**Não renomear somente os controles antigos:** cada intenção deve ser encontrável na atividade, na Semente e nos Rios. A metáfora serve à compreensão, não deve esconder o que a ação faz realmente.

### Exemplo

A pessoa está redigindo texto e consultando um artigo. Ela toca no medalhão do artigo e escolhe **Recolher ao Rio**: o artigo deixa de sobrepor o documento, mas permanece visível como atividade em **Abertos**. Ao escolher **Confluência** entre editor e navegador, o sistema sugere uma organização lado a lado no monitor escolhido. Ao voltar à redação, **Imersão** dá ao texto o espaço útil disponível, mantendo a Semente acessível. **Encerrar** jamais é disparado automaticamente pela ECA.

### Princípio de segurança

Uma janela não equivale necessariamente a um aplicativo. Várias janelas podem pertencer ao mesmo aplicativo. O **núcleo Rust A5 hoje recebe IDs de aplicativos, não handles inequívocos de cada janela**; portanto, ainda não dispõe de informação suficiente para aplicar estas ações corretamente. O contrato futuro deve criar *referências opacas temporárias a janelas*, emitidas e resolvidas localmente pela extensão GNOME, sem expor nomes de documentos, dados pessoais nem executar comandos arbitrários.

O Mutter dispõe de operações como minimizar, desminimizar, ativar, maximizar e solicitar fechamento; a implementação concreta deverá ter autorização por gesto explícito, checagem de janela elegível, tratamento de erros e restauração do estado anterior quando possível.

## 4. Opy e o comando por linguagem

A Opy será o **espaço ampliado de orientação e procura** quando a Semente compacta não for suficiente. Deve aceitar teclado, mouse, navegação assistiva e futuramente voz por ativação deliberada. Opy não substitui os Rios; é outra escala da mesma navegação.

Para buscas por voz: microfone começa **desligado** e somente é ativado por uma ação explícita; há indicador perceptível e encerramento imediato pelo usuário. Nomes guarani para programas ou comandos não serão inventados: deverão ser pesquisados e validados com interlocutores qualificados.

## 5. Barra superior: migração por equivalência funcional

A barra do GNOME permanece **visível nesta versão**. Para removê-la depois, a Semente deverá oferecer equivalentes funcionais, e não apenas links que abrem o menu antigo ainda ancorado no topo:

- volume, dispositivos de áudio e estado da conexão;
- rede Wi-Fi e Bluetooth com estado/erro visíveis;
- relógio, calendário, notificações e estado de atenção;
- ajustes, acessibilidade, sessão, energia e desligamento;
- indicadores não ocultáveis silenciosamente: uso de microfone, câmera, tela compartilhada e permissões.

Somente depois de ensaios de acessibilidade, retorno por teclado, recuperação em falha e teste nos dois monitores será disponibilizada a opção de ocultar a barra **exclusivamente na sessão ARANDU**. A sessão GNOME normal é o caminho de recuperação.

## 6. Arquitetura em fases

**A8.0 — Semente Adaptativa** (protótipo instalável): estados de aproximação; microícones internos de Sistema; Rios internos; posicionamento já existente; Rust para Abertos; backup e restauração A7. Sem ações mutantes sobre janelas.

**A8.1 — Janelas observáveis**: coletar referências opacas de cada janela do usuário, distinguir múltiplas janelas do mesmo app e monitorar estados sem polling intenso ou títulos sensíveis. Nenhuma ação automática.

**A8.2 — Recolher e Retomar**: operações reversíveis sob clique/teclado, com visualização correta no Rio Abertos, sem esconder documentos da navegação assistiva.

**A8.3 — Imersão e Confluência**: ocupação do espaço útil, layouts assistidos, regras multimonitor, desfazer e preservação do tamanho anterior.

**A8.4 — Opy e entrada multimodal**: busca por texto/teclado, voz sob ativação explícita, navegação por varredura ou dispositivo assistivo.

**A9 — Sistema sem barra**: integrar controles de sistema reais, testar indicadores críticos, só então oferecer ocultação reversível da TOPBAR.

## 7. Regras de validação

1. Nada de executar fechamento, minimização ou reorganização de janelas sem gesto do usuário.
2. Nenhuma alteração de identidade do ARANDU nem instalação de software em segundo plano sem validação.
3. Medir fluidez e custo no GNOME 46 com duas telas, incluindo escala distinta, extremos e troca de ordem.
4. Avaliar usuários que usam mouse, teclado, leitor de tela e navegação por acionador.
5. Recuperar A7 exatamente, byte a byte, se a A8 falhar ou não agradar.
6. Nunca substituir silenciosamente o significado de **fechar** por **minimizar**, nem vice-versa.
7. A Opy, a ECA e Tupã permanecem independentes do caminho crítico de abrir aplicativos e recuperar janelas.

**Síntese:** A Semente é pequena porque respeita o espaço. Cresce porque reconhece intenção. Os Rios preservam a continuidade. Espaços Vivos dão às atividades uma relação clara com o lugar que ocupam — sem retirar do usuário a última palavra.
