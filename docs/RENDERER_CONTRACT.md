# ARANDU CORE v0.4.0 — Renderer Contract

A v0.4.0 cria a fronteira entre a inteligência do ARANDU e qualquer backend gráfico.

O Core decide estado, geometria, identidade semântica, opacidade, escala,
conectores e rótulos. O backend decide como desenhar isso em GNOME/KDE,
Windows ou macOS.

O símbolo ARANDU continua sendo a identidade principal do nó e a referência
ao ícone original é preservada como identidade secundária/mini-selo.

Em `Collapsed`, os nós permanecem geometricamente na Semente e invisíveis.
Em `Aware`/`Near`, eles emergem progressivamente. Em `Expanded`/`Pinned`,
atingem as posições finais calculadas pela Geometria Adaptativa.
