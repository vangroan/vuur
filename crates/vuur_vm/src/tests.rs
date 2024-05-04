use crate::value::{GlobalId, LocalId, Program};
use crate::{
    handle::Handle,
    instruction_set::{Arg24, Op},
    value::{Closure, Module, ScriptFunc},
    vm_v2::{Store, VM},
};
use std::rc::Rc;

/// Create a recursive fibonacci script function.
fn fibonacci(store: &mut Store, module: Handle<Module>) {
    // func fib(n: Int) {
    //    if n <= 1 {
    //        return n
    //    } else {
    //        return fib(n - 1) + fib(n - 2)
    //    }
    // }
    let fib = GlobalId::new(0);
    let n = LocalId::new(0);
    let code = vec![
        Op::Load_Local { local_id: n },
        Op::I32_Const_Inline {
            arg: Arg24::from_i32(1),
        },
        Op::I32_LessEq,
        Op::Jump_False {
            addr: Arg24::from_u32(0),
        },
        Op::Load_Local { local_id: n },
        Op::Return,
        // Setup call to fib(n)
        Op::Load_Global { global_id: fib },
        // n - 1
        Op::Load_Local { local_id: n },
        Op::I32_Const_Inline {
            arg: Arg24::from_i32(1),
        },
        Op::I32_Sub,
        Op::Call_Closure { arity: 1 },
        // Setup call to fib(n)
        Op::Load_Global { global_id: fib },
        // n - 2
        Op::Load_Local { local_id: n },
        Op::I32_Const_Inline {
            arg: Arg24::from_i32(2),
        },
        Op::I32_Sub,
        Op::Call_Closure { arity: 1 },
        // fib(n - 1) + fib(n - 2)
        Op::I32_Add,
    ];
}

#[test]
fn test_vm_v2() {
    let module = Handle::new(Module::new("__main__"));

    let code = vec![
        Op::I32_Const_Inline {
            arg: Arg24::from_i32(1),
        },
        Op::I32_Const_Inline {
            arg: Arg24::from_i32(2),
        },
        Op::I32_Add,
        Op::Return,
    ];

    // Module top-level code.
    let func = Rc::new(ScriptFunc {
        constants: vec![],
        code: code.into_boxed_slice(),
        module: module.downgrade(),
    });

    let closure = Handle::new(Closure::new(func));
    let program = Program::new(module, closure);

    // ---------------------------------------------------------------------------------------------
    let mut vm = VM::new();
    let slot = vm.run_program(&program);
    println!("{slot:?}");
    assert_eq!(slot.unwrap().raw(), 3);
}
