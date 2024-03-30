use r_derive::*;

use crate::callable::core::*;
use crate::lang::*;
use crate::object::*;


#[derive(Debug, Clone, PartialEq)]
#[builtin(sym = "length")]
pub struct PrimitiveLength;

impl Callable for PrimitiveLength {
    fn formals(&self) -> ExprList {
        ExprList::from(vec![(None, Expr::Ellipsis(None))])
    }
    fn call_matched(&self, args: List, _ellipsis: List, stack: &mut CallStack) -> EvalResult {
        let mut args = Obj::List(args);
        let mut x = args.try_get_named("x")?.force(stack)?;

        let length: usize = match x {
            Obj::Vector(ref mut vec) => {
                match vec {
                    Vector::Double(rep) => rep.len(),
                    Vector::Integer(rep) => rep.len(),
                    Vector::Logical(rep) => rep.len(),
                    Vector::Character(rep) => rep.len(),
                }
            },
            Obj::List(l) => todo!(),
            Obj::Environment(env) => todo!(),
            _ => {
                todo!()
            }
        };

        EvalResult::Ok(Obj::Vector(Vector::from(vec![OptionNA::Some(length as i32)])))
    }
}

#[cfg(test)]
mod tests {
    use crate::{r, r_expect};


    #[test]
    fn subset_mask() {
        r_expect! {{"
            x = 1:2
            length(x[(true, false)]) == 1L
        "}}
    }
}
