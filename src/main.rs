use std::fs;
use std::path::Path;
use std::io;


mod evaluate;
mod symbol;
mod env;


fn load(fname: &str, env: &mut env::Env) {

    let fpath = Path::new(fname);
    let code = fs::read_to_string(&fpath).expect("FADHB: ní féidir an comhad a léamh");
    evaluate::parse_eval(code, env, false);

}

fn read_console() -> String {
    let mut expr = String::new();

    io::stdin().read_line(&mut expr)
    .expect("FADHB: Ag léamh líne");

    expr
}

fn run_repl(env: &mut env::Env) {
    loop {
        println!("meirgeach >");
        let expr = read_console();
        println!("{}", evaluate::parse_eval(expr, env, true));
    }
}

fn usage() {

    let desc = env!("CARGO_PKG_DESCRIPTION");
    let name = env!("CARGO_PKG_NAME");
    println!("{}, {}", name, desc);
    println!("Usage: {} [comhad.me]", name);

}


fn main() {

    let mut env = env::create_env();

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => load(&args[1], &mut env),
        1 => run_repl(&mut env),
        _ => usage(),
    }
}
