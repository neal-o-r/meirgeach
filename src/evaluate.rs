use std::rc::Rc;
use crate::symbol::{atomise, Atom, Lambda};
use crate::env::{Env, env_get, env_add};
use std::collections::HashMap;


fn tokenise(code: String) -> Vec<String> {
    code
    .replace("\n", "")
    .replace("\t", "")
    .replace("(", " ( ")
    .replace(")", " ) ")
    .split_whitespace()
    .map(|x| x.to_string())
    .collect()
}


fn parse<'a>(tokens: &'a[String], env: &mut Env) -> (Atom, &'a[String]) {
    let (token, tail) = tokens.split_first().unwrap();
    match &token[..] {
        "(" => read_ahead(tail, env),
        ")" => panic!("FADHB: ')' gan súil leis!"),
        _ => (atomise(token.to_string(), env), tail)
    }
}


fn read_ahead<'a>(tokens: &'a[String], env: &mut Env) -> (Atom, &'a[String]) {
    let mut out = vec![];
    let mut ts = tokens;
    loop {
        let (next_token, tail) = ts.split_first().expect("FADHB: Ag súil le ')'");
        if next_token == ")" {
            return (Atom::List(out), tail);
        }
        let (exp, new_ts) = parse(&ts, env);
        out.push(exp);
        ts = new_ts;
    }

}

fn eval_def(args: &[Atom], env: &mut Env) -> Atom {
    match args {
        [Atom::Symbol(s), val] => {
            let v = eval(val, env);
            env_add(s, v.clone(), env);
            v
        },
        _ => panic!("FADHB: Argóint mícheart ag `sainigh`")
    }
}


fn eval_lambda(args: &[Atom]) -> Atom {
  let params = args.first().ok_or(
      "FADHB: `lambda` gan argóint".to_string(),
    ).unwrap();
  let exp = args.get(1).ok_or(
      "FADHB: `lambda` gan corp".to_string(),
  ).unwrap();

  Atom::Lambda(Lambda {exp: Rc::new(exp.clone()), params: Rc::new(params.clone())})

}

fn eval_if(args: &[Atom], env: &mut Env) -> Atom {
    match args {
        [cond, t, f] => {
            match eval(cond, env) {
                Atom::Bool(true) => eval(t, env),
                Atom::Bool(false) => eval(f, env),
                _ => panic!("FADHB: Ag súil le boole i `má`")
            }
        }
        _ => panic!("FADHB: Argóint mícheart i `má`")
    }

}

fn eval_built_ins(ast: &[Atom], env: &mut Env) -> Option<Atom> {
    match ast {
        [Atom::Symbol(s), args @ ..] => 
            match s.as_ref() {
                "sainigh" => Some(eval_def(args, env)),
                "athfhriotal" => Some(args[0].clone()),
                "'" => Some(args[0].clone()), //allow for this quick way of quoting
                "lambda" => Some(eval_lambda(args)),
                "má" => Some(eval_if(args, env)),
                _ => None,
            }
        _ => None,
    }
}


fn params_to_str(form: &Rc<Atom>) -> Vec<String> {

    let list = match form.as_ref() {
        Atom::List(s) => s.clone(),
        _ => panic!("FADHB: ag súil le argóint don lambda i liosta"),
    };

    list
        .iter()
        .map(|x| {
            match x {
                Atom::Symbol(s) => s.clone(),
                _ => panic!("FADHB: ag súil le argóint don lambda i liosta"),
            }   
        }).collect()
}


fn apply_lambda(lam: &Lambda, args: &[Atom], env: &mut Env) -> Atom {
    // turn the params into a list of strings
    let ps = params_to_str(&lam.params);
    // check if the arg length matches the lam params
    if ps.len() != args.len() {
        panic!("FADHB: Ag súil le {} argóint, fuair {} argóint", ps.len(), args.len())
    };

    let vs = eval_map(args, env);
    let mut inner: HashMap<String, Atom> = HashMap::new();
    for (k, v) in ps.iter().zip(vs.iter()) {
        inner.insert(k.clone(), v.clone());
    }
    // create a new Env
    let mut new_env = Env {env: inner, outer: Some(env)};

    // eval the lambda in that new env
    eval(&lam.exp, &mut new_env)
}


fn eval_map(args: &[Atom], env: &mut Env) -> Vec<Atom> {
    let mut out = vec![];
    for a in args {
        out.push(eval(a, env));
    }
    out
    //args.iter().map(|x| eval(x, env)).collect()
}


fn eval(ast: &Atom, env: &mut Env) -> Atom {
    match ast {
        Atom::Bool(_b) => ast.clone(),
        Atom::Number(_n) => ast.clone(),
        Atom::Func(_n) => ast.clone(),
        Atom::Symbol(_s) => env_get(ast, env),
        Atom::List(l) => 
            match eval_built_ins(l, env) {
                Some(a) => a,
                None => 
                    match &l[..] {
                        [Atom::Func(f), args @ ..] => f(&eval_map(args, env)),
                        [Atom::Lambda(l), args @ ..] => apply_lambda(l, args, env),
                        _ => {
                            // eval the eval mapped stuff
                            // if it's a one item List then take the item out of the List 
                            // and eval it as a bare object
                            let out = eval_map(l, env);
                            if (out.len() == 1) {
                                eval(&out[0], env)
                            } else {
                                eval(&Atom::List(out), env)
                            }
                        },
                    },
            },
        _ => panic!("FADHB: Earráid Comhréir!")
    }
}


pub fn parse_eval(code: String, env: &mut Env, repl: bool) -> Atom {

    let binding = tokenise(code);
    let (ast, _) = parse(&binding, env);
    if repl {
        eval(&ast, env)
    } else {
        match ast {
            Atom::List(l) => {
                eval_map(&l, env); 
                let x = eval(&l.last().unwrap().clone(), env);
                x
            },
            _ => panic!("FADHB")
        }
    }
}

