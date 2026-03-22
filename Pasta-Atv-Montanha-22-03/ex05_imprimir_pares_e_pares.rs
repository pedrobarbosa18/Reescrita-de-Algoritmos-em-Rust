pub fn imprimir_pares_e_pares(lista: &[i32]) {
    // Bloco 1 — O(n)
    for &x in lista {
        println!("{}", x);
    }

    // Bloco 2 — O(n²)
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executa_sem_panico() {
        imprimir_pares_e_pares(&[1, 2, 3]);
    }

    #[test]
    fn lista_vazia_sem_panico() {
        imprimir_pares_e_pares(&[]);
    }
}
