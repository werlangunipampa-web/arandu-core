# ARANDU A0.6 — Inventário reproduzível de artefatos ontológicos

## Alcance

O script `scripts/ontology_inventory.py` gera um inventário **local**, determinístico, de arquivos **rastreados por Git** sob o diretório `ontology/`, com extensões RDF/Turtle, RDF/XML, OWL, JSON-LD e variantes de triplas/quádruplas. Produz caminho relativo, tamanho e SHA-256.

**Atenção:** encontrar um arquivo no diretório `ontology/` não comprova que ele seja canônico ou a versão oficial da Semente. O inventário documenta artefatos, não interpreta axiomas, não valida semântica RDF/OWL e não representa a ontologia completa do ecossistema.

## Executar no checkout do `arandu-core`

```bash
python3 scripts/ontology_inventory.py --root .
```

O script apenas lê arquivos rastreados e imprime JSON na saída padrão. Não modifica Git, a sessão gráfica, o estado vivo da Semente ou a instalação local. A presença da CLI `git` e do Python 3 é necessária. Se houver erro, a execução falha sem produzir inventário incompleto como se fosse sucesso.

## Segurança de publicação

- **Não** execute automaticamente este script sobre laboratórios ou arquivos privados com a finalidade de publicar o resultado: até metadados (nomes de arquivos e hashes) podem ser sensíveis.
- **Não** inclua arquivos vivos de `~/.local/state/arandu-seed`, `seed_id`, backups, certificados, chaves ou históricos individuais.
- Para comparar artefatos privados, faça a análise em ambiente restrito e publique somente conclusões autorizadas e revisadas.
- Não configure upload automático do JSON a partir de checkouts privados.

## Critérios A0.6

1. Inventário determinístico dos artefatos ontológicos públicos existentes.
2. Identificação humana da origem, versão, integridade e precedência dos manifestos.
3. Distinção documentada entre material canônico, histórico e laboratório.
4. Só depois, mapeamento de classes e propriedades com referência exata e testes de consistência.

## Estado

Ferramenta proposta para inventário local no repositório público. **A ontologia canônica integral permanece não verificada**. Não há autorização para importar ou alterar a Semente oficial.
