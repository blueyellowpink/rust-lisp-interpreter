use rust_lisp_interpreter::RispErr;

fn main() {
    let expr = "(+ 1 1 (- 2 2 (+ 3 3) (+ 4 4) ) (+ 1 1 (+ 1 1 (- 1 1))) )".to_owned();
    let env = &mut rust_lisp_interpreter::default_env();
    // loop {
    println!("risp >");
    // let expr = rust_lisp_interpreter::slurp_expr();
    match rust_lisp_interpreter::parse_eval(expr, env) {
        Ok(res) => println!("// 🔥 => {}", res),
        Err(e) => match e {
            RispErr::Reason(msg) => println!("// 🙀 => {}", msg),
        },
    }
    // }
}
