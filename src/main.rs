use std::io::{stdin, stdout, Write};

pub mod parser;
pub mod vm;

fn main() {
    let std_in = stdin();
    let mut vm = vm::VM::new();
    loop {
        let mut s = String::new();
        loop {
            print!("> ");
            stdout().flush().unwrap();
            std_in.read_line(&mut s).unwrap();
            if s.chars().filter(|x| *x == '(').count() == s.chars().filter(|x| *x == ')').count()
            {
                break;
            }
        }

        let parser = parser::Parser::new(&s);
        for node in parser {
            println!("Parsed: {node:?}");
            let result = vm.eval(&node);
            println!("Evaluated: {result}");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::VM;
    #[test]
    fn test_gcd() {
        fn gcd(a: i64, b: i64) -> i64 {
            let (a, b) = (b, a % b);
            if b == 0 {
                a
            } else {
                gcd(a, b)
            }
        }

        let gcd_text =
            "(global gcd (fn (a b) (with (_a b _b (mod a b)) (if (== _b 0) _a (gcd _a _b)))))";
        let mut vm = VM::new();
        vm.execute(gcd_text);
        for i in 1..50 {
            for j in 1..50 {
                let query = format!("(gcd {i} {j})");
                let _r1 = vm.execute(&query);
                let r1 = _r1[0].as_integer().unwrap();
                let r2 = gcd(i, j);
                assert_eq!(r1, r2);
            }
        }
        for i in -50..0 {
            for j in -50..0 {
                let query = format!("(gcd {i} {j})");
                let _r1 = vm.execute(&query);
                let r1 = _r1[0].as_integer().unwrap();
                let r2 = gcd(i, j);
                assert_eq!(r1, r2);
            }
        }
    }
}
