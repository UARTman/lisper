use crate::vm::{Data, VM};

pub type NativeFn = fn(&mut VM, &[Data]) -> Option<Data>;

fn _wrap(f: NativeFn) -> Data {
    Data::NativeFunction(Box::new(f))
}

fn add(_vm: &mut VM, args: &[Data]) -> Option<Data> {
    let mut acc: i64 = 0;
    for a in args {
        acc += a.as_integer()?;
    }
    Some(Data::Integer(acc))
}

fn modulo(_vm: &mut VM, args: &[Data]) -> Option<Data> {
    let a = args[0].as_integer()?;
    let b = args[1].as_integer()?;
    Some(Data::Integer(a % b))
}

fn eq(_vm: &mut VM, args: &[Data]) -> Option<Data> {
    let a = &args[0];
    let b = &args[1];
    Some(Data::Boolean(a == b))
}

#[rustfmt::skip]
const FUNCTION_TABLE: [(&str, NativeFn); 3] = [
    ("+", add),
    ("mod", modulo),
    ("==", eq),
];

pub fn load_native_functions(vm: &mut VM) {
    for (name, f) in FUNCTION_TABLE {
        vm.declare_global(name, _wrap(f))
    }
}
