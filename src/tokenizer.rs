use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum TokenizationError {
    #[error("Unexpected character at line {line}, column {column}: {character}")]
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },
    #[error("Unexpected escape sequence at line {line}, column {column}: {character}")]
    UnexpectedEscapeSequence {
        character: char,
        line: usize,
        column: usize,
    },
    #[error("Unterminated string literal at line {line}, column {column}")]
    UnterminatedStringLiteral {
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword, usize, usize),
    Identifier(String, usize, usize),
    Punctuation(char, usize, usize),
    StringLiteral(String, usize, usize),
}

impl Token {
    fn to_string(&self) -> String {
        match self {
            Token::Keyword(keyword, _, _) => format!("Keyword: {:?}", keyword),
            Token::Identifier(identifier, _, _) => format!("Identifier: {}", identifier),
            Token::Punctuation(punct, _, _) => format!("Punctuation: '{}'", punct),
            Token::StringLiteral(string, _, _) => format!("String Literal: \"{}\"", string),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    SELECT,
    FROM,
    WHERE,
    ALL,
}

impl Keyword {
    pub fn to_string(&self) -> String {
        match self {
            Keyword::SELECT => String::from("SELECT"),
            Keyword::FROM => String::from("FROM"),
            Keyword::WHERE => String::from("WHERE"),
            Keyword::ALL => String::from("ALL"),
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut in_string = false;
    let mut escape = false;
    let mut line_number = 1;
    let mut column = 0;

    for c in input.chars() {
        match c {
            '\n' => {
                line_number += 1;
                column = 1;
            }
            _ => {
                column += 1;
            }
        }

        if escape {
            match c {
                'n' => buffer.push('\n'),
                't' => buffer.push('\t'),
                '\\' => buffer.push('\\'),
                '"' => buffer.push('"'),
                _ => return Err(TokenizationError::UnexpectedEscapeSequence { character: c, line: line_number, column }),
            }
            escape = false;
        } else if in_string {
            match c {
                '\\' => {
                    escape = true;
                }
                '"' => {
                    tokens.push(Token::StringLiteral(
                        buffer.clone(),
                        line_number,
                        column - buffer.len(),
                    ));
                    buffer.clear();
                    in_string = false;
                }
                _ => buffer.push(c),
            }
        } else {
            if c.is_whitespace() {
                if !buffer.is_empty() {
                    match buffer.to_uppercase().as_str() {
                        "SELECT" => tokens.push(Token::Keyword(Keyword::SELECT, line_number, column - buffer.len())),
                        "FROM" => tokens.push(Token::Keyword(Keyword::FROM, line_number, column - buffer.len())),
                        "WHERE" => tokens.push(Token::Keyword(Keyword::WHERE, line_number, column - buffer.len())),
                        "*" => tokens.push(Token::Keyword(Keyword::ALL, line_number, column - buffer.len())),
                        _ => tokens.push(Token::Identifier(buffer.clone(), line_number, column - buffer.len())),
                    }
                    buffer.clear();
                }
            } else {
                match c {
                    '"' => {
                        in_string = true;
                    }
                    ',' | '.' => {
                        if !buffer.is_empty() {
                            tokens.push(Token::Identifier(
                                buffer.clone(),
                                line_number,
                                column - buffer.len(),
                            ));
                            buffer.clear();
                        }
                        tokens.push(Token::Punctuation(c, line_number, column));
                    }
                    _ => buffer.push(c),
                }
            }
        }
    }

    if escape {
        return Err(TokenizationError::UnterminatedStringLiteral { line: line_number, column });
    }

    if !buffer.is_empty() {
        if in_string {
            return Err(TokenizationError::UnterminatedStringLiteral { line: line_number, column });
        } else {
            tokens.push(Token::Identifier(buffer.clone(), line_number, column - buffer.len()));
        }
    }

    Ok(tokens)
}


#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_tokenization_simple() {
        let input = "SELECT * FROM \"examples/posts/hello-world.md\"";
        let expected_tokens = vec![
            Token::Keyword(Keyword::SELECT, 1, 1),
            Token::Keyword(Keyword::ALL, 1, 8),
            Token::Keyword(Keyword::FROM, 1, 10),
            Token::StringLiteral(String::from("examples/posts/hello-world.md"), 1, 16),
        ];

        assert_eq!(tokenize(input).unwrap(), expected_tokens);
    }
}
