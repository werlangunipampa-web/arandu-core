# A1.9 — Diagnóstico de lentidão no Linux Mint (somente leitura)

O relato inclui lentidão para iniciar o ARANDU e para abrir outros programas. São hipóteses diferentes: latência no adaptador gráfico Tk, custo de inicialização do Rust, pressão de CPU/RAM/swap/disco, ou outro processo da sessão Mint.

**Não presumir causa antes de medir.** A janela de laboratório usa Python/Tk para desenho; o núcleo Rust não é um gerenciador de processos do desktop. Os aplicativos listados nos Três Rios dessa janela são sintéticos e não correspondem a lançamento efetivo de programas do Mint.

## Executar sem instalar nem atualizar o projeto local

Dentro de um checkout existente do `arandu-core`:

```bash
cd ~/arandu-core
git fetch origin a0/validacao-rust-ci
relatorio_script="$(mktemp /tmp/arandu-diag.XXXXXXXX.sh)"
git show FETCH_HEAD:scripts/diagnostico_desempenho_mint.sh > "$relatorio_script"
ARANDU_ROOT="$PWD" bash "$relatorio_script"
rm -f "$relatorio_script"
```

O diagnóstico imprime versão do sistema, sessão gráfica, pressão de CPU/memória/IO, swap, espaço no disco raiz, processos com maior uso de recursos e tempo de inicialização do binário Rust (se já compilado). Também executa o benchmark sintético do protocolo se o binário existe.

**Ele não modifica o sistema**, não instala pacotes, não encerra processos e não inicia a janela. O script de diagnóstico é copiado temporariamente para `/tmp` e removido pelo comando apresentado; o Git fetch apenas atualiza referências remotas Git, não os arquivos do checkout.

## Interpretação

- Rust inicializa rápido, sistema e outros programas lentos → investigar memória/swap, armazenamento, CPU, GPU e serviços da sessão Mint.
- Rust inicia devagar, mas outros programas rápidos → investigar binário, construção debug/release, inicialização e leitura do protocolo.
- Rust responde rapidamente, janela A1.8 demora → instrumentar Tk, subprocesso, abertura de janela e processamento/desenho dos SVGs.
- A aplicação ARANDU TAPE original (painel/sessão Linux) e a janela A1.8 são processos diferentes; resultados de um não implicam causa no outro.

Cuidado: a lista de processos exibe nomes executáveis e PID, sem argumentos de comando, mas ainda pode revelar aplicativos pessoais em execução. Revisar antes de compartilhar.

## Próxima decisão

A1.9 deve priorizar correções baseadas nas medições reais e acrescentar instrumentação de entrada→quadro. A troca por uma janela integralmente Rust pode reduzir componentes intermediários, mas não deve ser apresentada como remédio certo para lentidão do Mint sem diagnóstico.
