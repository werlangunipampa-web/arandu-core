#!/usr/bin/env bash
# Diagnóstico A1.9: leitura apenas; sem sudo, sem alteração de serviços.
set -Eeuo pipefail
export LC_ALL=C
cd "$(dirname "${BASH_SOURCE[0]}")/.."
echo "========== ARANDU / Linux Mint — diagnóstico somente leitura =========="
date -Is
echo
echo "== Ambiente gráfico e sistema =="
if [[ -r /etc/os-release ]]; then
  grep -E '^(PRETTY_NAME|VERSION_ID)=' /etc/os-release || true
fi
printf 'Desktop=%s / Sessão=%s\n' "${XDG_CURRENT_DESKTOP:-desconhecido}" "${XDG_SESSION_TYPE:-desconhecida}"
uname -rm
echo
echo "== Carga e memória =="
uptime
free -h
if [[ -r /proc/pressure/cpu ]]; then
  for kind in cpu memory io; do
    if [[ -r /proc/pressure/$kind ]]; then
      printf '%s: ' "$kind"
      head -n 1 "/proc/pressure/$kind"
    fi
  done
fi
printf '\n== Disco e swap ==\n'
df -h / 2>/dev/null || true
swapon --show --noheadings --bytes 2>/dev/null | awk '{sum+=$3; used+=$4} END {printf "Swap bytes: total=%0.f usada=%0.f\n",sum,used}'
echo
echo "== Processos com maior CPU (sem argumentos privados) =="
ps -eo pid,comm,%cpu,%mem,rss,etime --sort=-%cpu | head -n 16
echo
echo "== Processos com maior RAM (sem argumentos privados) =="
ps -eo pid,comm,%cpu,%mem,rss,etime --sort=-rss | head -n 16
echo
echo "== Processos ARANDU (nome, uso e tempo) =="
ps -eo pid,comm,%cpu,%mem,rss,etime | grep -Ei 'arandu|python3|cinnamon' | head -n 30 || true
echo
echo "== Inicialização de sessão / boot (resumo) =="
if command -v systemd-analyze >/dev/null 2>&1; then
  timeout 8s systemd-analyze time 2>/dev/null || true
fi
echo
echo "== Tempo do core Rust pré-compilado (3 execuções) =="
if [[ -x target/debug/arandu-cli ]]; then
  python3 - <<'PY'
import subprocess,time
from pathlib import Path
binary=str(Path("target/debug/arandu-cli").resolve())
for i in range(3):
    start=time.perf_counter()
    try:
        result=subprocess.run([binary,"--window-protocol"],input="quit\n",
                              text=True,capture_output=True,timeout=5,check=False)
        elapsed=(time.perf_counter()-start)*1000
        print(f"Execução {i+1}: {elapsed:.1f} ms, retorno={result.returncode}, bytes={len(result.stdout)}")
    except subprocess.TimeoutExpired:
        print(f"Execução {i+1}: timeout >5s")
PY
else
  echo "SKIP: binário target/debug/arandu-cli ainda não encontrado; não compilei nada."
fi
echo
echo "== Benchmark gráfico sintético, se compilado =="
if [[ -x target/debug/arandu-cli ]]; then
  python3 scripts/benchmark_window_protocol.py --frames 100 || true
fi
echo
echo "== Resumo =="
echo "Nenhum pacote instalado, processo encerrado, configuração alterada ou arquivo de usuário lido."
echo "Os dados acima são amostras, não um diagnóstico definitivo da máquina."
