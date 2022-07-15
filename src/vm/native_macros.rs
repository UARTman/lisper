use crate::parser::ASTNode;

use super::{Data, VM};

pub type NativeMacro = fn(&mut VM, &[ASTNode]) -> Option<Data>;

fn _wrap(f: NativeMacro) -> Data {
    Data::NativeMacro(Box::new(f))
}

fn global(vm: &mut VM, args: &[ASTNode]) -> Option<Data> {
    let id_node = args.get(0)?;
    let id = id_node.as_id()?;
    let val_node = args.get(1)?;
    let val = vm.eval(val_node);
    vm.declare_global(&id, val);
    Some(Data::Null)
}

fn do_seq(vm: &mut VM, args: &[ASTNode]) -> Option<Data> {
    args.iter().map(|x| vm.eval(x)).last()
}

fn with_variables(vm: &mut VM, args: &[ASTNode]) -> Option<Data> {
    let variable_declaration = args[0].as_list()?;
    if variable_declaration.len() % 2 != 0 {
        return None;
    }
    vm.scope_begin();
    for i in 0..variable_declaration.len() / 2 {
        let id = variable_declaration[i * 2].as_id()?;
        let contents_node = variable_declaration[i * 2 + 1].clone();
        let contents = vm.eval(&contents_node);
        vm.declare_local(&id, contents);
    }

    let result = vm.eval(&args[1]);

    vm.scope_end();
    Some(result)
}

fn if_else(vm: &mut VM, args: &[ASTNode]) -> Option<Data> {
    let condition = vm.eval(&args[0]).as_bool()?;
    Some(if condition {
        vm.eval(&args[1])
    } else {
        vm.eval(&args[2])
    })
}

fn lambda(_vm: &mut VM, args: &[ASTNode]) -> Option<Data> {
    let fargs = args[0].as_list()?.iter().filter_map(|x| x.as_id()).collect();
    Some(Data::Lambda(fargs, args[1].clone()))
}

#[rustfmt::skip]
const MACRO_TABLE: [(&str, NativeMacro); 5] = [
    ("global", global),
    ("do", do_seq),
    ("with", with_variables),
    ("if", if_else),
    ("fn", lambda),
];

pub fn load_native_macros(vm: &mut VM) {
    for (name, m) in MACRO_TABLE {
        vm.declare_global(name, _wrap(m));
    }
}
