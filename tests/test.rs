use rust_lisp_interpreter::{RispErr, RispExp};

#[test]
fn test_bool() {
    let test_cases = vec!["(> 6 4 3 1)".to_owned()];
    let expects = vec![true];
    let env = &mut rust_lisp_interpreter::default_env();
    for i in 0..test_cases.len() {
        if let Ok(RispExp::Bool(value)) =
            rust_lisp_interpreter::parse_eval(test_cases[i].clone(), env)
        {
            assert_eq!(value, expects[i]);
        }
    }
}

#[test]
fn test_number() {
    let test_cases = vec![
        "(+ 1 1 (- 2 2 (+ 3 3) (+ 4 4) ) (+ 1 1 (+ 1 1 (- 1 1))) )".to_owned(),
        "(+ 10 5 (- 10 3 3))".to_owned(),
    ];
    let expects = vec![-8.0, 19.0];
    let env = &mut rust_lisp_interpreter::default_env();
    for i in 0..test_cases.len() {
        if let Ok(RispExp::Number(num)) =
            rust_lisp_interpreter::parse_eval(test_cases[i].clone(), env)
        {
            assert_eq!(num, expects[i]);
        }
    }
}

#[test]
fn test() {
    let test_cases = vec![
        "(def a 1)".to_owned(),
        "(+ a 1)".to_owned(),
        "(if (> 2 4 6) 1 2)".to_owned(),
        "(if (< 2 4 6) 1 2)".to_owned(),
        "(def add-one (fn (a) (+ 1 a)))".to_owned(),
        "(add-one 1)".to_owned(),
    ];
    let env = &mut rust_lisp_interpreter::default_env();

    if let RispExp::Symbol(sym) =
        rust_lisp_interpreter::parse_eval(test_cases[0].clone(), env).unwrap()
    {
        assert_eq!(sym, "a");
    }
    if let RispExp::Number(num) =
        rust_lisp_interpreter::parse_eval(test_cases[1].clone(), env).unwrap()
    {
        assert_eq!(num, 2.0);
    }
    if let RispExp::Number(num) =
        rust_lisp_interpreter::parse_eval(test_cases[2].clone(), env).unwrap()
    {
        assert_eq!(num, 2.0);
    }
    if let RispExp::Number(num) =
        rust_lisp_interpreter::parse_eval(test_cases[3].clone(), env).unwrap()
    {
        assert_eq!(num, 1.0);
    }
    if let RispExp::Symbol(sym) =
        rust_lisp_interpreter::parse_eval(test_cases[4].clone(), env).unwrap()
    {
        assert_eq!(sym, "add-one");
    }
    if let RispExp::Number(num) =
        rust_lisp_interpreter::parse_eval(test_cases[5].clone(), env).unwrap()
    {
        assert_eq!(num, 2.0);
    }
}
