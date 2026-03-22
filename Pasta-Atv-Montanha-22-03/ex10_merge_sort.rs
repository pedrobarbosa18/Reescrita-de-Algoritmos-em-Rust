pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let mut i = 0;
    let mut j = 0;

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let lista = vec![38, 27, 43, 3, 9, 82, 10];
        assert_eq!(merge_sort(lista), vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn lista_ja_ordenada() {
        let lista = vec![1, 2, 3, 4, 5];
        assert_eq!(merge_sort(lista), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_invertida() {
        let lista = vec![5, 4, 3, 2, 1];
        assert_eq!(merge_sort(lista), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_vazia() {
        let lista: Vec<i32> = vec![];
        assert_eq!(merge_sort(lista), vec![]);
    }

    #[test]
    fn lista_com_duplicatas() {
        let lista = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        assert_eq!(merge_sort(lista), vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
