use ferris_says::say;
use std::env;
use std::io::{stdout, BufWriter};

fn main() {
    // Obtém a mensagem dos argumentos de linha de comando ou usa padrão
    let message = env::args().nth(1).unwrap_or_else(|| String::from("Hello fellow Rustaceans!"));
    let width = message.chars().count();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // Tratamento de erro profissional
    if let Err(e) = say(&message, width, &mut writer) {
        eprintln!("Erro ao exibir mensagem: {}", e);
    }
}

// Funções para teste
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

pub fn biggest(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn smallest(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn square(n: i32) -> i32 {
    n * n
}

pub fn cube(n: i32) -> i32 {
    n * n * n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_pass() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_fail() {
        assert_eq!(add(2, 2), 5); // Este teste vai falhar
    }

    #[test]
    fn test_subtract_pass() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_subtract_fail() {
        assert_eq!(subtract(5, 3), 3); // Este teste vai fal
    }

    #[test]
    fn test_multiply_pass() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_multiply_fail() {
        assert_eq!(multiply(2, 3), 5); // Este teste vai fal
    }

    #[test]
    fn test_divide_pass() {
        assert_eq!(divide(6, 3), Some(2));
    }

    #[test]
    fn test_divide_fail() {
    assert_eq!(divide(6, 3), Some(3)); // Este teste vai
    }

    #[test]
    fn test_is_even_pass() {
        assert!(is_even(4));
    }

    #[test]
    fn test_is_even_fail() {
        assert!(is_even(5)); // Este teste vai falhar
    }

    #[test]
    fn test_is_odd_pass() {
        assert!(is_odd(5));
    }

    #[test]
    fn test_is_odd_fail() {
        assert!(is_odd(4)); // Este teste vai falhar
    }

    #[test]
    fn test_is_biggest_pass() {
        assert_eq!(biggest(5, 3), 5);
    }

    #[test]
    fn test_is_biggest_fail() {
        assert_eq!(biggest(2, 3), 2); // Este teste vai falhar
    }

    #[test]
    fn test_is_smallest_pass() {
        assert_eq!(smallest(2, 3), 2);
    }

    #[test]
    fn test_is_smallest_fail() {
        assert_eq!(smallest(5, 3), 5); // Este teste vai
    }

    #[test]
    fn test_square_pass() {
    assert_eq!(square(4), 16);
    }

    #[test]
    fn test_square_fail() {
        assert_eq!(square(5), 20); // Este teste vai falhar
    }

    #[test]
    fn test_cube_pass() {
        assert_eq!(cube(3), 27);
    }

    #[test]
    fn test_cube_fail() {
        assert_eq!(cube(3), 30); // Este teste vai falhar
    }
}