///use rand;
use rand::Rng;
use std::env;

/// Sequentially multiply A and B square matrices.
pub fn simple_multiply_a_b(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if b.len() == 0 {
        return vec![];
    } else {
        if b[0].len() == 0 {
            return vec![];
        }
    }

    if a.len() == 0 {
        return vec![];
    } else {
        if a[0].len() == 0 {
            return vec![];
        }
    }

    // Assert that a and b are square matrices of the same size.
    assert_eq!(a.len(), a[0].len());
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), b[0].len());

    let n = b.len();

    let mut c = vec![vec![0.0_f64; n]; n];

    for j in 0..n {
        for i in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn random_matrix(n: usize) -> Vec<Vec<f64>> {
        let mut rng = rand::thread_rng();
        let mut mat = vec![vec![0.0f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                mat[i][j] = rng.gen::<f64>();
            }
        }
        mat
}

fn print_2d(m: &[Vec<f64>], n: usize) {
    for i in 0..n {
        for j in 0..n {
            print!("{} ", m[i][j]);
        }
        println!();
    }
     println!();

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<usize>().unwrap();
    let a  = random_matrix(n);
    let b = random_matrix(n);
    let c = simple_multiply_a_b(&a, &b);
    print_2d(&a, n);
    print_2d(&b, n);
    print_2d(&c, n);
    
}
