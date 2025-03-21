fn main() {
    println!("{}", sum(3,2));
    println!("{}", division(6,2));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
}

#[cfg(test)]
mod tests {
    use crate::{division, sum};

    #[test]
    fn sum_test() {
        assert_eq!(sum(1, 2), 3);
    }

    #[test]
    fn division_test() {
        assert_eq!(division(6, 2), 3);
    }
}