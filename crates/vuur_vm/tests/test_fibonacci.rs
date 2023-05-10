#[test]
fn test_vm_fibonacci() {
    let module = vuur_parse::parse_str(include_str!("test_fibonacci.vu")).expect("parsing fibonacci");
    let chunk = vuur_compile::compile(&module).expect("compiling module");

    let mut buf = String::new();
    vuur_compile::disassemble(&mut buf, &chunk).expect("disassemble fibonacci");
    println!("{buf}");

    let mut vm = vuur_vm::VM::new();
    let result = vm.run(&chunk);
    println!("result: {result:?}");
}
