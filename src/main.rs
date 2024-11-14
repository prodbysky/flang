use std::{collections::HashMap, error::Error, fs::read_to_string};

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
    While(Box<Function>, Vec<Function>),
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
        Function::While(cond, body) => {
            while eval_func(vars, *cond.clone()) != 0 {
                for f in &body {
                    eval_func(vars, f.clone());
                }
            }
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

fn parse_func(str: &str, i: &mut usize) -> Option<Function> {
    let chars: Vec<_> = str.chars().collect();

    while *i < chars.len() && (chars[*i].is_whitespace() || chars[*i] == ')') {
        *i += 1;
    }

    if *i >= chars.len() {
        return None;
    }

    if chars[*i].is_ascii_digit() {
        let begin = *i;
        while *i < chars.len() && chars[*i].is_ascii_digit() {
            *i += 1;
        }
        return Some(Function::Number(str[begin..*i].parse().unwrap()));
    }

    let valid_ident_char = |c: char| c.is_ascii_alphabetic() || c == '_';
    let begin = *i;
    while *i < chars.len() && valid_ident_char(chars[*i]) {
        *i += 1;
    }
    let ident = &str[begin..*i];

    match ident {
        "define" => {
            *i += 1;
            let name = parse_identifier(str, i)?;
            let value = parse_func(str, i)?;
            Some(Function::Define(name, Box::new(value)))
        }
        "add" => parse_binary_function(str, i, Function::Add),
        "sub" => parse_binary_function(str, i, Function::Sub),
        "print" => {
            *i += 1;
            let mut args = vec![];
            while *i < chars.len() && chars[*i] != ')' {
                args.push(parse_func(str, i)?);
            }
            Some(Function::Print(args))
        }
        "equal" => parse_binary_function(str, i, Function::Equal),
        "notequal" => parse_binary_function(str, i, Function::NotEqual),
        "less" => parse_binary_function(str, i, Function::Less),
        "more" => parse_binary_function(str, i, Function::More),
        "for" => {
            *i += 1;
            let init = parse_func(str, i)?;
            let condition = parse_func(str, i)?;
            let increment = parse_func(str, i)?;
            let mut body = vec![];
            while *i < chars.len() && chars[*i] != ')' {
                body.push(parse_func(str, i)?);
            }
            Some(Function::For(
                Box::new(init),
                Box::new(condition),
                Box::new(increment),
                body,
            ))
        }
        "while" => {
            *i += 1;
            let cond = parse_func(str, i)?;
            let mut body = vec![];
            while *i < chars.len() && chars[*i] != ')' {
                body.push(parse_func(str, i)?);
            }
            Some(Function::While(Box::new(cond), body))
        }
        "if" => {
            *i += 1;
            let condition = parse_func(str, i)?;
            let mut body = vec![];
            while *i < chars.len() && chars[*i] != ')' {
                body.push(parse_func(str, i)?);
            }
            Some(Function::If(Box::new(condition), body))
        }
        _ => Some(Function::Ident(ident.to_string())),
    }
}

fn parse_binary_function(
    str: &str,
    i: &mut usize,
    constructor: fn(Box<Function>, Box<Function>) -> Function,
) -> Option<Function> {
    *i += 1;
    let left = parse_func(str, i)?;
    let right = parse_func(str, i)?;
    Some(constructor(Box::new(left), Box::new(right)))
}

fn parse_identifier(str: &str, i: &mut usize) -> Option<String> {
    let chars: Vec<_> = str.chars().collect();
    let valid_ident_char = |c: char| c.is_ascii_alphabetic() || c == '_';
    let start = *i;

    while *i < chars.len() && valid_ident_char(chars[*i]) {
        *i += 1;
    }

    if start == *i {
        None
    } else {
        Some(str[start..*i].to_string())
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

fn run(module: Vec<Function>) {
    let mut vars = HashMap::new();
    for func in module {
        eval_func(&mut vars, func);
    }
    dbg!(vars);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let source = read_to_string(args.next().unwrap()).unwrap();
    let program = parse(source);
    run(program);
    Ok(())
}
