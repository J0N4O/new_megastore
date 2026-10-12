use new_megastore::{Grafo, recomendar_produtos};

#[test]
fn teste_integracao_recomendacoes() {
    let mut grafo = Grafo {
        conexoes: std::collections::HashMap::new(),
    };

    grafo.adicionar_conexao(1, 2);
    grafo.adicionar_conexao(1, 3);
    grafo.adicionar_conexao(1, 4);
    grafo.adicionar_conexao(2, 5);

    let recomendacoes = recomendar_produtos(&grafo, 1);

    assert!(recomendacoes.contains(&2));
    assert!(recomendacoes.contains(&3));
    assert!(recomendacoes.contains(&4));
    assert!(!recomendacoes.contains(&5));
}