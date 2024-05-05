use crate::value::{Value};
use crate::{
    handle::Handle,
    instruction_set::{Arg24, Op},
    value::{Closure, ConstantId, GlobalId, LocalId, Module, Program, ScriptFunc},
    vm_v2::{VM},
};

use std::rc::Rc;
use std::time::{Instant, Duration};

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

#[test]
fn test_vm_v2() {
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
        // Op::I32_Const_Inline {
        //     arg: Arg24::from_i32(1),
        // },
        // Op::I32_Const_Inline {
        //     arg: Arg24::from_i32(2),
        // },
        // Op::I32_Add,
        Op::Return,
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
    let start = Instant::now();
    let value = vm.run_program(&program);
    println!("time: {}µs", (Instant::now() - start).as_micros());
    println!("{value:?}");
    assert_eq!(value.unwrap().into_i32().unwrap(), 55);
}
