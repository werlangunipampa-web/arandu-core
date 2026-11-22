# A1.3 — Fronteira portável de teclado para janela nativa

## Entrega

O novo módulo `crates/arandu-input-api/src/desktop_keyboard.rs` introduz eventos de teclado **normalizados** (`DesktopKey`) e sua tradução para intenções da Confluência e dos Três Rios.

**Importante:** este módulo não abre uma janela, não captura teclas do sistema e não registra hooks. Ele fornece uma fronteira Rust que um futuro adaptador de janela nativa poderá utilizar, em X11, Wayland ou outras plataformas.

## Correspondência inicial

| Evento | Intenção Rust |
| --- | --- |
| Escape ou 1 | Recolher Confluência |
| 2 | Expandir |
| 3 | Fixar / desafixar |
| Seta direita/baixo | Próximo nó |
| Seta esquerda/cima | Nó anterior |
| M | Rio Memória |
| O | Rio Abertos |
| F | Rio Frequentes |

Somente eventos `Press` mapeiam para ações; `Release` e teclas desconhecidas não geram ação.

## Cobertura de testes

- Escape sempre significa `Collapse`;
- eventos `Release` e desconhecidos não disparam navegação;
- teclas para rios e movimento convergem para `NavigationIntent`;
- `Expand` e `TogglePin` permanecem ações diferentes.

## Segurança e próximo estágio

As teclas mapeadas **não** ativam aplicativos, executam comandos, leem o estado da Semente ou acessam a ECA. Não há janela nativa nesta entrega. Antes de criar o compositor Wayland, é preciso integrar um toolkit de janela isolado, encaminhar eventos normalizados ao motor de estados, renderizar cenas e medir consumo e latência.

**Critério:** nenhum merge sem CI verde e revisão humana.
