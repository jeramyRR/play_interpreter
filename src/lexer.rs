use super::token::TokenType;

#[derive(Debug)]
pub struct Lexer {
  input: String,
  position: usize,      // points to the current char in input
  read_position: usize, // current reading position in input (after current char)
  ch: Option<char>,     // current char under examination
}

impl Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      input: String::from(input),
      position: 0,
      read_position: 0,
      ch: input.chars().nth(0),
    };

    lexer.advance_read_position();
    lexer
  }

  pub fn advance_read_position(&mut self) {
    let read_pos = self.read_position;

    if read_pos >= self.input.len() {
      self.ch = None;
    } else {
      self.ch = self.input.chars().nth(read_pos);
    }

    self.position = read_pos;
    self.read_position = read_pos + 1;
  }

  pub fn next_token(&mut self) -> TokenType {
    match self.ch {
      Some('=') => TokenType::Equals,
      _ => TokenType::Illegal,
    }
  }
}

#[test]
pub fn test_next_token() {
  let input: &str = "=+(){},;";

  let mut lexer: Lexer = Lexer::new(input);

  assert_eq!(input, "=+(){},;");
}

#[test]
pub fn test_read_char() {
  let input: &str = "=+(){},;";
  let mut lexer: Lexer = Lexer::new(input);

  let expected_read_pos_after_one_read = 2;

  lexer.advance_read_position();

  let actual_read_pos_after_one_read = lexer.read_position;

  assert_eq!(
    actual_read_pos_after_one_read,
    expected_read_pos_after_one_read
  );
}
