use std::collections::HashMap;

pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";
// Identifiers + literals
pub const IDENT: &'static str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &'static str = "INT"; // 1343456
                                     // Operators
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";
// Delimiters
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";
pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";
// Keywords
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";

lazy_static! {
  static ref KEYWORDS: HashMap<&'static str, TokenType> = {
    let mut keywords = HashMap::new();
    keywords.insert("let", TokenType::Let);
    keywords.insert("fn", TokenType::Function);
    keywords.insert("if", TokenType::If);
    keywords.insert("else", TokenType::Else);
    keywords.insert("return", TokenType::Return);
    keywords.insert("true", TokenType::True);
    keywords.insert("false", TokenType::False);
    keywords
  };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
  Illegal,
  EOF,           // End of File
  Ident(String), // add, foobar, x, y, ...
  Integer(u64),  // 123456
  Assign,        // =
  Equals,        // ==
  NotEquals,     // !=
  And,           // &&
  Or,            // ||
  Plus,          // +
  Minus,         // -
  Asterisk,      // *
  FWDSlash,      // /
  BackSlash,     // \
  Bang,          // !
  Carrot,        // ^
  BinaryOr,      // |
  BinaryAnd,     // &
  Pipe,          // |>
  Comma,         // ,
  Semicolon,     // ;
  LParen,        // (
  RParen,        // )
  LBrace,        // {
  RBrace,        // }
  LessThan,      // <
  GreaterThan,   // >
  Function,      // function
  Let,           // let
  If,            // if
  Else,          // else
  Return,        // return
  True,          // true
  False,         // false
}

impl TokenType {
  pub fn lookup_ident(identifier: &str) -> TokenType {
    match KEYWORDS.get(identifier) {
      Some(token) => (*token).clone(),
      _ => TokenType::Ident(String::from(identifier)),
    }
  }
}
