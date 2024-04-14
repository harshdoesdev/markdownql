use thiserror::Error;

use markdown::mdast::{Node, Heading, Paragraph};
use markdown::to_mdast;
use markdown::ParseOptions;

use markdownql::parser::{Query, Element};

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Error reading file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Error parsing Markdown: {0}")]
    MarkdownParseError(String),

    #[error("Invalid element: {0}")]
    InvalidElement(String),
}

#[derive(Debug)]
pub struct QueryResult {
    pub headings: Vec<String>,
    pub paragraphs: Vec<String>,
    pub matching_text: Vec<String>,
}

pub struct MarkdownQueryExecutor;

impl MarkdownQueryExecutor {
    pub fn execute_query(query: Query) -> Result<QueryResult, ExecutorError> {
        // Construct file path
        let file_path = std::env::current_dir()?.join(&query.file_path);

        // Read the Markdown file
        let markdown_content = std::fs::read_to_string(&file_path)?;

        // Parse Markdown content into AST
        let ast = to_mdast(&markdown_content, &ParseOptions::gfm()).map_err(|e| ExecutorError::MarkdownParseError(e))?;

        let mut headings = Vec::new();
        let mut paragraphs = Vec::new();
        let mut matching_text = Vec::new();

        for element in &query.elements {
            match element {
                Element::Headings => {
                    headings.extend(Self::extract_headings(&ast));
                }
                Element::Paragraphs => {
                    paragraphs.extend(Self::extract_paragraphs(&ast));
                }
                Element::Text(text) => {
                    matching_text.extend(Self::extract_matching_text(&ast, text));
                }
                Element::All => {
                    headings.extend(Self::extract_headings(&ast));
                    paragraphs.extend(Self::extract_paragraphs(&ast));
                }
            }
        }

        Ok(QueryResult {
            headings,
            paragraphs,
            matching_text,
        })
    }

    fn extract_headings(root: &Node) -> Vec<String> {
        let mut headings = Vec::new();
        Self::extract_headings_recursive(root, &mut headings);
        headings
    }

    fn extract_headings_recursive(node: &Node, headings: &mut Vec<String>) {
        match node {
            Node::Heading(heading) => {
                headings.push(Self::heading_to_string(heading));
            }
            Node::Root(root) => {
                for child in &root.children {
                    Self::extract_headings_recursive(child, headings);
                }
            }
            _ => {}
        }
    }

    fn extract_paragraphs(root: &Node) -> Vec<String> {
        let mut paragraphs = Vec::new();
        Self::extract_paragraphs_recursive(root, &mut paragraphs);
        paragraphs
    }

    fn extract_paragraphs_recursive(node: &Node, paragraphs: &mut Vec<String>) {
        match node {
            Node::Paragraph(paragraph) => {
                paragraphs.push(Self::paragraph_to_string(paragraph));
            }
            Node::Root(root) => {
                for child in &root.children {
                    Self::extract_paragraphs_recursive(child, paragraphs);
                }
            }
            _ => {}
        }
    }

    fn extract_matching_text(root: &Node, text: &str) -> Vec<String> {
        let mut matching_text = Vec::new();
        Self::extract_matching_text_recursive(root, text, &mut matching_text);
        matching_text
    }

    fn extract_matching_text_recursive(node: &Node, text: &str, matching_text: &mut Vec<String>) {
        match node {
            Node::Text(text_node) => {
                if text_node.value.contains(text) {
                    matching_text.push(text_node.value.clone());
                }
            }
            Node::Root(root) => {
                for child in &root.children {
                    Self::extract_matching_text_recursive(child, text, matching_text);
                }
            }
            _ => {}
        }
    }
    
    fn heading_to_string(heading: &Heading) -> String {
        let mut result = String::new();
        for child in &heading.children {
            if let Node::Text(text_node) = child {
                result.push_str(&text_node.value);
            }
        }
        result
    }
    
    fn paragraph_to_string(paragraph: &Paragraph) -> String {
        let mut result = String::new();
        for child in &paragraph.children {
            if let Node::Text(text_node) = child {
                result.push_str(&text_node.value);
            }
        }
        result
    }
}
