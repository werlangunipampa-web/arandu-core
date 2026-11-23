# ADR A8.4 — Janelas Vivas: Recolher e Retomar com Rust + GNOME

**23/11/2026 · Estado: código Rust testado no CI; adaptador GNOME e instalador testados localmente, sem validação na máquina do usuário.**

## Fundamento

**Uma atividade não deixa de existir quando deixa de ocupar a tela.** Os Rios são relações que orientam ações, e não precisam ser faixas permanentes. A Semente, a Opy e os Espaços Vivos serão caminhos coerentes para recuperar e organizar o trabalho, preservando a escolha da pessoa.

A A8.4 inicia a tradução desse compromisso em ações reais e reversíveis. A extensão visual continua GNOME Shell 46, mas **o plano de ação vem do núcleo Rust**, de forma independente da interface.

## Problema observado

A A8.2 identifica aplicativos em Abertos via Snapshot → Rust, mas o mesmo aplicativo pode possuir várias janelas. A lista agregada de IDs não é suficiente para pedir minimização de uma janela particular sem risco de agir sobre a janela errada.

## Contrato técnico

1. A extensão GNOME lista **janelas elegíveis** com `Meta.TabList.NORMAL_ALL`, omitindo diálogos, janelas sem representação de aplicativo e janelas especiais.
2. Cada uma possui **referência opaca** baseada em sua sequência estável e geração efêmera da solicitação. O adaptador informa ao Rust apenas ID de aplicativo, monitor, área de trabalho, capacidades e estado mínimo (minimizada/maximizada/tela cheia).
3. **Títulos de documentos ficam no processo GNOME Shell**, usados apenas como rótulos truncados na Opy. Jamais trafegam no protocolo Rust.
4. Ao clicar em **Recolher**, **Retomar** ou **Focar** em Abertos, o adaptador inicia `arandu-cli --window-intent-stdin` com um protocolo de tamanho limitado. No Rust, `WindowSnapshot` e `plan_window_action` validam a identidade e as capacidades da janela.
5. O Rust retorna apenas sequências permitidas: `minimize`, `unminimize,activate` ou `activate`. **Não executa chamadas ao Mutter.**
6. Antes de aplicar, GNOME **revalida a identidade, monitor, área, estado e capacidades**. Se a janela mudou ou desapareceu, a ação é cancelada. A Opy cancela pedidos pendentes ao fechar.
7. A extensão aplica a operação permitida via `Meta.Window.minimize()/unminimize()` e `Main.activateWindow()`, somente depois do clique explícito. **Nenhum fechamento, kill ou comando shell arbitrário está disponível neste estágio**.

## Decisões de experiência

- Opy → **Abertos** mostra cada janela separadamente, mesmo quando há múltiplas janelas do mesmo aplicativo.
- **Recolher ao Rio** corresponde ao minimizar tradicional, mantendo a atividade acessível em Abertos; não equivale a salvar ou encerrar.
- **Retomar/Focar** tornam a atividade novamente visível e ativa.
- Os botões usuais do GNOME, `Alt+Tab`, Super, atalhos atuais da Confluência, relógio, energia e áreas de trabalho **permanecem funcionais**.
- A ação não inicia automaticamente por proximidade, IA, ECA, voz passiva ou observação de foco.
- A interface continua não modal e navegável por teclado; futura voz e outros dispositivos assistivos serão adaptadores para **as mesmas intenções**.

## O que não está pronto

**Imersão**, **Confluência** e **Encerrar** possuem contratos em Rust, mas ainda não são aplicados em janelas reais. Em especial, Encerrar requer cuidado com documentos não salvos; Confluência requer layouts reversíveis e compatibilidade multimonitor.

**A retirada da barra superior está fora desta entrega**, pois funções de rede, áudio, notificações e indicadores de privacidade ainda dependem dela.

A A8.4 é implementada em protótipo instalável, com teste de sintaxe JS, sete testes offline do instalador e CI `cargo test --workspace --locked` aprovado na revisão Rust `2638de024af563d931413ccfb43566e729936000`. **Não se afirma validação no GNOME real até receber resultado do computador do usuário.**

## Segurança e recuperação

- Verificar A8.2 íntegra pelo hash do arquivo instalado e do manifesto.
- Clonar exatamente uma revisão Rust conhecida e executar os testes antes de editar a extensão.
- Copiar `extension.js` e `stylesheet.css` originais e permitir restauração byte a byte, sem `sudo`.
- Não instalar componentes do GNOME globalmente; não modificar TOPBAR, Confluência antiga ou a sessão GNOME normal.
- Se qualquer pré-requisito divergir, abortar. Não oferecer `--force`.
- Validar as duas telas, a perda de foco, um aplicativo com duas janelas e cancelamento antes de considerar A8.4 consolidada.

## Próximos passos

Validar A8.4 na sessão ARANDU real. Em seguida: tornar a apresentação de Abertos uma grade de atividades com acesso por teclado, criar configuração de atalhos sem conflitos, desenvolver a ação **Imersão** com volta ao tamanho anterior e implementar **Confluência** com duas janelas e desfazer.

**Princípio-guia:** a atividade é contínua; os controles são claros; a autonomia é humana; a representação visual é leve.
