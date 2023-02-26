use std::{fs::File, io::Write};

use vuur_compile::{compile, disassemble};

#[test]
fn test_basic_arithmetic() {
    const SRC: &[&str] = &["1 + 2"];

    for source in SRC {
        let module = vuur_parse::parse_str(source).unwrap();

        let chunk = compile(&module).unwrap_or_else(|err| panic!("bytecode generation failed for module: {}", err));

        let mut buf = String::new();
        disassemble(&mut buf, &chunk).expect("failed to disassemble bytecode chunk");
        println!("{}", buf);

        let mut file = File::create("./test_chunk.bin").unwrap();
        let mut buf = Vec::new();
        chunk.encode(&mut buf).unwrap();
        file.write(&buf).expect("write chunk binary file");
        file.flush().expect("flush chunk binary file");
        drop(file);
    }
}
