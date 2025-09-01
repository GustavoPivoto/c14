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