use std::collections::{HashMap, HashSet, VecDeque};
use new_megastore::{Grafo, Produto, recomendar_produtos};
use std::time::Instant;
mod benchmark;

fn buscar_por_categoria(produtos: &HashMap<u32, Produto>, categoria: &str) {
    println!("Produtos da categoria {}:", categoria);

    for produto in produtos.values() {
        if produto.categoria == categoria {
            println!("- {}", produto.nome);
        }
    }
}

fn main() {
        let mut produtos: HashMap<u32, Produto> = HashMap::new();

        let mut grafo = Grafo {
            conexoes: HashMap::new(),
        };

        let mut visitados: HashSet<u32> = HashSet::new();
        visitados.insert(1);

        let mut fila: VecDeque<u32> = VecDeque::new();
        fila.push_back(1);

        produtos.insert(
            1,
            Produto{
                id: 1,
                nome: String::from("Notebook"),
                categoria: String::from("Eletronicos"),
                preco: 3500.00,
            },
        );

        produtos.insert(
            2,
            Produto{
                id: 2,
                nome: String::from("Mouse"),
                categoria: String::from("Perifericos"),
                preco: 120.00,
            },
        );

        produtos.insert(
            3,
            Produto{
                id: 3,
                nome: String::from("Teclado"),
                categoria: String::from("Perifericos"),
                preco: 180.00,
            },
        );

        produtos.insert(
            4,
            Produto {
                id: 4,
                nome: String::from("Headset"),
                categoria: String::from("Perifericos"),
                preco: 250.00,
            },
        );

        produtos.insert(
            5,
            Produto {
                id: 5,
                nome: String::from("Mousepad"),
                categoria: String::from("Perifericos"),
                preco: 80.00,
            },
        );

        grafo.adicionar_conexao(1, 2);
        grafo.adicionar_conexao(1, 3);
        grafo.adicionar_conexao(1, 4);
        grafo.adicionar_conexao(2, 3);
        grafo.adicionar_conexao(2, 5);

        while let Some(atual) = fila.pop_front() {
            println!("Processando: {}", atual);

            if let Some(conexoes) = grafo.conexoes.get(&atual) {
                for vizinho in conexoes {
                    println!("Encontrei conexao com: {}", vizinho);

                    if !visitados.contains(vizinho) {
                        visitados.insert(*vizinho);
                        fila.push_back(*vizinho);
                    }
                }
            }
        }

        if let Some(conexoes) = grafo.conexoes.get(&1) {
            println!("Produtos relacionados ao Notebook:");

            for id in conexoes {
                if let Some(produto) = produtos.get(id) {
                    println!("- {}", produto.nome);
                }
            }
        }

        if let Some(conexoes) = grafo.conexoes.get(&2) {
            println!("Produtos relacionados ao Mouse:");

            for id in conexoes {
                if let Some(produto) = produtos.get(id) {
                    println!("- {}", produto.nome);
                }
            }
        }

        if let Some(produto) = produtos.get(&2) {
            println!("Produto encontrado: {}", produto.nome);
            println!("ID: {}", produto.id);
            println!("Categoria: {}", produto.categoria);
            println!("Preço: R$ {:.2}", produto.preco);
        }

    buscar_por_categoria(&produtos, "Perifericos");

    let cronometro = Instant::now();

    let recomendacoes = recomendar_produtos(&grafo, 1);

    let duracao = cronometro.elapsed();

    println!("Tempo das recomendações: {:?}", duracao);
    
    for id in &recomendacoes {
        if let Some(produto) = produtos.get(id) {
            println!("Nome dos produtos: {}", produto.nome);
        }
    }

    benchmark::executar();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_recomendacoes() {
        let mut grafo = Grafo {
            conexoes: HashMap::new(),
        };

        grafo.adicionar_conexao(1, 2);
        grafo.adicionar_conexao(1, 3);
        grafo.adicionar_conexao(1, 4);
        grafo.adicionar_conexao(2, 5);

        let recomendacoes = recomendar_produtos(&grafo, 1);
        assert!(recomendacoes.contains(&2));
        assert!(!recomendacoes.contains(&5));
    }
}