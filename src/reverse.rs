#[cfg(test)]
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for i in 0..xs.len() {
        rev.insert(0, xs[i].clone())
    }
    rev
}

fn wrong_reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec!();
    for i in 1..xs.len() {
        rev.insert(0, xs[i].clone())
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;
    
    quickcheck! {
        fn check1(xs: Vec<u32>) -> bool {
            xs == reverse(&reverse(&xs))
        }
    }

    quickcheck! {
        fn check2(xs: Vec<u32>) -> bool {
            xs == wrong_reverse(&wrong_reverse(&xs))
        }
    }
}
