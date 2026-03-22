pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1;
    while i < n {
        println!("{}", i);
        i *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executa_n_32() {
        potencias_de_dois(32);
    }

    #[test]
    fn executa_n_1() {
        potencias_de_dois(1);
    }

    #[test]
    fn executa_n_grande() {
        potencias_de_dois(1_000_000);
    }
}
