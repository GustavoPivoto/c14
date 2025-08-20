# hello-rust

Este projeto é um exemplo simples em Rust, criado seguindo o guia oficial de início rápido disponível em [rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started). Ele demonstra como utilizar uma dependência externa, a biblioteca [`ferris-says`](https://crates.io/crates/ferris-says), para exibir uma mensagem estilizada no terminal.

## Como executar

1. Instale o [Rust](https://www.rust-lang.org/learn/get-started) seguindo as instruções oficiais.
2. Clone este repositório e acesse a pasta `hello-rust`:
   ```sh
   git clone <url-do-repositorio>
   cd hello-rust
   ```
3. Execute o projeto com o comando:
   ```sh
   cargo run
   ```

## Sobre o código

O código principal está em `src/main.rs` e faz o seguinte:
- Importa a função `say` da biblioteca `ferris-says`.
- Cria uma mensagem personalizada.
- Calcula a largura da mensagem.
- Usa um `BufWriter` para escrever a saída no terminal.
- Exibe a mensagem com o mascote Ferris.

Exemplo de saída:
```
 _____________________________
< Hello fellow Rustaceans! >
 ---------------------------
        \
         \
          _~^~^~_
      \) /  o o  \ (/
        '_   -   _'
        / '-----' \
```

## Dependências

- [`ferris-says`](https://crates.io/crates/ferris-says) versão 0.3.2

## Referências

- Guia oficial de início rápido: [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)

## Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](../LICENSE)