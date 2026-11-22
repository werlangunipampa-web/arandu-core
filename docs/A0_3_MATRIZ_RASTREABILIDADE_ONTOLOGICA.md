# ARANDU A0.3 — Matriz de rastreabilidade ontológica

**Status:** especificação de interoperabilidade em desenvolvimento (não é a ontologia canônica).  
**Repositório de integração:** `arandu-core`  
**Marco:** A0.3 — correspondência entre conceitos documentados e contratos Rust  
**Princípio:** a experiência gráfica é uma expressão da ontologia; não redefine a identidade da Semente.

## 1. Evidências documentais verificadas

| Fonte | Evidência | Consequência para o desktop |
| --- | --- | --- |
| `ARANDU-G4-PRIVATE/README.md` | Semente individual (`seed_id`), geração de código e Corpo são entidades distintas; migrações exigem registro causal e compatibilidade | Não equiparar atualização de software à substituição da Semente |
| `ARANDU-ECA-PRIVATE/README.md` | ECA é laboratório teórico-experimental pré-geracional; resultados não viram capacidade oficial automaticamente | Consultas ECA somente em ambiente isolado até validação |
| `ARANDU-AVE-PRIVATE/README.md` | Separação entre core portátil, adaptadores, mocks e referência oficial não modificada | Interface e serviços da plataforma não são o núcleo ontológico |
| `arandu-core/docs/CONFLUENCE_SEED.md` | Estados Collapsed, Aware, Near, Expanded, Pinned; tempo lógico fornecido pelo host | Comportamento reproduzível da Semente visual sem relógio oculto |
| `arandu-core/crates/arandu-model/src/lib.rs` | Rios Open, Memory, Frequent; nós de aplicativo e memória recente | Rio Memória de aplicativos **não é** memória cognitiva/estrutural da Semente |
| `arandu-core/crates/arandu-model/src/ontology_contract.rs` | Contratos experimentais de identidade, procedência, status epistemológico e leitura | Ponto de integração **proposto**, sujeito à validação semântica |

**Limite de verificação:** a leitura dos arquivos acima não equivale a inspecionar todos os axiomas e artefatos canônicos da ontologia ARANDU. Antes de declarar compatibilidade ontológica completa, localizar manifestos, versões, relações, invariantes e histórico de migração.

## 2. Matriz de compatibilidade (provisória)

| Conceito | Contrato Rust inicial | Grau de correspondência | Lacuna / teste necessário |
| --- | --- | --- | --- |
| Identidade individual | `SeedReference.seed_id` | Estrutural, preliminar | Verificar formato, unicidade, autoridade, ciclo de vida e mudança proibida |
| Geração técnica | `GenerationReference.generation_id` | Estrutural, preliminar | Verificar genealogia técnica, compatibilidade e migração |
| Corpo | `BodyReference.body_id` | Estrutural, preliminar | Verificar autorização, nidificação e revogação |
| Procedência | `Provenance(source_id,recorded_at)` | Insuficiente | Adicionar cadeia, autoria, evidência, hashes e verificações de integridade após exame da ontologia |
| Estados epistemológicos | `EpistemicStatus` | Hipótese de integração | Conferir taxonomia e processos de validação; enum não representa axioma formal |
| Conhecimento | `KnowledgeReference` | Hipótese de integração | Distinguir conceitos, relações, interpretações, experiências e evidências |
| Leitura | `OntologyReader.lookup` | Contrato técnico inicial | Definir sujeito autorizado, finalidade, escopo, auditoria e política de negação |
| ECA | Nenhum adaptador integrado | Não implementado | Definir protocolo sintético, sem alteração de memória oficial |
| Memória cognitiva | Não implementada no Core de navegação | Não implementado | Não confundir com `RecentMemory` (MRU de aplicativos) |
| Consentimento | Ainda não modelado | Não implementado | Definir autorização explícita, revogação e testes de não divulgação |
| Ontologia completa | Não incorporada | Não verificado | Inventariar ontologia canônica antes da normalização para Rust |

## 3. Restrições de projeto

1. **Identidade:** mudanças na geração de software não autorizam alterar `seed_id`.
2. **Autonomia:** nenhum efeito externo a partir de uma simples consulta ontológica.
3. **Epistemologia:** observação, hipótese, simulação e interpretação não são equivalentes a conhecimento validado.
4. **Procedência:** conteúdo sem origem verificável deve ser identificado como não validado.
5. **Privacidade:** nenhum backup, identificador vivo, estado pessoal, token ou memória privada entra no repositório público.
6. **Aprendizagem:** nenhuma experiência experimental do ECA modifica automaticamente o estado oficial.
7. **Desempenho:** interface deve funcionar quando o serviço ontológico estiver ausente ou indisponível.
8. **Portabilidade:** modelos de domínio não dependem de GNOME, Wayland, Qt ou de drivers.
9. **Reversibilidade:** migração e promoção de capacidades precisam de validação e trilha de auditoria.
10. **Cultura:** termos guarani e interpretações culturais exigem pesquisa de fontes e revisão apropriada; não pressupor equivalências.

## 4. Pontos fracos dos contratos A0.2 — explícitos

- `SeedReference`, `GenerationReference` e `BodyReference` contêm strings sem validação de formato ou autoridade: **não oferecem certificação de identidade**.
- `OntologyReader` não possui método de escrita, mas **não implementa autorização nem auditoria por si só**; quem implementa o trait deve cumprir as políticas de acesso.
- `KnowledgeReference` é uma referência mínima, não uma representação da estrutura cognitiva ausubeliana.
- `EpistemicStatus` não realiza promoção de hipóteses, validação científica nem avaliação de evidências.
- Nenhum dado da Semente oficial foi importado.

## 5. Próximas entregas e critérios de aceite

### A0.4 — Recuperação ontológica canônica
- Identificar o(s) arquivo(s) original(is) RDF/Turtle, manifestos e constituição da Semente.
- Determinar precedência entre gerações e distinguir estado oficial de laboratórios.
- Criar mapa de classes, propriedades, restrições e proveniência com referência exata.
- **Aceite:** cada correspondência deve apontar para fonte/versão verificável.

### A0.5 — Contrato de autorização (somente dados sintéticos)
- Consultas com sujeito, finalidade, escopo e decisão explícitos.
- Padrão de negação por falta de permissão; logs sem dados sensíveis.
- Testes negativos para leitura não autorizada.
- **Aceite:** CI verde e revisão humana de fronteiras de privacidade.

### A1 — Integração experimental
- Adaptador ECA substituível por mock; jamais acesso direto à Semente viva na CI.
- Medir latência, memória e tempo de resposta do desktop com e sem serviço.
- **Aceite:** operação gráfica independente de falha/ausência do serviço.

## 6. Política de distribuição

Documentação e componentes distribuíveis podem ser publicados nos repositórios públicos; pesquisas privadas, estado vivo, dados pessoais e artefatos ainda não revisados permanecem privados. Atualizar via commits e PRs, validar na CI e publicar somente após aprovação.

---
**Esta matriz é uma especificação técnica rastreável, não uma declaração de completude ontológica.**
