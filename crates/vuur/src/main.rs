//! Read-eval-print loop.
use std::io::Write;
use vuur_lexer::Lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vuur v{}", env!("CARGO_PKG_VERSION"));
    run_repl()?;

    Ok(())
}

fn run_repl() -> std::io::Result<()> {
    let mut input = String::new();

    loop {
        input.clear();

        print!(">>> ");
        std::io::stdout().flush()?; // flush prompt
        std::io::stdin().read_line(&mut input)?;

        // stdin reads up to and including newline or EOF
        let trimmed = input.trim();

        for token in Lexer::from_str(trimmed).into_iter() {
            println!("{:?}", token.kind);
        }
    }
}
