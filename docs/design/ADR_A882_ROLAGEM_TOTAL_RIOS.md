# ARANDU A8.8.2 — Rolagem total, sem truncar atividades

**Data:** 23/11/2026 · **Estado:** fontes e instalador experimentais; testes offline aprovados, ainda não validado em GNOME Shell real.

## Observação real

O usuário confirmou que a Confluência/Opy da A8.8.1 funcionava, mas não conseguia percorrer todos os aplicativos. A análise do código original identificou cortes explícitos:
- Opy → Aplicativos: `installed.slice(0,35)`;
- Opy → Pesquisa: `matches.slice(0,9)`;
- Rio Abertos: `windows.slice(0,12)`;
- Rios Memória e Frequentes: `ids.slice(0,10)`;
- Snapshot histórico: `unique(...).slice(0,15)`.

**Esses cortes impedem a rolagem de alcançar itens não renderizados.**

## Correção de experiência

- **Aplicativos:** St.ScrollView vertical existente é mantido. Carregamento progressivo de 24 itens por vez, disparado somente quando a pessoa percorre a lista até perto do fim. Nenhum polling permanente, cronômetro ou trabalho periódico em repouso. Opy mantém os ícones e nomes originais dos aplicativos instalados e respeita `should_show()`.
- **Pesquisa:** deixa de limitar resultados aos nove primeiros e usa a mesma paginação acionada por rolagem.
- **Três Rios:** Memória, Abertos e Frequentes continuam em três colunas de tamanho contido, mas **cada coluna ganha seu próprio St.ScrollView vertical**. A vista não precisa expandir o fundo inteiro. Os cartões não são mais truncados artificialmente após 10 ou 12 itens. Um contador exibe a quantidade efetivamente retornada.
- **Histórico:** o adaptador pode receber até 128 IDs distintos por Rio de um Snapshot local. A origem dos dados pode guardar menos itens; a UI **não inventa histórico**.
- **Teclado:** preserva `Super+Alt+1/2/3`, `Super+Alt+←/→`, `Super+Alt+Enter`, `Super+Alt+0`. A seleção por seta deve ajustar a rolagem para manter o cartão visível.
- **Resgate:** mantém `Super+Alt+9`, Semente guardável, GNOME original como sessão de recuperação, e o painel GNOME inalterado.
- **Rust:** não altera o núcleo Rust A8.4 ou a modelagem dos Rios; esta é uma correção de apresentação GNOME Shell enquanto a ponte Rust da Opy não é concluída.

## Incidente paralelo: A9.0 interrompida

O usuário registrou `ARANDU_A90_COMPATIBLE` seguido de `ARANDU_A90_ERROR: Arquivo ausente ou inesperado: .../a90-sistema/bin/arandu-sistema`. O instalador A9.0 cria o manifesto antes do fim da aplicação e limita o binário a até 4 MB na rotina de integridade; o erro pode refletir ausência ou tamanho excessivo. **Não é possível afirmar a causa específica sem inspeção do arquivo instalado.**

Como o A9.0 pode ter deixado parte do JS/CSS/metadata/Rust alterada, **A8.8.2 deve recusar a instalação se a pasta de backup ativo da A9.0 existir**. O utilitário `arandu_a90_recuperacao_segura.py` faz `--inspect` somente leitura, verifica os hashes de todos os arquivos contra o manifesto A9.0 e somente com `--recover` solicitado restaura arquivos anteriores comprovados. Fontes e binários novos com hashes reconhecidos são preservados dentro do backup arquivado; arquivos inesperados impedem qualquer mudança.

## Validações

- 7 testes de integridade e semântica do instalador A8.8.2.
- Sintaxe JavaScript verificada via Node.
- Ensaio sintético: `--check → --apply → --check → --restore → --check → --apply → --restore`, com os hashes da A8.8.1 recuperados em ambas as restaurações.
- Ensaio sintético da recuperação parcial A9.0: `--inspect → --recover → validação SHA-256`, com extensão recuperada.
- **Ainda pendente:** teste real com rolagem no GNOME 46/X11 do usuário, duas telas, muitos aplicativos, teclado e orientação assistiva.

## Evolução

Próximo passo: A9.1 Sistema Rust revalidado sobre a base de código A8.8.2, corrigindo o limite de tamanho e o rollback do binário antes de qualquer nova tentativa de implantação. A solução sem barra definitiva continua condicionada a status de privacidade, avisos e controles críticos acessíveis.

[ADR A8.8.1](ADR_A881_NAVEGACAO_TECLADO_TRES_RIOS.md) · [ADR A9.0](ADR_A90_OPY_SISTEMA_RUST.md) · [Plano Diretor](../arquitetura/PLANO_DIRETOR_SUBSTITUICAO_GNOME_69_FUNCOES.md)
