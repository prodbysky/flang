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
    If(Box<Function>, Vec<Function>),
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
        Function::If(condition, body) => {
            if eval_func(vars, *condition) != 0 {
                for f in &body {
                    eval_func(vars, f.clone());
                }
            }
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
    (if!($condition:expr, $($f:expr),* $(,)?)) => {
        Function::If(Box::new($condition), vec![$($f),*])
    };

}

fn parse_func(str: &str, i: &mut usize) -> Option<Function> {
    let chars: Vec<_> = str.chars().collect();

    // Skip whitespace and closing parentheses
    while *i < chars.len() && (chars[*i].is_whitespace() || chars[*i] == ')') {
        *i += 1;
    }

    // If we've reached the end of the string, return None to signal end of parsing
    if *i >= chars.len() {
        return None;
    }

    // Check if we're parsing a number
    if chars[*i].is_ascii_digit() {
        let begin = *i;
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            *i += 1;
        }
        return Some(Function::Number(str[begin..*i].parse().unwrap()));
    }

    // Check if we're parsing an identifier
    let valid_ident_char = |c: char| c.is_ascii_alphabetic() || c == '_';
    let begin = *i;

    while *i < chars.len() && valid_ident_char(chars[*i]) {
        *i += 1;
    }

    let ident = &str[begin..*i];

    // If the identifier is empty, return None to signal no more valid content to parse
    if ident.is_empty() {
        return None;
    }

    match ident {
        "define" => {
            *i += 1;
            // Find the closing parenthesis after the "define" arguments
            if let Some(pos) = str[*i..].find(')') {
                let closing = pos + *i;
                let args: Vec<_> = str[*i..closing].split_whitespace().collect();
                *i += args[0].len() + 1;
                Some(Function::Define(
                    args[0].to_string(),
                    Box::new(parse_func(str, i)?),
                ))
            } else {
                None // If we don't find a closing parenthesis, return None
            }
        }
        "add" => {
            *i += 1;
        }
        _ => None,
    }
}

fn parse(str: String) -> Vec<Function> {
    let mut i = 0;
    let mut fs = vec![];
    while i < str.len() {
        if let Some(func) = parse_func(&str, &mut i) {
            fs.push(func);
        } else {
            break;
        }
    }
    fs
}
fn main() -> Result<(), Box<dyn Error>> {
    let source = "define(a 69)";
    let program = parse(source.to_string());
    dbg!(program);
    // let program = vec![
    //     func!(@define "a" func!(5)),
    //     func!(if!(
    //             func!(equal!(func!(5), ident!("a"))),
    //             func!(print!(func!(69)))
    //         )
    //     ),
    // func!(for!(func!(5), func!(print!(ident!("_i"))))),
    // func!(@define "b" func!(add!(ident!("a"), func!(10)))),
    // func!(print!(ident!("a"), ident!("b"))),
    // ];
    // run(program);
    Ok(())
}
