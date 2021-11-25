use std::time::Instant;

use logos_tokenizer::Tokenizer;

fn main() {
  let css = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  let processor = Tokenizer::new(css, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  println!("rust: tokenizer/small(7K)\t\t: {:?}", end);

  let css = include_str!("../../../assets/bootstrap.css");
  let start = Instant::now();
  let processor = Tokenizer::new(css, false);
  while !processor.end_of_file() {
    processor.next_token(false);
  }
  let end = start.elapsed();
  println!("rust: tokenizer/fairly_large(201K)\t: {:?}", end);
}
