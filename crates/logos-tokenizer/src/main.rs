use std::time::Instant;

use logos::Logos;
use logos_tokenizer::TokenType;

fn main() {
  let css = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();

  let mut lex = TokenType::lexer("/* */");

  while let Some(t) = lex.next() {
    println!("{:?}", t);
  }
}
