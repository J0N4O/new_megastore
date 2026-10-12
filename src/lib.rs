use std::collections::{HashMap, HashSet, VecDeque};

pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub categoria: String,
    pub preco: f64,
}

pub struct Grafo {
    pub conexoes: HashMap<u32, Vec<u32>>,
}

impl Grafo {
    pub fn adicionar_conexao(&mut self, origem: u32, destino: u32) {
        if origem == destino {
            return;
        }

        let conexoes_origem = self.conexoes.entry(origem).or_insert(Vec::new());

        if !conexoes_origem.contains(&destino) {
            conexoes_origem.push(destino);
        }

        let conexoes_destino = self.conexoes.entry(destino).or_insert(Vec::new());

        if !conexoes_destino.contains(&origem) {
            conexoes_destino.push(origem);
        }
    }
}


pub fn recomendar_produtos(
    grafo: &Grafo,
    inicio: u32,
) -> Vec<u32> {
    let mut visitados: HashSet<u32> = HashSet::new();
    let mut recomendacoes: Vec<u32> = Vec::new();

    visitados.insert(inicio);

    let mut fila: VecDeque<(u32, usize)> = VecDeque::new();
    fila.push_back((inicio, 0));

    while let Some((atual, distancia)) = fila.pop_front() {
        if distancia >= 1 {
            continue;
        }

        if let Some(conexoes) = grafo.conexoes.get(&atual) {
            for vizinho in conexoes {
                if !visitados.contains(vizinho) {
                    visitados.insert(*vizinho);
                    fila.push_back((*vizinho, distancia + 1));

                    if distancia + 1 == 1 {
                        recomendacoes.push(*vizinho);
                    }
                }
            }
        }
    }

    recomendacoes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nao_deve_duplicar_conexoes() {
        let mut grafo = Grafo {
            conexoes: HashMap::new(),
        };

        grafo.adicionar_conexao(1, 2);
        grafo.adicionar_conexao(1, 2);

        assert_eq!(grafo.conexoes.get(&1).unwrap().len(), 1);
        assert_eq!(grafo.conexoes.get(&2).unwrap().len(), 1);
    }

    #[test]
    fn nao_deve_conectar_produto_a_ele_mesmo() {
        let mut grafo = Grafo {
            conexoes: HashMap::new(),
        };

        grafo.adicionar_conexao(1, 1);

        assert!(grafo.conexoes.is_empty());
    }
}
