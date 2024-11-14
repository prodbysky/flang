use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug)]
enum Function {
    Define(String, Box<Function>),
    Add(Box<Function>, Box<Function>),
    Sub(Box<Function>, Box<Function>),
    Number(i32),
    Ident(String),
    Print(Vec<Function>),
}

fn eval_func(vars: &mut HashMap<String, i32>, f: Function) -> i32 {
    match f {
        Function::Define(name, func) => {
            if let Function::Number(a) = *func {
                vars.insert(name, a);
            } else {
                let value = eval_func(vars, *func);
                vars.insert(name, value);
            }
        }
        Function::Number(a) => {
            return a;
        }
        Function::Add(a, b) => return eval_func(vars, *a) + eval_func(vars, *b),
        Function::Sub(a, b) => return eval_func(vars, *a) - eval_func(vars, *b),
        Function::Ident(name) => return *vars.get(&name).unwrap(),
        Function::Print(values) => {
            for val in values {
                print!("{} ", eval_func(vars, val));
            }
            println!();
        }
    };

    0
}

fn run(module: Vec<Function>) {
    let mut vars = HashMap::new();
    for func in module {
        eval_func(&mut vars, func);
    }
    dbg!(vars);
}

macro_rules! ident {
    ($name:literal) => {
        Function::Ident(String::from($name))
    };
}
macro_rules! func {
    ($num:literal) => {
        Function::Number($num)
    };

    (@define $name:literal $right:expr) => {
        Function::Define(String::from($name), Box::new($right))
    };

    (add!($left:expr, $right:expr)) => {
        Function::Add(Box::new($left), Box::new($right))
    };

    (sub!($left:expr, $right:expr)) => {
        Function::Sub(Box::new($left), Box::new($right))
    };
    (print!($($arg:expr),* $(,)?)) => {
        Function::Print(vec![$($arg),*])
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = vec![
        func!(@define "a" func!(5)),
        func!(@define "b" func!(add!(ident!("a"), func!(10)))),
        func!(print!(ident!("a"), ident!("b"))),
    ];
    run(program);
    Ok(())
}
