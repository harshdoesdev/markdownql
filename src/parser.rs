use markdownql::tokenizer::{Token, Keyword};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Headings,
    Paragraphs,
    Text(String),
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub elements: Vec<Element>,
    pub file_path: String,
    pub condition: Option<String>,
}


pub fn parse_query(tokens: &[Token]) -> Result<Query, ParseError> {
    let mut elements = Vec::new();
    let mut file_path = String::new();
    let mut condition: Option<String> = None;

    let mut tokens_iter = tokens.iter().peekable();

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Keyword(keyword, _, _) => match keyword {
                Keyword::SELECT => {
                    elements.extend(parse_select(&mut tokens_iter)?);
                }
                Keyword::FROM => {
                    file_path = parse_file_path(&mut tokens_iter)?;
                }
                Keyword::WHERE => {
                    condition = parse_condition(&mut tokens_iter)?;
                }
                _ => {}
            },
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    Ok(Query {
        elements,
        file_path,
        condition,
    })
}

fn parse_select(tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Vec<Element>, ParseError> {
    let mut elements = Vec::new();

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Keyword(Keyword::ALL, _, _) => {
                elements.push(Element::All);
            }
            Token::Identifier(identifier, _, _) => {
                elements.push(match identifier.as_str() {
                    "headings" => Element::Headings,
                    "paragraphs" => Element::Paragraphs,
                    "text" => {
                        if let Some(Token::StringLiteral(text, _, _)) = tokens_iter.next() {
                            Element::Text(text.clone())
                        } else {
                            return Err(ParseError::UnexpectedEndOfInput);
                        }
                    }
                    _ => Element::Text(identifier.clone()),
                });
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }

        if let Some(next_token) = tokens_iter.peek() {
            match next_token {
                Token::Punctuation(',', _, _) => {
                    // Consume the comma
                    tokens_iter.next();
                }
                _ => break,
            }
        }
    }

    Ok(elements)
}

fn parse_file_path(tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<String, ParseError> {
    if let Some(token) = tokens_iter.next() {
        match token {
            Token::StringLiteral(file_path, _, _) => Ok(file_path.clone()),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    } else {
        Err(ParseError::UnexpectedEndOfInput)
    }
}

fn parse_condition(_tokens_iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Option<String>, ParseError> {
    unimplemented!("not yet supported")
}


#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_query_simple() {
        let tokens = vec![
            Token::Keyword(Keyword::SELECT, 1, 1),
            Token::Keyword(Keyword::ALL, 1, 8),
            Token::Keyword(Keyword::FROM, 1, 10),
            Token::StringLiteral(String::from("examples/posts/hello-world.md"), 1, 16),
        ];

        let expected_query = Query {
            elements: vec![Element::All],
            file_path: String::from("examples/posts/hello-world.md"),
            condition: None,
        };

        assert_eq!(parse_query(&tokens).unwrap(), expected_query);
    }
}
