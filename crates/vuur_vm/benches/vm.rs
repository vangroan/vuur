use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use vuur_vm::handle::Handle;
use vuur_vm::instruction_set::{Arg24, Op};
use vuur_vm::value::{Closure, ConstantId, GlobalId, LocalId, Module, Program, ScriptFunc, Value};
use vuur_vm::vm_v2::VM;

/// Create a recursive fibonacci script function.
fn fibonacci(module: Handle<Module>) -> Rc<ScriptFunc> {
    // func fib(n: Int) -> Int {
    //    if n <= 1 {
    //        return n
    //    } else {
    //        return fib(n - 1) + fib(n - 2)
    //    }
    // }
    let fib = GlobalId::new(0);
    let n = LocalId::new(0);
    let code = vec![
        Op::Load_Local(n),
        Op::I32_Const_Inline(Arg24::from_i32(1)),
        Op::I32_LessEq,
        Op::Jump_False {
            addr: Arg24::from_u32(6),
        },
        Op::Load_Local(n),
        Op::Return,
        // Setup call to fib(n)
        Op::Load_Global(fib),
        // n - 1
        Op::Load_Local(n),
        Op::I32_Const_Inline(Arg24::from_i32(1)),
        Op::I32_Sub,
        Op::Call_Closure { arity: 1 },
        // Setup call to fib(n)
        Op::Load_Global(fib),
        // n - 2
        Op::Load_Local(n),
        Op::I32_Const_Inline(Arg24::from_i32(2)),
        Op::I32_Sub,
        Op::Call_Closure { arity: 1 },
        // fib(n - 1) + fib(n - 2)
        Op::I32_Add,
        Op::Return,
    ];

    Rc::new(ScriptFunc {
        constants: vec![],
        code: code.into_boxed_slice(),
        module: module.downgrade(),
    })
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        let fib_arg_1 = 10;

        let module = Handle::new(Module::new("__main__"));

        // Global variable slots would be determined by top-level `var` and `func` statements.
        for _ in 0..1 {
            module.borrow_mut().vars.push(Value::Nil);
        }

        let fib_func = fibonacci(module.clone());

        let code = vec![
            // func fib(n: Int) -> Int:
            Op::Closure(ConstantId::new(0)),    // create closure
            Op::Store_Global(GlobalId::new(0)), // Store closure in variable
            // fib(5)
            Op::Load_Global(GlobalId::new(0)), // Load closure from variable
            Op::I32_Const_Inline(Arg24::from_i32(fib_arg_1)),
            Op::Call_Closure { arity: 1 },
            Op::Return,
            Op::End,
        ];

        // Module top-level code.
        let func = Rc::new(ScriptFunc {
            constants: vec![
                Value::Func(fib_func), // ConstantId(0)
            ],
            code: code.into_boxed_slice(),
            module: module.downgrade(),
        });

        let closure = Handle::new(Closure::new(func));
        let program = Program::new(module, closure);

        // ---------------------------------------------------------------------------------------------
        let mut vm = VM::new();

        b.iter(|| vm.run_program(black_box(&program)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
