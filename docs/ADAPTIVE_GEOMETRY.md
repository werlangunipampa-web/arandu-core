# ARANDU CORE v0.3.0 — Geometria Adaptativa

## Objetivo

Transformar a disposição dos três rios em uma decisão do núcleo Rust, não da interface gráfica de um sistema operacional.

O renderer informa:

- largura e altura lógicas da área disponível;
- posição desejada da Semente da Confluência;
- quantidade de nós por rio é obtida pelo próprio Core.

O Core devolve:

- posição segura da Semente;
- coordenadas de cada nó;
- direção escolhida de cada rio;
- espaçamento efetivo;
- indicador de compactação.

## Regras v0.3.0

- **MEMÓRIA** prefere o eixo horizontal.
- **ABERTOS** prefere o eixo vertical.
- **FREQUENTES** prefere um eixo diagonal.
- Um rio pode inverter sua direção quando houver mais espaço no lado oposto.
- A Semente é empurrada para dentro da margem segura se estiver fora da área útil.
- O espaçamento é reduzido em telas restritas antes de permitir corte.
- A quantidade de nós determina diretamente o comprimento necessário de cada rio.
- O resultado usa unidades lógicas e não conhece pixels físicos, DPI, toolkit ou sistema operacional.

## Responsabilidade dos adapters

Linux, macOS e Windows continuam responsáveis somente por descobrir a área disponível e renderizar as coordenadas devolvidas pelo Core. A política geométrica permanece única.
