# ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos

## 1. Sobre o projeto

O ConectaStore é um sistema desenvolvido em Rust para demonstrar como estruturas de grafos podem ser utilizadas para relacionar produtos e gerar recomendações em uma loja virtual.

O projeto utiliza conexões entre produtos para identificar itens relacionados, facilitando a demonstração de recomendações baseadas em relacionamentos.

## 2. Objetivos

- Representar relações entre produtos utilizando grafos.
- Cadastrar e consultar informações de produtos.
- Identificar produtos relacionados por meio de conexões.
- Utilizar busca em largura (BFS) para percorrer o grafo.
- Evitar recomendações duplicadas durante a busca.
- Avaliar o desempenho do algoritmo em diferentes volumes de dados.

## 3. Tecnologias utilizadas

- **Linguagem:** Rust
- **Gerenciador de pacotes e compilação:** Cargo
- **Estruturas de dados:** HashMap, Vec, HashSet e VecDeque
- **Testes:** testes unitários e de integração
- **Medição de desempenho:** biblioteca padrão `std::time::Instant`

## 4. Modelo do grafo

Cada produto é identificado por um ID. As conexões representam relações entre produtos.

O sistema utiliza uma lista de adjacência implementada com `HashMap<u32, Vec<u32>>`. As conexões são bidirecionais, permitindo percorrer as relações nos dois sentidos.

A busca em largura (BFS) utiliza uma fila (`VecDeque`) e um conjunto de visitados (`HashSet`) para controlar a exploração e evitar revisitar os mesmos produtos.

## 5. Estrutura do projeto

```text
new_megastore/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── benchmark.rs
├── tests/
│   └── integracao.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

## 6. Como executar

É necessário ter o Rust e o Cargo instalados.

No terminal, dentro da pasta do projeto, execute:

```bash
cargo run
```

Para executar os testes:

```bash
cargo test
```

O programa demonstra consultas de produtos, relacionamentos e recomendações, além de realizar medições de tempo para diferentes quantidades de produtos.

## 7. Testes

O projeto possui testes unitários e de integração para verificar se:

- As recomendações incluem produtos diretamente conectados.
- Produtos conectados apenas em níveis mais distantes não aparecem como recomendações diretas.
- O algoritmo respeita a lógica de recomendação implementada.

Os testes são executados com `cargo test`.

## 8. Análise de desempenho

O projeto possui um módulo de benchmark que mede o tempo de execução do algoritmo de recomendação em grafos de diferentes tamanhos.

Em uma execução de teste, foram observados os seguintes resultados:

| Quantidade de produtos | Recomendações diretas | Tempo observado |
|---:|---:|---:|
| 100 | 1 | 3,1 µs |
| 1.000 | 1 | 3 µs |
| 10.000 | 1 | 5,5 µs |

Esses valores são resultados de uma execução específica e podem variar conforme o computador, a carga do sistema e a estrutura do grafo. Como o algoritmo retorna apenas os vizinhos diretos do produto inicial, ele evita explorar conexões de níveis mais distantes.

## 9. Limitações e melhorias futuras

- Implementar cadastro de produtos por meio de entrada do usuário.
- Criar recomendações personalizadas a partir do histórico de clientes.
- Adicionar pesos às conexões para representar avaliações ou frequência de compras.
- Ampliar os testes de desempenho com diferentes topologias e densidades de grafo.
- Melhorar a interface de utilização e a apresentação dos resultados.

## 10. Autoria e links

**Projeto acadêmico:** ConectaStore — Sistema de Recomendação de Produtos Baseado em Grafos.

**Repositório GitHub:** [Adicionar link após publicar o projeto]

**Vídeo de apresentação:** [Adicionar link após publicar o vídeo]

**Relatório técnico:** [Adicionar o PDF final]
