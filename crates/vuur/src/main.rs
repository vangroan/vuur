//! Read-eval-print loop.
use std::io::Write;
use vuur_compile::disassemble;
use vuur_lexer::Lexer;
use vuur_vm::VM;

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
        std::io::stdout().flush()?; // ensure prompt shows
        std::io::stdin().read_line(&mut input)?;

        // stdin reads up to and including newline or EOF
        let trimmed = input.trim();

        for token in Lexer::from_source(trimmed).into_iter() {
            println!("{:?} '{}'", token.kind, token.fragment(trimmed));
        }

        match vuur_parse::parse_str(trimmed) {
            Ok(module) => {
                println!("-----------");
                println!("Syntax Tree");
                println!("");
                println!("{:#?}", module);

                match vuur_compile::compile(&module) {
                    Ok(chunk) => {
                        let mut buf = String::new();
                        match disassemble(&mut buf, &chunk) {
                            Ok(_) => {
                                println!("-----------");
                                println!("Disassembly");
                                println!("");
                                println!("{}", buf);
                            }
                            Err(err) => eprintln!("{}", err),
                        }

                        println!("--------");
                        println!("Evaluate");
                        println!("");
                        let mut vm = VM::new();
                        vm.run(&chunk);
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn demo_dissasm() {
    println!("----------------");
    println!("Disassemble Demo");
    println!("");

    let mut buf = String::new();
    let mut chunk = vuur_compile::Chunk::new("demo.vuur", vec![]);
    vuur_compile::write_header(&mut chunk);

    match vuur_compile::disassemble(&mut buf, &chunk) {
        Ok(_) => println!("{}", buf),
        Err(err) => eprintln!("disassm demo failed: {}", err),
    }
}
