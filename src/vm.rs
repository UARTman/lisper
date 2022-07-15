use im::{vector, HashMap, Vector};

use crate::parser::*;

pub mod datatypes;
pub mod native_functions;
pub mod native_macros;

pub use datatypes::*;

pub struct VM {
    global_scope: HashMap<String, Data>,
    stack: Vector<HashMap<String, Data>>,
}

impl VM {
    pub fn new() -> Self {
        let mut this = Self {
            global_scope: HashMap::new(),
            stack: vector![],
        };
        native_functions::load_native_functions(&mut this);
        native_macros::load_native_macros(&mut this);
        this
    }

    pub fn eval(&mut self, node: &ASTNode) -> Data {
        match node {
            ASTNode::List(l) => {
                let mut args: Vector<ASTNode> = l.clone();
                args.remove(0);
                let f = self.eval(&l[0]);
                match f {
                    Data::NativeFunction(fun) => {
                        let args_evaluated: Vec<Data> = args.iter().map(|x| self.eval(x)).collect();
                        fun(self, &args_evaluated).unwrap()
                    }
                    Data::NativeMacro(fun) => {
                        let args_values: Vec<ASTNode> = args.iter().map(|x| x.clone()).collect();
                        fun(self, &args_values).unwrap()
                    }
                    Data::Lambda(argnames, node) => {
                        let args_evaluated: Vec<Data> = args.iter().map(|x| self.eval(x)).collect();
                        self.scope_begin();
                        for (i, argname) in argnames.iter().enumerate() {
                            self.declare_local(argname, args_evaluated[i].clone());
                        }
                        let result = self.eval(&node);
                        self.scope_end();
                        result
                    }
                    _ => todo!(),
                }
            }
            ASTNode::NumberLiteral(i) => Data::Integer(i.clone()),
            ASTNode::StringLiteral(s) => Data::String(s.clone()),
            ASTNode::Identifier(id) => {
                let data = self.lookup(id).unwrap().clone(); // TODO: Change to Result
                data
            }
            ASTNode::Quote(node) => Data::Quote(node.clone()),
            ASTNode::_RParen => panic!("RParen!"),
            ASTNode::BoolLiteral(b) => Data::Boolean(*b),
            ASTNode::Null => Data::Null,
        }
    }

    pub fn lookup(&self, id: &String) -> Option<&Data> {
        for i in self.stack.iter().rev() {
            if i.contains_key(id) {
                return i.get(id);
            }
        }
        self.global_scope.get(id)
    }

    pub fn lookup_mut(&mut self, id: &String) -> Option<&mut Data> {
        for i in self.stack.iter_mut().rev() {
            if i.contains_key(id) {
                return i.get_mut(id);
            }
        }
        self.global_scope.get_mut(id)
    }

    pub fn declare_global(&mut self, id: &str, data: Data) {
        self.global_scope.insert(id.to_string(), data);
    }

    pub fn scope_begin(&mut self) {
        self.stack.push_back(HashMap::new());
    }

    pub fn scope_end(&mut self) {
        self.stack.pop_back();
    }

    pub fn declare_local(&mut self, id: &str, data: Data) {
        self.stack
            .get_mut(self.stack.len() - 1)
            .unwrap()
            .insert(id.to_string(), data);
    }

    pub fn execute(&mut self, text: &str) ->  Vec<Data> {
        let parser = Parser::new(text);
        parser.map(|node| {
            println!("Parsed: {node:?}");
            let result = self.eval(&node);
            println!("Evaluated: {result}");
            result
        }).collect()
    }
}
