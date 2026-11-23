# Plano Diretor ARANDU — Substituição Funcional do GNOME (69 itens)

**Referência:** GNOME Shell 46 / Linux Mint 22.3, observada no desenvolvimento ARANDU em 23/11/2026.  
**Estado:** matriz de planejamento; **não significa que estes 69 itens estejam instalados na máquina nem tenham sido implementados**. O inventário de programas real requer coleta local, consentida, só de leitura.

## Norte filosófico

O GNOME deixa de determinar a experiência visível: sem barra superior obrigatória, sem dock permanente, a Semente pode recolher-se e os Rios tornam-se perspectivas acionadas por gesto ou teclado. O Rust governa ontologia, relações, navegação, intenções, estados e política. A integração GNOME/Mutter só apresenta e executa ações autorizadas enquanto for necessária. O usuário escolhe o grau de exposição da interface.

**Não substituir tudo indiscriminadamente:** NetworkManager, D-Bus, PipeWire, BlueZ, logind, polkit, portals, AT-SPI, Orca, libinput e os formatos/padrões interoperáveis do Linux são ativos de infraestrutura. Criar outra UI em Rust não obriga a reescrever um daemon funcional em Rust.

**Ganhos de desempenho são hipóteses a testar**, e não promessas automáticas. Comparar A8.4, GNOME original e ARANDU novo em condições idênticas.

## Legenda

- **P0** = dependência crítica para operar sem barra, acessibilidade, segurança ou recuperação;
- **P1** = experiência essencial;
- **P2** = capacidade ampliada;
- **P3/P4** = opcional, de maior custo ou de infraestrutura estratégica;
- **A8.8** Três Rios em 3 colunas, Semente recolhível e atalhos; **A9.1** Sistema sem barra; **A9.2–A9.3** Espaços Vivos; **A10** Ysyry; **A11** configurações críticas; **A12** arquivos e interoperabilidade; **A13** aplicativos e diagnósticos; **A14+** aplicações próprias facultativas; **A15+** autonomia estrutural.

## Inventário de substituição (69 funções e componentes)

### Navegação

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 01 | Visão Atividades e grade | Opy: grade grande de Aplicativos | A8.8 | P1 |
| 02 | Busca de apps | Opy: busca unificada em Rust | A8.8 | P1 |
| 03 | Dash, dock e favoritos | Rio Frequentes e fixados por escolha | A8.8 | P1 |
| 04 | Alt+Tab | Rio Abertos por janela | A8.8 | P1 |
| 05 | Atividades recentes | Rio Memória local consentida | A8.8 | P1 |
| 06 | Frequência dos aplicativos | Rio Frequentes sob demanda | A8.8 | P1 |
| 07 | Mapa convencional | Opy: três Rios em colunas | A8.8 | P1 |
| 08 | Ícone persistente de acesso | Semente guardável e resgatável | A8.8 | P0 |
| 09 | Atalhos de sessão | Preservar Super+Alt+1/2/3, setas e Enter | A8.8 | P0 |
| 10 | Áreas de trabalho | Opy: Territórios/Áreas | A8.8 | P1 |

### Janelas

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 11 | Minimizar | Recolher ao Rio por gesto espacial | A9.2 | P1 |
| 12 | Restaurar minimizada | Retomar por medalhão ou atalho | A9.2 | P1 |
| 13 | Maximizar | Imersão reversível | A9.2 | P2 |
| 14 | Organizar lado a lado | Confluência com escolha explícita | A9.2 | P2 |
| 15 | Fechar | Encerrar com confirmação e preservação de dados | A9.3 | P0 |
| 16 | Botões X, — e □ | Ações contextuais; retirada opcional onde compatível | A9.3 | P3 |
| 17 | Visão das janelas por área | Mapa de atividades por território | A9.2 | P2 |

### Sistema

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 18 | Barra superior | Semente/Opy + avisos indispensáveis | A9.1 | P0 |
| 19 | Quick Settings | Opy: Centro do Sistema | A9.1 | P0 |
| 20 | Relógio e calendário | Tempo na Semente/Opy | A9.1 | P0 |
| 21 | Notificações | Opy Avisos e histórico consultável | A9.1 | P0 |
| 22 | Indicadores microfone/câmera/tela | Avisos persistentes de privacidade | A9.1 | P0 |
| 23 | Rede Wi-Fi, Ethernet e VPN | Opy Conexões | A9.1 | P0 |
| 24 | Bluetooth | Opy Conexões/Dispositivos | A9.1 | P1 |
| 25 | Volume, mute, dispositivos | Opy Som | A9.1 | P0 |
| 26 | Brilho e luz noturna rápida | Opy Luz | A9.1 | P1 |
| 27 | Energia e bateria | Opy Energia e Consumo | A9.1 | P0 |
| 28 | Bloquear, suspender e desligar | Opy Energia/Sessão | A9.1 | P0 |
| 29 | Captura de tela/gravação | Opy Capturar com permissões | A11 | P1 |

### Aparência

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 30 | Papel de parede | Ysyry: Paisagem e Território | A10 | P1 |
| 31 | Tema escuro/claro | Ysyry: Paletas globais | A10 | P1 |
| 32 | Ícones e fontes | Ysyry: Identidade visual | A10 | P2 |
| 33 | Resolução/escala | Territórios: Monitores | A10 | P1 |
| 34 | Posição/orientação monitores | Territórios: Distribuição espacial | A10 | P1 |
| 35 | Perfil de cor e Night Light | Ysyry: Luz e Cores | A10 | P2 |

### Configuração

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 36 | Atalhos do teclado | Opy Acessos: Mapa de Comandos | A11 | P0 |
| 37 | Leitor de tela, zoom e contraste | Opy Acessibilidade, preservando AT-SPI/Orca | A11 | P0 |
| 38 | Mouse, touchpad e tablet | Opy Dispositivos de Entrada | A11 | P1 |
| 39 | Idiomas e layouts de teclado | Opy Linguagem | A11 | P1 |
| 40 | Região, fuso, data/hora | Opy Tempo e Região | A11 | P1 |
| 41 | Privacidade e permissões | Opy Guardião | A11 | P0 |
| 42 | Usuários e senhas | Opy Identidades | A11 | P2 |
| 43 | Contas online | Opy Pontes de Serviços | A12 | P2 |
| 44 | Compartilhamento e RDP | Opy Conexões Remotas | A12 | P2 |
| 45 | Aplicativos padrão e MIME | Opy Associações | A12 | P2 |
| 46 | Mídia removível | Opy Dispositivos | A12 | P1 |
| 47 | Impressão e digitalização | Opy Impressão e Digitalização | A12 | P2 |

### Arquivos

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 48 | GNOME Files/Nautilus | ARANDU Arquivos | A12 | P1 |
| 49 | Pesquisa de arquivos | Busca Local ARANDU com indexação opcional | A12 | P2 |
| 50 | Lixeira | ARANDU Lixeira Recuperável | A12 | P1 |
| 51 | Copiar, mover, compactar | ARANDU Fluxo de Arquivos | A12 | P2 |
| 52 | Backup/restauração | ARANDU Resguardo | A12 | P1 |

### Aplicativos

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 53 | GNOME Software | ARANDU Catálogo | A13 | P2 |
| 54 | Atualizações | ARANDU Atualizações (APT/Flatpak preservados) | A11 | P0 |
| 55 | Gerenciador de extensões | ARANDU Módulos | A13 | P2 |

### Diagnóstico

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 56 | Monitor de sistema | ARANDU Saúde | A13 | P1 |
| 57 | Logs | ARANDU Diagnóstico | A13 | P2 |
| 58 | Discos/partições | ARANDU Armazenamento | A13 | P2 |

### Apps opcionais

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 59 | Editor de texto | ARANDU Escrita, apenas quando justificado | A14+ | P3 |
| 60 | Terminal | ARANDU Terminal, apenas quando justificado | A14+ | P3 |
| 61 | Leitor de documentos | ARANDU Leitura, apenas quando justificado | A14+ | P3 |
| 62 | Fotos, mídia e calculadora | Apps ARANDU somente por ganho comprovado | A14+ | P3 |

### Infraestrutura

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 63 | GNOME Shell UI | Front-end ARANDU Rust independente | A15+ | P3 |
| 64 | Mutter compositor/WM | Avaliar compositor próprio; não antecipar | A15+ | P4 |
| 65 | gnome-session | Sessão ARANDU independente | A15+ | P3 |
| 66 | gnome-settings-daemon | Agentes ARANDU por função, sem duplicar serviços | A15+ | P3 |

### Padrões Linux

| ID | Função atual | Substituto ARANDU | Fase | Prioridade |
|---|---|---|---|---|
| 67 | D-Bus, portals, PipeWire, BlueZ | Manter serviços e interoperabilidade | Permanente | P0 |
| 68 | NetworkManager, logind, polkit | Manter serviços e controles de permissão | Permanente | P0 |
| 69 | AT-SPI, Orca, libinput | Manter padrões de acessibilidade e entrada | Permanente | P0 |

## Portões de aprovação para retirar a barra definitivamente

1. Resgate da Semente por teclado em qualquer monitor, mesmo após falha da camada gráfica; GNOME original disponível no login.
2. Rede e VPN, som e microfone, energia e sessão controláveis sem depender de botão da barra antiga.
3. Notificações, agenda, status de serviços e **indicadores de câmera, microfone e compartilhamento de tela** visíveis sempre que necessários — mesmo com a Semente guardada.
4. Abertos usa janela individual; Memória distingue atividade encerrada de janela recuperável; Frequentes requer histórico autorizado.
5. Teclado, mouse, alto contraste, AT-SPI/Orca e métodos assistivos não exigem hover, animação ou acesso por microfone.
6. Backup e rollback íntegros; testes de duas telas e retorno após erro.
7. Medição comparativa de CPU, memória, tempo de login, latência e estabilidade (a meta é melhorar, mas não afirmar melhora sem medição).

## Metas de referência — a validar

| Métrica | Direção de engenharia | Como medir |
|---|---|---|
| CPU em repouso | Aproximar-se de zero em espera, sem polling pesado | CPU por processo com Opy fechada |
| Opy pronta | p95 abaixo de 150 ms após cache aquecido (meta provisória) | Abrir e fechar 50 vezes, dois monitores |
| Memória | Menor ou equivalente a GNOME + A8.4 na mesma carga | RSS/PSS da sessão completa, não só Rust |
| Fluidez | Sem travamentos perceptíveis na troca de áreas/janelas | Latência, frames perdidos e CPU/GPU |
| Recuperação | Restauro integral em toda fase | Teste automatizado de rollback |
| Acessibilidade | Todas as ações essenciais por teclado e leitor | Roteiro assistivo, sem mouse |

## Atualizações incrementais

O usuário confirmou funcionalidade da A8.4, inclusive Recolher e Retomar reais, mas considerou os botões excessivos. As ADRs A8.5–A8.7 definem mover a **janela real** para um Rio, guardar a própria Semente e eliminar a barra como paradigma. Isso **não** comprova a implementação dessas ADRs. A tabela acima inicia o backlog e deve receber, para cada função, a implementação, teste, recursos consumidos, compatibilidade, responsável e decisão de promover/recuperar.

**A planilha editável com 69 linhas e colunas adicionais de dependência, estágio e critério de aceitação acompanha esta entrega na conversa.**

## Referências de inventário

- GNOME 46, notas oficiais: https://release.gnome.org/46/
- GNOME Help: https://help.gnome.org/gnome-help/
- GNOME Settings: https://help.gnome.org/gnome-help/prefs.html
- GNOME Components: https://developer.gnome.org/components/
- GNOME Shell 46 developer guide: https://gjs.guide/extensions/upgrading/gnome-shell-46.html
- Freedesktop XDG Desktop Portal: https://flatpak.github.io/xdg-desktop-portal/docs/writing-a-new-backend.html

## Documentos relacionados

- [ADR A8.3 — Rios Discretos](../design/ADR_A8_3_RIOS_DISCRETOS_ESPACOS_VIVOS.md)
- [ADR A8.5 — A Janela Flui](../design/ADR_A85_JANELA_FLUI_PARA_SEMENTE.md)
- [ADR A8.6 — Semente Recolhida](../design/ADR_A86_RIOS_SEMENTE_RECOLHIDA_ATALHOS.md)
- [ADR A8.7 — Sem Barras](../design/ADR_A87_ARANDU_SEM_BARRAS.md)
