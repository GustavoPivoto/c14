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