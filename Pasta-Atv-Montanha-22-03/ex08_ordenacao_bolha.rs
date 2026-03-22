pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_desordenada() {
        let mut lista = vec![64, 34, 25, 12, 22, 11, 90];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn lista_ja_ordenada() {
        let mut lista = vec![1, 2, 3, 4, 5];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_invertida() {
        let mut lista = vec![5, 4, 3, 2, 1];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_vazia() {
        let mut lista: Vec<i32> = vec![];
        ordenacao_bolha(&mut lista);
        assert_eq!(lista, vec![]);
    }
}
