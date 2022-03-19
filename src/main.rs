use std::collections::HashMap;
use std::time::Instant;

fn solve(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let mut c = Vec::new();
    let mut n_set: HashMap<i64, i64> = HashMap::new();

    for num in b {
        let count = n_set.entry(num).or_insert(0);
        *count += 1;
    }
    for num in a {
        let count = n_set.entry(num).or_insert(0);
        if !is_prime(&count) {
            c.push(num);
        }
    }
    return c;
}
fn is_prime(number: &i64) -> bool {
    if number <= &1 {
        return false;
    }
    let stop = ((*number as f64).sqrt() + 1.0) as i64;
    for i in 2..stop {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
fn main() {
    let a = vec![2, 3, 9, 2, 5, 1, 3, 7, 10];
    let b = vec![2, 1, 3, 4, 3, 10, 6, 6, 1, 7, 10, 10, 10];

    let before = Instant::now();
    let c = solve(a, b);

    // C=[2,9,2,5,7,10]
    println!("C: {:?}", c);

    println!("Elapsed time: {:.2?}", before.elapsed());
}
