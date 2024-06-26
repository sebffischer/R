use r_derive::*;

use crate::callable::core::*;
use crate::context::Context;
use crate::error::Error;
use crate::lang::*;
use crate::object::*;

#[derive(Debug, Clone, PartialEq)]
#[builtin(sym = "environment")]
pub struct PrimitiveEnvironment;
impl Callable for PrimitiveEnvironment {
    fn formals(&self) -> ExprList {
        ExprList::from(vec![(Some(String::from("fun")), Expr::Missing)])
    }

    fn call(&self, args: ExprList, stack: &mut CallStack) -> EvalResult {
        let (vals, _) = self.match_arg_exprs(args, stack)?;
        let mut vals = Obj::List(vals);

        // default when `fun` is missing or not found
        let fun = vals.try_get_named("fun");
        if let Ok(Obj::Promise(_, Expr::Missing, _))
        | Err(Signal::Error(Error::ArgumentMissing(_))) = fun
        {
            return Ok(Obj::Environment(stack.env().clone()));
        };

        // otherwise we can evaluate value and return result's environment
        match fun?.force(stack)? {
            Obj::Promise(.., e) => Ok(Obj::Environment(e.clone())),
            Obj::Function(.., e) => Ok(Obj::Environment(e.clone())),
            Obj::Environment(e) => Ok(Obj::Environment(e.clone())),
            _ => Error::ArgumentInvalid(String::from("fun")).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{r, r_expect};

    #[test]
    fn no_args() {
        (r! { environment() }).expect("environment() returns non-environment value");
    }

    #[test]
    fn functions_return_env() {
        r_expect! {{"
            x <- function() { function() { } }
            environment(x()) != environment(x)
        "}}
    }

    #[test]
    fn capture_local_env() {
        r_expect! {{"
            x <- function() { environment() }
            x() != environment(x)
        "}}
    }
}
