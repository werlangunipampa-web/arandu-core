# ARANDU CORE v0.2 — Semente da Confluência

A Semente da Confluência é o estado compacto do ARANDU. Em repouso, apenas o centro permanece visível. A complexidade dos três rios emerge somente quando existe intenção de interação.

## Estados

- `Collapsed`: somente a Semente.
- `Aware`: a Semente percebe aproximação e pode ganhar halo/microanimação.
- `Near`: os primeiros veios/conectores podem começar a aparecer.
- `Expanded`: MEMÓRIA, ABERTOS e FREQUENTES ficam totalmente disponíveis temporariamente.
- `Pinned`: os rios permanecem abertos até comando explícito.

## Independência de hardware

O Core não conhece mouse, trackpad, GNOME, Cocoa ou Win32. Ele recebe sinais abstratos (`Proximity`, `HoverEntered`, `Expand`, `TogglePin`, `Collapse`, `Activity`, `Tick`) e devolve estado/eventos.

A proximidade é normalizada entre `0.0` e `1.0`. Cada adapter decide como transformar distância física, gesto, foco assistivo ou outro mecanismo nessa escala.

## Tempo determinístico

O Core não lê o relógio do sistema para o recolhimento automático. O host fornece um tempo lógico monotônico (`Duration`). Isso torna os testes determinísticos e mantém a mesma semântica em Linux, macOS e Windows.

## Regra importante de UX

Ao abrir por hover, sair do círculo central não fecha os rios imediatamente. O adapter deve enviar `Activity` enquanto o usuário percorre os nós. Sem atividade, o estado temporário recolhe automaticamente (padrão: 4 s). `Pinned` nunca recolhe por timeout.

## Próximo marco

v0.3 implementará Geometria Adaptativa: cálculo de posições, espaçamento e orientação dos rios conforme viewport, posição da Semente, margens e quantidade de nós.
