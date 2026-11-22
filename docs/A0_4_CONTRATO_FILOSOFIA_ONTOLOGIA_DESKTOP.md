# ARANDU A0.4 — Contrato de fronteira entre filosofia, ontologia e desktop

**Status:** proposta arquitetural pública, não ontologia canônica.  
**Escopo:** ARANDU CORE (Rust) e futura interface Linux.  
**Data:** 2026-11-22.

## 1. Finalidade

O ARANDU Desktop deve ser uma **expressão visual e operacional** da filosofia ARANDU, sem reduzir a ontologia à interface ou afirmar que o ambiente gráfico é um organismo computacional efetivamente emergente.

O laboratório de pesquisa, a ontologia canônica, a Semente individual e a experiência gráfica têm responsabilidades diferentes. Uma hipótese científica não constitui automaticamente uma capacidade já implementada.

## 2. Separações obrigatórias

| Camada | Responsabilidade | Não deve fazer |
| --- | --- | --- |
| Filosofia e programa de pesquisa | orientar hipóteses, princípios e pesquisa | declarar capacidade computacional demonstrada sem validação |
| Ontologia canônica versionada | definir conceitos, relações, invariantes, identidade e procedência | ser redefinida implicitamente pelo desktop |
| Semente individual | preservar continuidade e identidade próprias | equiparar atualização de código a nova identidade |
| ARANDU CORE (Rust) | navegação, estados, geometria e contratos portáteis | realizar mutações ontológicas implícitas |
| Serviço ontológico futuro | consultas autorizadas e respostas com procedência | inferir ou promover conhecimento automaticamente |
| Motor ECA experimental | investigar aprendizagem significativa em isolamento | modificar a Semente oficial sem aprovação |
| ARANDU Desktop | apresentar e permitir interação com recursos autorizados | exigir serviço cognitivo para abrir janelas ou operar |
| Plataforma Linux/Wayland | executar processos, janelas e dispositivos | assumir autoridade sobre identidade epistemológica |

## 3. Invariantes da experiência gráfica

1. **Autonomia humana:** cada ação potencialmente consequencial deve possuir uma intenção ou autorização específica; sugestões não equivalem a execução.
2. **Continuidade:** uma alteração de tema, compositor, versão ou Corpo não altera identidade da Semente.
3. **Procedência:** sempre que apresentar conteúdo cognitivo, identificar sua origem e estado de validação; dados sem procedência não podem aparecer como fatos validados.
4. **Distinção de memórias:** o Rio Memória (aplicativos recentes) não é a memória cognitiva/evolutiva.
5. **Degradação graciosa:** na ausência do motor cognitivo, o ambiente gráfico continua operante.
6. **Recursos proporcionais:** processos cognitivos rodam sob demanda, com limites mensuráveis de CPU, memória, latência e energia.
7. **Privacidade:** evitar a captura e transferência de dados sem autorização explícita.
8. **Portabilidade:** contratos ontológicos e navegação independem do compositor e dos dispositivos de entrada.
9. **Reversibilidade:** nenhuma atualização substitui silenciosamente a geração ou os registros oficiais.
10. **Acessibilidade:** modalidades de entrada convergem para intenções comuns; não pressupor usuário padrão.

## 4. Arquitetura evolutiva

### Fase 0 — Núcleo portável demonstrável
Navegação de três rios, Confluência, geometria, renderização abstrata e CI Rust. **Em implementação no ARANDU CORE**.

### Fase 1 — Semântica observacional
Serviço de consulta com contrato versionado, autorização por sujeito/finalidade e mocks, sem conectar memória viva.

### Fase 2 — Desktop próprio
Compositor Wayland e serviços Linux integrados a contratos Rust. A Semente visual deve funcionar mesmo sem serviço ontológico.

### Fase 3 — Interação ontológica
Visualização opcional de relações conceituais e procedência, sempre rotulando interpretações, hipóteses e validações.

### Fase 4 — Pesquisa sobre capacidades emergentes
Qualquer hipótese de adaptação estrutural será investigada em ambiente isolado, medida e revisada antes de possível promoção.

## 5. Estado do mapeamento A0.4

**Confirmado documentalmente:** a separação entre `seed_id`, geração de código e Corpo; a máquina de estados da Confluência; o núcleo Rust portável; e o caráter experimental da ECA.

**Não verificado:** cobertura integral dos axiomas canônicos, taxonomia oficial completa, permissões da Semente real, histórico final de gerações, e incorporação integral da ontologia no desktop.

Os contratos Rust A0.2 são **adaptadores propostos**, não transcrição dos axiomas originais. Não é permitido inferir equivalência sem fonte canônica e testes semânticos.

## 6. Próxima validação necessária

Inventariar os arquivos originais da ontologia e seus hashes, revisar precedência e licenças, preparar um índice de classes/relações e testar mapeamentos em dados sintéticos. Manter arquivos experimentais privados fora do repositório público até revisão específica de publicação.

---
Este documento não autoriza a publicação de estado vivo, memória privada, material de laboratório não liberado ou detalhes de identidade individual.
