use markdownql::{
    executor::MarkdownQueryExecutor,
    parser::parse_query,
    tokenizer::tokenize,
};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("markdownql>> ");
        match readline {
            Ok(line) => {
                if matches!(line.trim(), "exit" | "quit") {
                    break; // Exit loop if the command is "exit" or "quit"
                }
                let _ = rl.add_history_entry(line.as_str());
                match tokenize(&line) {
                    Ok(tokens) => {
                        match parse_query(&tokens) {
                            Ok(query) => {
                                let result = MarkdownQueryExecutor::execute_query(query);
                                match result {
                                    Ok(result) => println!("{:#?}", result),
                                    Err(e) => eprintln!("Query execution error: {}", e),
                                }
                            }
                            Err(e) => eprintln!("Error parsing query: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Tokenization error: {}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}
