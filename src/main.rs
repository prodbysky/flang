use std::{collections::HashMap, error::Error};

#[derive(Debug, Clone)]
enum Function {
    Define(String, Box<Function>),
    Add(Box<Function>, Box<Function>),
    Sub(Box<Function>, Box<Function>),
    Number(i32),
    Ident(String),
    Print(Vec<Function>),
    Equal(Box<Function>, Box<Function>),
    NotEqual(Box<Function>, Box<Function>),
    Less(Box<Function>, Box<Function>),
    More(Box<Function>, Box<Function>),
    For(Box<Function>, Box<Function>, Box<Function>, Vec<Function>),
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
        Function::For(begin, end, increment, body) => {
            let mut counter = eval_func(vars, *begin);
            let end = eval_func(vars, *end);
            let increment = eval_func(vars, *increment);

            vars.insert(String::from("_i"), counter);

            while counter < end {
                for f in &body {
                    eval_func(vars, f.clone());
                }
                counter += increment;
                vars.insert(String::from("_i"), counter);
            }
            vars.remove(&String::from("_i"));
        }
        Function::Equal(a, b) => {
            return (eval_func(vars, *a) == eval_func(vars, *b)) as i32;
        }
        Function::NotEqual(a, b) => {
            return (eval_func(vars, *a) != eval_func(vars, *b)) as i32;
        }
        Function::Less(a, b) => {
            return (eval_func(vars, *a) < eval_func(vars, *b)) as i32;
        }
        Function::More(a, b) => {
            return (eval_func(vars, *a) > eval_func(vars, *b)) as i32;
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
    (for!($start:expr, $end:expr, $incr:expr, $($f:expr),* $(,)?)) => {
        Function::For(Box::new($start), Box::new($end), Box::new($incr), vec![$($f),*])
    };
    (for!($start:expr, $end:expr, $($f:expr),* $(,)?)) => {
        Function::For(Box::new($start), Box::new($end), Box::new(1), vec![$($f),*])
    };
    (for!($end:expr, $($f:expr),* $(,)?)) => {
        Function::For(Box::new(Function::Number(0)), Box::new($end), Box::new(Function::Number(1)), vec![$($f),*])
    };
    (print!($($arg:expr),* $(,)?)) => {
        Function::Print(vec![$($arg),*])
    };
    (equal!($left:expr, $right:expr)) => {
        Function::Equal(Box::new($left), Box::new($right))
    };
    (not_equal!($left:expr, $right:expr)) => {
        Function::NotEqual(Box::new($left), Box::new($right))
    };
    (less!($left:expr, $right:expr)) => {
        Function::Less(Box::new($left), Box::new($right))
    };
    (more!($left:expr, $right:expr)) => {
        Function::More(Box::new($left), Box::new($right))
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let program = vec![
        func!(for!(func!(5), func!(print!(ident!("_i"))))),
        // func!(@define "a" func!(5)),
        // func!(@define "b" func!(add!(ident!("a"), func!(10)))),
        // func!(print!(ident!("a"), ident!("b"))),
    ];
    run(program);
    Ok(())
}
