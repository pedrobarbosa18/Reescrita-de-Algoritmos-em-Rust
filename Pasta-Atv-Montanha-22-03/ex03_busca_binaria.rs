// A lista deve estar ordenada em ordem crescente.
pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = esquerda + (direita - esquerda) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encontra_elemento() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 7), Some(3));
    }

    #[test]
    fn elemento_ausente() {
        assert_eq!(busca_binaria(&[1, 3, 5, 7, 9], 4), None);
    }

    #[test]
    fn primeiro_elemento() {
        assert_eq!(busca_binaria(&[2, 4, 6, 8], 2), Some(0));
    }

    #[test]
    fn ultimo_elemento() {
        assert_eq!(busca_binaria(&[2, 4, 6, 8], 8), Some(3));
    }

    #[test]
    fn lista_vazia() {
        assert_eq!(busca_binaria(&[], 1), None);
    }
}
