// Não teste com n > 40 — o tempo de execução cresce muito.
pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casos_base() {
        assert_eq!(fibonacci_recursivo(0), 0);
        assert_eq!(fibonacci_recursivo(1), 1);
    }

    #[test]
    fn sequencia_conhecida() {
        let esperado = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34];
        for (n, &fib) in esperado.iter().enumerate() {
            assert_eq!(fibonacci_recursivo(n as u64), fib);
        }
    }

    #[test]
    fn fib_10() {
        assert_eq!(fibonacci_recursivo(10), 55);
    }
}
