pub mod input;
mod tokenizer;

pub mod ref_ring;
pub use crate::tokenizer::*;

pub mod list;


// pub fn tokenize(input: &str) -> Vec<Token> {
//   let mut res = vec![];
//   let tokenizer = Tokenizer::new(input, false);
//   while !tokenizer.end_of_file() {
//     res.push(tokenizer.next_token(true));
//   }
//   res
// }
