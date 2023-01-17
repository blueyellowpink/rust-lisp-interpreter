use rust_lisp_interpreter::{LispParser, RispErr, RispExp};

#[test]
fn test_bool() {
    let test_cases = vec!["(> 6 4 3 1)".to_owned()];
    let expects = vec![true];
    let mut parser = LispParser::new();
    for i in 0..test_cases.len() {
        if let Ok(RispExp::Bool(value)) = parser.parse_eval(test_cases[i].clone()) {
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
    let mut parser = LispParser::new();
    for i in 0..test_cases.len() {
        if let Ok(RispExp::Number(num)) = parser.parse_eval(test_cases[i].clone()) {
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
    let mut parser = LispParser::new();

    if let RispExp::Symbol(sym) = parser.parse_eval(test_cases[0].clone()).unwrap() {
        assert_eq!(sym, "a");
    }
    if let RispExp::Number(num) = parser.parse_eval(test_cases[1].clone()).unwrap() {
        assert_eq!(num, 2.0);
    }
    if let RispExp::Number(num) = parser.parse_eval(test_cases[2].clone()).unwrap() {
        assert_eq!(num, 2.0);
    }
    if let RispExp::Number(num) = parser.parse_eval(test_cases[3].clone()).unwrap() {
        assert_eq!(num, 1.0);
    }
    if let RispExp::Symbol(sym) = parser.parse_eval(test_cases[4].clone()).unwrap() {
        assert_eq!(sym, "add-one");
    }
    if let RispExp::Number(num) = parser.parse_eval(test_cases[5].clone()).unwrap() {
        assert_eq!(num, 2.0);
    }
}
