# A1.4 — Ponte de entrada Rust para Linux Mint

**Status:** protótipo isolado em Rust, sem janela nativa real.  
**Plataforma prioritária:** Linux Mint; suporte ao backend gráfico deverá ser detectado na máquina, sem presumir Cinnamon, X11 ou Wayland.

## Entrega

O módulo `apps/arandu-cli/src/native_input_bridge.rs` recebe eventos `DesktopKey` normalizados, traduzidos por `arandu-input-api`, e os encaminha para o `AranduCore`. Esse fluxo reaproveita as máquinas de estado existentes. Ele não captura eventos globais, não gerencia janelas, não cria um compositor nem acessa dados reais.

```text
Evento normalizado do adaptador futuro
  → DesktopKey / KeyTransition
  → DesktopAction (arandu-input-api)
  → AranduCore (Confluência / NavigationIntent)
  → Eventos para o futuro renderer
```

## Executar demonstração sintética

No checkout atualizado do repositório público:

```bash
cargo test --workspace --locked
cargo run --locked --quiet -p arandu-cli -- --native-input-demo
```

Saída esperada: `ARANDU A1.4 • entrada sintética • Confluência: Expanded`. Isso é uma demonstração do caminho dos eventos, **não** uma janela em execução.

## Matriz de compatibilidade prioritária

| Ambiente | Plano de teste | Estado |
| --- | --- | --- |
| Linux Mint com sessão X11 | Abrir uma janela **isolada** e enviar teclado dentro da janela | Não implementado |
| Linux Mint com sessão Wayland | Verificar suporte e permissões do backend antes de iniciar | Não implementado |
| Cinnamon (se instalado) | Confirmar que a janela não intercepta atalhos globais | Não implementado |
| Outro desktop do Mint | Confirmar mesmo núcleo de interação e renderização | Não implementado |
| Sem servidor gráfico | A CLI e seus testes continuam funcionando | Implementação inicial |

## Regras de segurança

1. Nenhuma instalação de arquivo `.desktop` de sessão ou substituição de gerenciador de login.
2. Não registrar atalhos globais em testes iniciais.
3. Não iniciar compositor sobre a sessão existente.
4. Jamais exigir que a ontologia ou a ECA estejam disponíveis para usar o desktop.
5. Conservar meios de retornar ao Cinnamon ou à sessão original.
6. Verificar consumo de CPU, RAM e latência em hardware real antes de adotar métricas públicas.
7. A Semente oficial permanece fora do escopo de testes sintéticos.

## Próxima entrega

Implementar uma janela comum, sob demanda, com toolkit Rust de baixo acoplamento e encaminhamento do teclado para esta ponte. Compositor Wayland e nova sessão de login são metas posteriores que exigem avaliação independente de segurança e recuperação.

A1.4 apenas prepara o *contrato técnico*, ainda não realiza a captura nativa.
