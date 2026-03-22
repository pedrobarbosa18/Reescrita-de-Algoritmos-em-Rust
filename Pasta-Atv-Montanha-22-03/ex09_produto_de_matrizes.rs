pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produto_2x2() {
        let a = vec![vec![1i64, 2], vec![3, 4]];
        let b = vec![vec![5i64, 6], vec![7, 8]];
        let c = produto_de_matrizes(&a, &b);
        assert_eq!(c[0], vec![19, 22]);
        assert_eq!(c[1], vec![43, 50]);
    }

    #[test]
    fn produto_com_identidade() {
        let a = vec![vec![3i64, 7], vec![2, 5]];
        let identidade = vec![vec![1i64, 0], vec![0, 1]];
        let c = produto_de_matrizes(&a, &identidade);
        assert_eq!(c, a);
    }

    #[test]
    fn produto_com_zeros() {
        let a = vec![vec![9i64, 8], vec![7, 6]];
        let zeros = vec![vec![0i64, 0], vec![0, 0]];
        let c = produto_de_matrizes(&a, &zeros);
        assert_eq!(c, zeros);
    }
}
