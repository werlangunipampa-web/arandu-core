# A1.10 — Convergência visual com o ARANDU instalado no GNOME

## Referência de verdade

A referência não é uma imagem inventada: é a suíte GNOME já desenvolvida em [ARANDU-TAPE](https://github.com/werlangunipampa-web/ARANDU-TAPE).

Elementos examinados no repositório da suíte:

- `ARANDU-SUITE/TOPBAR/arandu-topbar@local/stylesheet.css`: barra entre 46 e 54 px, fundo mate/terra, acentos em dourado, botões de cantos arredondados, hover discreto.
- `ARANDU-SUITE/Confluencia/app/assets/semente.svg`: Semente original com aro dourado, face escura e relevo concêntrico.
- `README.md` e `INSTALAR.md`: Memória, Abertos e Frequentes, repouso da Semente após 7 s e ajuda após 6 s.
- `ARANDU-SUITE/docs/COMANDOS-E-DIAGNOSTICO.md`: contrato assistivo e atalhos históricos.

## Implementado nesta etapa

1. A janela assíncrona do Linux Mint adota a paleta terra/mate/dourado em lugar do tema verde genérico.
2. Cabeçalho compacto com identidade ARANDU, Confluência, Semente e Três Rios.
3. Botões com contraste, estados hover e dimensões menores, próximos à linguagem da TOPBAR existente.
4. Desenho da Semente em **anéis concêntricos calculados em Rust**, inspirado na identidade cromática do SVG original, preservando a renderização leve em Tk.
5. Medalhões e conectores mantêm geometria e estados vindos do núcleo Rust.
6. Nenhuma chamada a IA ou ECA no caminho crítico de navegação e renderização.

## O que NÃO mudou

- O GNOME instalado não foi alterado, atualizado ou reiniciado.
- Nenhuma extensão existente foi substituída.
- A A1.10 continua sendo **janela experimental**; Tk e GNOME Shell não compartilham o mesmo renderizador.
- Os aplicativos apresentados nessa janela continuam sintéticos.
- A Semente nova é uma aproximação por primitivas SVG, **não uma reprodução exata do arquivo original**.
- A barra do GNOME pode ter personalizações locais diferentes das registradas no repositório.

## Próximas etapas para convergência funcional

- Registrar imagens e métricas do GNOME em execução somente com consentimento do usuário; comparar escala, espaçamento, cores e disposição de rios.
- Compartilhar um contrato de identidade visual (tokens, estados, comportamento) entre as implementações, sem acoplar o núcleo a GNOME.
- Reproduzir os tempos de repouso e a ajuda respeitando as regras específicas da versão GNOME.
- Preservar a TOPBAR e o caminho de recuperação; testar separadamente GNOME X11 e futuras sessões Wayland.
- Medir tempos reais na máquina Linux Mint antes de adicionar animações.

Critério de aceite: CI verde, teste de teclado e janela virtual e validação humana no GNOME/X11 real.
