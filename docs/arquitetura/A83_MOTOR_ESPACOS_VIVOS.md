# ARANDU A8.3 — Motor Rust de Espaços Vivos e Rios Discretos

**Estado: código experimental no ramo A8.3. Não substitui o gerenciador de janelas e não executa comandos sobre janelas reais.**

## Princípio ontológico

O usuário organiza **atividades**, não apenas superfícies. A atividade minimizada mantém sua presença no Rio Abertos; encerrar é diferente de recolher. Os Rios continuam estruturas semânticas mesmo quando a tela não desenha faixas. Opy e Semente revelam essas estruturas por intenção.

## Entrega em Rust

O módulo `crates/arandu-core/src/espacos_vivos.rs` exporta:

- `WindowRef`: identificador opaco por janela, vinculado a uma geração positiva e sem títulos ou arquivos;
- `WindowSnapshot`: conjunto limitado a 128 janelas, rejeita referências duplicadas e gerações incoerentes;
- `WindowObservation`: estado e capacidades mínimos por janela, com monitor/área;
- `WindowIntent`: Recolher ao Rio, Trazer ao foco, Imersão, Confluência e Encerrar;
- `plan_window_action`: plano declarativo, **nunca executado** diretamente pelo Rust;
- `RiverPresentation::Discreet`: ausência de trilhas persistentes como opção padrão.

Os testes verificam referências obsoletas, duplicação, múltiplas janelas do mesmo app, ausência de capacidade, monitores e áreas incompatíveis, confirmação de Encerrar e ausência de efeitos reais na preparação do plano.

O CLI fornece `cargo run --locked -p arandu-cli -- --espacos-vivos-demo`, que simula as cinco intenções com dados sintéticos **sem consultar GNOME**.

## Adaptação GNOME planejada

A primeira ligação real exigirá:

1. Enumerar janelas elegíveis sem transferir títulos de documentos.
2. Emitir referências **opacas e efêmeras**, específicas por janela, com geração; nunca confundir IDs de aplicativos da ponte A5 com IDs de janela.
3. A cada gesto, obter nova observação e solicitar ao Rust um plano.
4. Conferir novamente existência, geração, capacidades e foco no adaptador antes de executar.
5. Traduzir os efeitos ao Mutter com erros controlados; `RequestClose` deve preservar diálogos de salvamento, e `TileSideBySide` terá desfazer.
6. Disponibilizar ações pela Opy/Abertos e por teclado; não retirar atalhos convencionais antes da equivalência assistiva.

**Nenhum plano pode ser executado por evento ECA/IA, observação passiva ou voz sem confirmação.** O enum `ConfirmedGesture` é apenas um tipo do contrato; prova de gesto precisa ocorrer na camada de interface, não pode ser inferida pelo Rust.

## O que acontece com as faixas antigas?

A extensão Confluência histórica faz duas coisas: desenha as faixas de rios e fornece `Snapshot` via D-Bus. Desabilitá-la eliminaria **também a fonte dos dados A5**.

O patch experimental separado `arandu_a83_rios_discretos.py` aplica uma condição na visibilidade do ator raiz **somente se `ARANDU_SESSION=1`**. A Confluência continua executando serviço e consultas, enquanto o GNOME comum mantém sua interface anterior. Este script:

- exige GNOME Shell 46/X11 e usuário normal (sem sudo);
- verifica padrão exato do código instalado e recusa versões desconhecidas;
- salva cópia integral de `extension.js`, SHA-256 anterior e posterior;
- restaura byte a byte somente quando o arquivo ainda corresponde ao patch;
- **não toca na TOPBAR, Opy A8.2, sessão GNOME nem Rust A5**.

O patch fica em arquivo para download na conversa; **não está incorporado à instalação padrão nem foi executado no computador do usuário**. Se a verificação não reconhecer a versão da Confluência, interromper e rever o código instalado, sem forçar substituições.

## Critérios de validação

- Compilar e testar `arandu-core` e CLI.
- Validar na máquina GNOME 46 que o patch mantém `org.arandulab.Tape.Windows1.Snapshot` funcional e oculta faixas históricas na sessão ARANDU.
- Entrar no GNOME original e verificar que nada foi escondido ali.
- Verificar Opy Aplicativos, Hora, Áreas de trabalho, Energia e Abertos/Rust após patch.
- Confirmar restauração exata do arquivo antigo e da aparência dos Rios.
- Implementar adaptador mutante somente após validar o contrato por janela, com backup e recuperação.

**A barra superior permanece visível** até rede, áudio, notificações, calendário, energia, privacidade e acessibilidade terem equivalentes próprios confiáveis.
