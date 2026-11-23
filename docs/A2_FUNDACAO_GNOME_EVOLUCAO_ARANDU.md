# ARANDU A2 — Fundação GNOME e autonomia progressiva

**Contrato arquitetural em desenvolvimento.** Este documento preserva a continuidade da Ontologia ARANDU; não substitui seus textos fundadores.

## Tese

O ARANDU começa usando GNOME Shell, Mutter e GTK como infraestrutura operacional, mas mantém como fundamento seu próprio código, ontologia, núcleo Rust, Semente, Confluência, Três Rios, aprendizagem significativa, acessibilidade e identidade visual.

A infraestrutura GNOME é uma dependência substituível. A autonomia futura será atingida **por módulos testados e úteis**, sem abandonar bibliotecas maduras simplesmente por princípio.

## Camadas

1. **Ontologia ARANDU:** governança, propósito, diferenciação progressiva, reconciliação integradora, aprendizagem significativa, respeito à agência do usuário e responsabilidade energética.
2. **Núcleo Rust:** navegação, Semente, estados dos Rios, memória local consentida, geometria e lógica reutilizável independentemente do desktop.
3. **Adaptadores:** tradução entre GNOME e as intenções do Rust; comandos humanos independentes do dispositivo.
4. **Interface ARANDU:** Confluência, TOPBAR, Opy e identidade cultural própria.
5. **GNOME Shell/Mutter/GTK:** janelas, dois monitores, notificações, áreas de trabalho, áudio, rede, Bluetooth, acessibilidade e configurações.
6. **ECA e Tupã:** gradualmente, por solicitação, com limites e autorização. O desktop permanece plenamente funcional desligando-se ambos.

## Código entregue nesta fase

O repositório histórico ARANDU-TAPE já possui, quando a extensão Confluência está ativa, um serviço local D-Bus de consulta Snapshot no GNOME. A primeira ponte lê uma única amostra sob demanda e entrega **somente identificadores de aplicativos** ao núcleo Rust. Títulos e documentos não são transferidos para o Rust. A interface atual não informa o foco de maneira confiável, portanto não o inventamos.

A nova entrada de CLI é --gnome-apps-stdin. A consulta local é scripts/arandu_gnome_snapshot.py. O primeiro rio conectado é **ABERTOS**. Não há coleta contínua, serviço persistente, inferência de uso frequente nem modificação de janelas.

## Acessibilidade desde o início

O perfil de entrada em Rust já admite teclado, mouse, touchpad, voz, tons, movimento da cabeça, acionadores e periféricos adaptados. A mesma intenção (próximo, anterior, selecionar rio, confirmar, cancelar) deve ser acessível por modalidades diferentes.

Esta fase só contém **mapeamentos de sinais simulados ou normalizados**. Câmera, microfone, rastreamento, reconhecimento, dwell, calibração e integração de hardware não estão implementados aqui. Adaptações futuras exigirão consentimento específico, calibração, limiares, debounce, interrupção acessível, confirmação de ações, proteção de dados e validação com pessoas usuárias antes de declarações de acessibilidade funcional.

## Leveza, energia e autoevolução

- GNOME continua responsável pelas tarefas básicas. ECA/IA não ficam no caminho crítico de abertura de aplicativos.
- Sem acesso contínuo a microfone, câmera, títulos de janelas ou atividade.
- Consulta contextual local e explícita, apenas quando necessária.
- Evitar wakeups e polling desnecessário; medir latência, CPU, RAM e consumo real antes de afirmar ganhos.
- Evolução proposta pela ECA somente com testes, backup, autorização humana, revisão e recuperação. Nenhuma alteração autônoma na sessão.
- Monitores e escalas são responsabilidade da plataforma; o núcleo é independente de resolução.

## Patrimônio preservado

- A versão ARANDU-DESKTOP de login da outra máquina, com Sway/GTK4 e otimizações testadas, continua um protótipo funcional que NÃO deve ser removido. O código mais recente da máquina anterior pode não estar todo no GitHub; precisamos comparar versões antes de migrar.
- ARANDU-TAPE contém a Confluência e TOPBAR do GNOME.
- arandu-core contém o Rust e os contratos de navegação e conhecimento.
- GNOME estável permanece como sessão de recuperação na máquina nova.

## Roteiro

**A2** ponte read-only GNOME → Rust e contratos assistivos. **A3** interface ARANDU sobre os serviços GNOME, preservando look-and-feel e monitor duplo. **A4** ativação contextual ECA/Tupã sob demanda. **A5+** independência progressiva por substituição de módulos demonstravelmente melhores, nunca por reconstrução indiscriminada.

O GNOME é alicerce operacional inicial; a ontologia e o código do ARANDU definem o sentido e a direção do ambiente.
