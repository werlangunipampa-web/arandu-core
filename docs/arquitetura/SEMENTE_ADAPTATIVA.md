# ARANDU — Contrato de Semente Adaptativa

**Versão de trabalho 1.0 · 23/11/2026**  
Status: regras de produto; algumas funções implementadas, outras em pesquisa.

## Fundamento

A Semente não é um lançador fixo nem um medalhão central obrigatório. É um ponto de orientação móvel que revela complexidade de modo progressivo, à medida que a pessoa manifesta intenção. Os Três Rios **Memória**, **Abertos** e **Frequentes** expressam relações entre atividades; **Sistema** é uma camada utilitária no aro, não um quarto Rio.

O princípio de diferenciação progressiva corresponde ao aumento contextual de detalhe; a reconciliação integradora exige manter o vínculo visual e operacional entre elementos.

## Estados e responsabilidades

| Conceito de experiência A8 | Estado equivalente no núcleo Rust | Responsabilidade |
| --- | --- | --- |
| IDLE (repouso) | Collapsed | Semente compacta, sem fluxos |
| AWARE (percepção) | Aware | Sinal sutil de proximidade |
| NEAR (aproximação) | Near | Funções contextuais e crescimento |
| HOVER/EXPANDED | Expanded | Rios e nós temporariamente visíveis |
| PINNED (fixada) | Pinned | Rios permanecem abertos até comando explícito |

Os tamanhos experimentais no protótipo são cerca de **78 / 92 / 113 / 132 px**; não são normas de ergonomia. A tradução entre estado gráfico e estado Rust deve ser explícita e testada, sem assumir que cinco nomes equivalem a cinco implementações completas já integradas.

Referência histórica: [Semente da Confluência v0.2](../CONFLUENCE_SEED.md).

## Interação

- Aproximação de ponteiro: sem câmera, microfone ou IA.
- Hover: não recolher ao sair momentaneamente do círculo se a pessoa está percorrendo um Rio; respeitar atividade/histerese.
- Clique: fixar ou recolher a expansão sem ação inesperada sobre uma janela.
- Teclado e tecnologias assistivas: permitir mesmo alcance sem depender do mouse ou proximidade.
- Semente deslocável em qualquer monitor; posição relativa persistente; recalcular Rios e nós ao mover, mudar escala ou desconectar monitor.
- Memória e Frequentes devem vir de observações e consentimento, não de inferência a partir de um único snapshot.
- Abertos pode consumir IDs de aplicativos via ponte GNOME → Rust A5. **ID de aplicativo não identifica inequivocamente janela**.

## Camada Sistema na Semente

Microícones do aro: busca, rede/Bluetooth, áudio, calendário/avisos, acessibilidade, energia e Opy. A voz poderá abrir busca e programas **somente por ativação explícita do microfone**, com indicador claro, cancelamento e desligamento imediato; sem escuta permanente. A atribuição de termos guarani requer validação linguística e cultural, não tradução inventada.

A passagem da TOPBAR para Sistema exige **equivalência funcional**. Não ocultar barreira de recuperação, estados de rede, botões críticos ou indicadores de compartilhamento/câmera/microfone antes de substitutos testados, inclusive leitores de tela. O GNOME normal deve permanecer acessível.

## Espaços Vivos

As intenções de janela planejadas são **Recolher ao Rio**, **Trazer ao foco**, **Imersão**, **Confluência** e **Encerrar**. Inicialmente apenas observação e contrato; comandos mutantes exigem referência opaca específica por janela, ação humana explícita, desfazer onde possível e respeito a documentos não salvos. Ver [documento completo](../filosofia/ARANDU_ESPACOS_VIVOS.md).

## Divisão de responsabilidades

- **GNOME/Mutter:** janelas e monitores, controles básicos e indicadores críticos.
- **Extensão ARANDU:** interface, estados, apresentação e navegação local.
- **Confluência:** ponte de consulta/intenções locais com limites.
- **Rust:** estados universais, geometria, regras de navegação e identificadores de aplicativo válidos.
- **ECA/Tupã:** futuras camadas opcionais sob solicitação/consentimento e com limites; nunca necessárias para abrir apps ou retomar uma janela.

## Validação antes de promoção

1. Testes unitários Rust e JS para estados, bordas, duas telas, mudança de monitor e navegação.
2. Testes de integração no GNOME real sem sobrescrever a sessão de recuperação.
3. Falhas devem degradar com segurança; nenhuma ação automática sobre documentos ou janelas.
4. Backup e restauração byte a byte da extensão anterior; monitorar regressões de CPU e uso de memória.
5. Acessibilidade, percepção de avisos de privacidade e controles por teclado antes de ocultar a barra.

**Fontes internas:** [Fundação GNOME A2](../A2_FUNDACAO_GNOME_EVOLUCAO_ARANDU.md), [Semente v0.2](../CONFLUENCE_SEED.md), [Manifesto Visual](../design/MANIFESTO_VISUAL_ARANDU.md), [Registro Filosófico Vivo](../filosofia/REGISTRO_FILOSOFICO_VIVO.md).
