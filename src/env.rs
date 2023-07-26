use std::collections::HashMap;
use crate::symbol::Atom;

pub struct Env<'a> {
    pub env: HashMap<String, Atom>,
    pub outer: Option<&'a Env<'a>>,
}

pub fn env_add(symbol: &str, atom: Atom, env: &mut Env) {
    // add a symbol to the Environment
    let _ = env.env.insert(symbol.to_string(), atom);
}

pub fn env_get(atom: &Atom, env: &Env) -> Atom {
    // get a symbol from the environment
    let sym = match atom {
        Atom::Symbol(s) => s, 
        _ => panic!("FADHB: Ag súil le siombail")
    };
    match env.env.get(sym) {
        Some(atom) => atom.clone(),
        None => {
            match &env.outer {
                Some(outer_env) => env_get(atom, &outer_env),
                None => panic!("FADHB: Níl an siombail `{}` ar fáil", sym),
            }
        }
    }
}

fn to_num(a: &Atom) -> f64 {
    match a {
        Atom::Number(n) => *n,
        _ => panic!("FADHB: Ag súil le uimhir!")
    }
}

fn map_to_num(a: &[Atom]) -> Vec<f64> {
    a.iter().map(|x| to_num(x)).collect()
}

fn add_globals(env: &mut Env) {

    fn sum(x: Vec<f64>) -> f64 {
        x.iter().sum()
    }
    fn prod(x: Vec<f64>) -> f64 {
        x.iter().copied().reduce(|a, b| a * b).unwrap()
    }

    let funcs = [
        ("+", Atom::Func(|a| Atom::Number(sum(map_to_num(a))))),
        ("-", Atom::Func(|a| Atom::Number(to_num(&a[0]) - sum(map_to_num(&a[1..]))))),
        ("*", Atom::Func(|a| Atom::Number(prod(map_to_num(a))))),
        ("/", Atom::Func(|a| Atom::Number(to_num(&a[0]) / prod(map_to_num(&a[1..]))))),

        ("=", Atom::Func(|a| match a { 
            [a, b] => Atom::Bool(a == b), 
            _ => panic!("FADHB: Argóint mícheart ag `=`") 
        })),
        
        (">", Atom::Func(|a| match a { 
            [Atom::Number(a), Atom::Number(b)] => Atom::Bool(a > b), 
            _ => panic!("FADHB: Argóint mícheart ag `>`") 
        })),
        
        ("<", Atom::Func(|a| match a { 
            [Atom::Number(a), Atom::Number(b)] => Atom::Bool(a < b), 
            _ => panic!("FADHB: Argóint mícheart ag `<`") 
        })),
        
        ("níl", Atom::Func(|a| match a { 
            [Atom::Bool(b)] => Atom::Bool(!b), 
            _ => panic!("FADHB: Argóint mícheart ag `níl`") 
        })),
        
        ("agus", Atom::Func(|a| match a { 
            [Atom::Bool(a), Atom::Bool(b)] => Atom::Bool(a & b), 
            _ => panic!("FADHB: Argóint mícheart ag `agus`") 
        })),
        
        ("ceann", Atom::Func(|a| match a { 
            [Atom::List(l)] => l[0].clone(),  
            _ => panic!("FADHB: Argóint mícheart ag `ceann`") 
        })),
        
        ("tóin", Atom::Func(|a| match a { 
            [Atom::List(l)] => Atom::List(l[1..].to_vec()), 
            _ => panic!("FADHB: Argóint mícheart ag `tóin`") 
        })),
       
        ("cons", Atom::Func(|a| match a { 
            [a, Atom::List(b)] => {let mut c = vec![a.clone()]; c.append(&mut b.clone()); Atom::List(c) },
            _ => panic!("FADHB: Argóint mícheart ag `cons`") 
        })),
        
        ("boole?", Atom::Func(|a| match a { 
            [Atom::Bool(_)] => Atom::Bool(true),
            _ => Atom::Bool(false) 
        })),
        
        ("folamh?", Atom::Func(|a| match a { 
            [Atom::List(a)] => Atom::Bool(a.len() == 0),
            _ => panic!("FADHB: Argóint míheart ag `folamh?`")
        })),

        ("liosta?", Atom::Func(|a| match a { 
            [Atom::List(_)] => Atom::Bool(true),
            _ => Atom::Bool(false) 
        })),

        ("fad", Atom::Func(|a| match a { 
            [Atom::List(a)] => {let x = a.len() as f64; Atom::Number(x) },
            _ => panic!("FADHB: Argóint mícheart ag `fad`")
        })),
        
        ("scríobh", Atom::Func(|a| match a { 
            [a] => {println!("{}", a); a.clone()},
            _ => panic!("FADHB: Argóint mícheart ag `scríobh`")
        })),
        
        ("mapáil", Atom::Func(|a| match a { 
            [Atom::Func(f), tail @ ..] => Atom::List(tail.iter().map(|t| f(&[t.clone()]).clone()).collect()),
            x => {println!("{}", x[2]); panic!("FADHB: Argóint mícheart ag `mapáil`")}
        })),
    ];
    
    for (n, f) in funcs.iter() {
        env.env.insert(n.to_string(), f.clone());
    }

}

pub fn create_env<'a>() -> Env<'a> {

    let symbol_types = vec!["#<EOF>"];

    let mut env: HashMap<String, Atom> = symbol_types.iter().map(
        |s| (s.to_string(), Atom::Symbol(s.to_string()))).collect();


    let mut e = Env { env, outer: None };
    add_globals(&mut e);

    return e;
}
