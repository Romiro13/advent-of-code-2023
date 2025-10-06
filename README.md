# 🎄 Advent of Code 2023

![Rust](https://img.shields.io/badge/Rust-1.72.1-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

## 📖 Sobre o Projeto

Soluções em Rust para os desafios do [Advent of Code 2023](https://adventofcode.com/2023/about). O Advent of Code é um calendário do advento com pequenos desafios de programação que podem ser resolvidos em qualquer linguagem.

Este repositório contém implementações organizadas e testadas para cada dia do evento, com foco em código limpo e eficiente.

## 🚀 Começando

### Pré-requisitos

- Rust 1.72.1 ou superior
- Cargo (incluído na instalação do Rust)

### Instalação

```bash
# Clone o repositório
git clone https://github.com/Romiro13/advent-of-code-2024.git
cd advent-of-code-2024

# Execute o projeto
cargo run
```

## 🛠️ Comandos de Desenvolvimento

### Executar Soluções

```bash
# Executar todas as soluções implementadas
cargo run

# Modo watch - recompila e executa automaticamente ao detectar mudanças
cargo watch -q -c -w src/ -x "run"
```

### Testes

```bash
# Executar todos os testes com saída detalhada
cargo test -- --nocapture

# Modo watch para testes - reexecuta automaticamente
cargo watch -q -c -x "test -- --nocapture"

# Executar testes de um dia específico
cargo test day_01 -- --nocapture
cargo test day_02 -- --nocapture
```

### Instalar cargo-watch (opcional, mas recomendado)

```bash
cargo install cargo-watch
```

## 📁 Estrutura do Projeto

```
advent-of-code-2023/
├── src/
│   ├── main.rs          # Ponto de entrada do programa
│   ├── day_01.rs        # Solução do dia 1
│   ├── day_02.rs        # Solução do dia 2
│   ├── day_03.rs        # Solução do dia 3
│   └── day_04.rs        # Solução do dia 4
├── inputs/
│   ├── input_day01.txt  # Entrada do dia 1
│   ├── input_day02.txt  # Entrada do dia 2
│   ├── input_day03.txt  # Entrada do dia 3
│   └── input_day04.txt  # Entrada do dia 4
└── Cargo.toml
```

## 🎯 Progresso dos Desafios

| Dia | Parte 1 | Parte 2 | Solução                    |
| --- | ------- | ------- | -------------------------- |
| 01  | ⭐      | ⭐      | [day_01.rs](src/day_01.rs) |
| 02  | ⭐      | ⭐      | [day_02.rs](src/day_02.rs) |
| 03  | ⭐      | ⭐      | [day_03.rs](src/day_03.rs) |
| 04  | ⭐      | ⭐      | [day_04.rs](src/day_04.rs) |

## 📦 Dependências

- **[lazy-regex](https://crates.io/crates/lazy-regex)** - Utilitários para trabalhar com expressões regulares
- **[itertools](https://crates.io/crates/itertools)** - Funcionalidades extras para iteradores
- **[anyhow](https://crates.io/crates/anyhow)** - Tratamento de erros simplificado

## 🧪 Testes

Cada módulo de dia inclui testes unitários com exemplos fornecidos pelo Advent of Code. Os testes garantem que as soluções funcionam corretamente antes de processar as entradas completas.

## 📝 Licença

Este projeto está licenciado sob os termos das licenças MIT OU Apache-2.0. Veja os arquivos de licença para mais detalhes.

## 👤 Autor

**Antonio Albuquerque Loiola**

- Email: romiro@hotmail.com.br
- GitHub: [@Romiro13](https://github.com/Romiro13)

## 🔗 Links Úteis

- [Advent of Code 2023](https://adventofcode.com/2023)
- [Documentação do Rust](https://doc.rust-lang.org/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
