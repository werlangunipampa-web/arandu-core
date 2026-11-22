# A1.1 — Prévia interativa local da Confluência

## O que esta etapa entrega

A CLI produz uma página HTML autônoma com três cenas SVG **pré-calculadas no Rust** usando `AranduCore::render_scene`. A página inclui controles acessíveis para alternar entre Semente recolhida, Três Rios expandidos e visualização fixada.

**Importante:** os botões alternam instantâneos pré-renderizados. A máquina de estados Rust **não** está sendo executada continuamente no navegador. Não há gerenciamento real de janelas, compositor Wayland, observação do ambiente ou motor ECA conectado.

## Gerar e abrir no Linux

No checkout atualizado do repositório:

```bash
cargo test --workspace --locked
cargo run --locked --quiet -p arandu-cli -- --preview-html > /tmp/arandu-a1.html
xdg-open /tmp/arandu-a1.html
```

A geração é inteiramente local, sem rede e com dados sintéticos. O comando `xdg-open` abre o HTML no aplicativo padrão do sistema; não altera a sessão Linux nem instala componentes.

## Interação disponível

- **1:** recolher a Semente.
- **2:** visualizar os Três Rios.
- **3:** visualizar a Confluência fixada.
- **Escape:** retornar à Semente.
- Os botões expõem seu estado por `aria-pressed`, são focáveis e podem ser ativados pelo teclado.

A mudança de estado entre os três instantâneos é controlada pelo navegador. Ela não envia intenção de navegação ao núcleo Rust, não abre aplicativos e não representa a existência de um sistema de eventos Wayland.

## Critérios de aceitação

1. Compilar e rodar os testes Rust, incluindo produção determinística das três cenas e validação da estrutura HTML.
2. Inspecionar a página no navegador, por teclado, em desktop e em uma janela estreita.
3. Conferir contraste, rótulos de estado, foco visível e ausência de carregamento externo.
4. Confirmar que nenhuma instância da Semente oficial, memória cognitiva ou identidade real foi acessada.
5. Medir tempo de geração e tamanho do arquivo em ambiente controlado antes de definir metas de desempenho.

## Próxima etapa

Investigar adaptador de eventos nativo com renderização interativa em processo isolado, preservando uma sessão gráfica alternativa funcional. O compositor Wayland completo exigirá um plano independente de segurança, entrada, saída, janelas, sessões e recuperação.

**A1.1 é um protótipo de interação e visualização, não o ARANDU Desktop pronto.**
