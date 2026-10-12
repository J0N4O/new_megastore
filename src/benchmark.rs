
use std::time::Instant;
use new_megastore::{Grafo, recomendar_produtos};

pub fn executar() {
    let tamanhos = [100u32, 1_000, 10_000];

    for quantidade in tamanhos {
        let mut grafo = Grafo {
            conexoes: std::collections::HashMap::new(),
        };

        // Cria conexões entre os produtos.
        for id in 1..quantidade {
            grafo.adicionar_conexao(id, id + 1);
        }

        // Mede somente o tempo do algoritmo.
        let inicio = Instant::now();
        let recomendacoes = recomendar_produtos(&grafo, 1);
        let duracao = inicio.elapsed();

        println!(
            "Produtos: {} | Recomendações diretas: {} | Tempo: {:?}",
            quantidade,
            recomendacoes.len(),
            duracao
        );
    }
}
