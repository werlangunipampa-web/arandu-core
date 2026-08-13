# Contribuindo

1. Crie branch a partir de `main`.
2. Não introduza dependência de sistema operacional dentro de `arandu-core`, `arandu-model` ou `arandu-navigation`.
3. Novas modalidades devem traduzir sinais para `NavigationIntent`.
4. Novos adaptadores implementam `PlatformAdapter`.
5. Rode `scripts/01_validar_workspace.sh` antes de abrir PR.
6. Recursos de voz/câmera/som devem ser locais por padrão e documentar riscos de acessibilidade e ativações involuntárias.
