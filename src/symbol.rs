use crate::env::{Env, env_add, env_get};
use std::fmt;
use std::rc::Rc;


#[derive(Clone)]
pub enum Atom {
    // the kinds of things an Atom can hold
    Symbol(String),
    Bool(bool),
    Str(String),
    Number(f64),
    List(Vec<Atom>),
    Func(fn(&[Atom]) -> Atom), // debug isn't supported for Func
    Lambda(Lambda)
}

#[derive(Clone)]
pub struct Lambda {
  pub params: Rc<Atom>,
  pub exp: Rc<Atom>,
}


impl fmt::Display for Atom {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let str = match self {
      Atom::Bool(a) => if *a {"#tá".to_string()} else {"#níl".to_string()},
      Atom::Str(s) => s.to_string(),
      Atom::Symbol(s) => s.clone(),
      Atom::Number(n) => n.to_string(),
      Atom::List(list) => {
        let xs: Vec<String> = list
          .iter()
          .map(|x| x.to_string())
          .collect();
        format!("({})", xs.join(", "))
      },
      Atom::Func(_) => "Function {}".to_string(),
      Atom::Lambda(_) => "Lambda {}".to_string(),
    };
    
    write!(f, "{}", str)
  }
}

impl PartialEq for Atom {
    
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::Bool(a), Atom::Bool(b)) => a == b,
            (Atom::Str(a), Atom::Str(b)) => a == b,
            (Atom::Symbol(a), Atom::Symbol(b)) => a == b,
            (Atom::Number(a), Atom::Number(b)) => a == b,
            (Atom::List(a), Atom::List(b)) => a == b, 
            _ => false
        }
    }
}


pub fn atomise(token: String, env: &mut Env) -> Atom {
    if token == "#tá" {
        return Atom::Bool(true);
    } else if token == "#níl" {
        return Atom::Bool(false);
    } else if token.starts_with(r#"""#) {
        return Atom::Str(token[1..token.len() - 1].to_string());
    } else if token.bytes().all(|c| c.is_ascii_digit()) {
        return Atom::Number(token.parse().unwrap());
    } else if env.env.contains_key(&token) {
        env_get(&Atom::Symbol(token), env)
    } else {
        Atom::Symbol(token)/*
        if !env.env.contains_key(&token) {
            env_add(&token, Atom::Symbol(token.clone()), env);
        }
        return env_get(&Atom::Symbol(token), env)*/
    }
}

