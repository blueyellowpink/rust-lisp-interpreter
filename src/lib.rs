use std::{collections::HashMap, fmt, io, num::ParseFloatError};

fn tokenize(expr: &str) -> Vec<String> {
    expr.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|x| x.to_owned())
        .collect()
}

fn parse(tokens: &[String]) -> Result<RispExp, RispErr> {
    let mut stack: Vec<&str> = vec![];
    let mut memo: HashMap<usize, Vec<RispExp>> = HashMap::new();
    memo.insert(0, vec![]);
    let mut level = 0;
    for token in tokens.iter() {
        if token == "(" {
            level += 1;
            memo.insert(level, vec![]);
        }
        if token == ")" {
            while let Some(item) = stack.pop() {
                let exp = match item {
                    "(" => break,
                    _ => parse_atom(item),
                };
                if let Some(current) = memo.get_mut(&level) {
                    current.insert(0, exp);
                }
            }
            let mut exp: Option<RispExp> = None;
            if let Some(current) = memo.get(&level) {
                exp = Some(RispExp::List(current.clone()));
            };
            level -= 1;
            if let Some(prev) = memo.get_mut(&level) {
                prev.insert(0, exp.unwrap());
            }
            continue;
        }
        stack.push(token);
    }
    Ok(memo.get(&0).unwrap()[0].clone())
}

fn parse_atom(token: &str) -> RispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => RispExp::Number(v),
        Err(_) => RispExp::Symbol(token.to_string()),
    }
}

#[derive(Clone)]
pub enum RispExp {
    Symbol(String),
    Number(f64),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispErr>), // bam
}

#[derive(Debug)]
pub enum RispErr {
    Reason(String),
}

#[derive(Clone)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

pub fn default_env() -> RispEnv {
    let mut data: HashMap<String, RispExp> = HashMap::new();
    data.insert(
        "+".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);

            Ok(RispExp::Number(sum))
        }),
    );
    data.insert(
        "-".to_string(),
        RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or_else(|| RispErr::Reason("expected at least one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

            Ok(RispExp::Number(first - sum_of_rest))
        }),
    );

    RispEnv { data }
}

fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    match exp {
        RispExp::Symbol(k) => env
            .data
            .get(k)
            .ok_or_else(|| RispErr::Reason(format!("unexpected symbol k='{}'", k)))
            .map(|x| x.clone()),
        RispExp::Number(_a) => Ok(exp.clone()),
        RispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or_else(|| RispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            let first_eval = eval(first_form, env)?;
            match first_eval {
                RispExp::Func(f) => {
                    let args_eval = arg_forms
                        .iter()
                        .map(|x| eval(x, env))
                        .collect::<Result<Vec<RispExp>, RispErr>>();
                    f(&args_eval?)
                }
                _ => Err(RispErr::Reason("first form must be a function".to_string())),
            }
        }
        RispExp::Func(_) => Err(RispErr::Reason("unexpected form".to_string())),
    }
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            RispExp::Symbol(s) => s.clone(),
            RispExp::Number(n) => n.to_string(),
            RispExp::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            RispExp::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

pub fn parse_eval(expr: String, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let parsed_exp = parse(&tokenize(&expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

pub fn slurp_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispErr> {
    args.iter().map(parse_single_float).collect()
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispErr> {
    match exp {
        RispExp::Number(num) => Ok(*num),
        _ => Err(RispErr::Reason("expected a number".to_string())),
    }
}
