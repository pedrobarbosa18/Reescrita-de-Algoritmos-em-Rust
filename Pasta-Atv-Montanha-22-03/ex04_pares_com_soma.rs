pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
                resultado.push((lista[i], lista[j]));
            }
        }
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pares_existentes() {
        let pares = pares_com_soma(&[1, 2, 3, 4, 5], 6);
        assert!(pares.contains(&(1, 5)));
        assert!(pares.contains(&(2, 4)));
    }

    #[test]
    fn nenhum_par() {
        let pares = pares_com_soma(&[1, 2, 3], 10);
        assert!(pares.is_empty());
    }

    #[test]
    fn lista_vazia() {
        let pares = pares_com_soma(&[], 5);
        assert!(pares.is_empty());
    }
}
