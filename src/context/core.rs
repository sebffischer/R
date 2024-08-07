use std::rc::Rc;

use crate::lang::{EvalResult, Signal};
use crate::object::*;
use crate::{error::*, internal_err};

pub trait Context: std::fmt::Debug + std::fmt::Display {
    #[inline]
    fn get(&mut self, name: String) -> EvalResult {
        (*self).env().get(name)
    }
    #[inline]
    fn get_mut(&mut self, name: String) -> EvalResult {
        self.get(name)
    }

    #[inline]
    fn get_ellipsis(&mut self) -> EvalResult {
        let err = Err(Signal::Error(Error::IncorrectContext("...".to_string())));
        self.get("...".to_string()).or(err)
    }

    #[inline]
    fn assign_lazy(&mut self, _to: Expr, _from: Expr) -> EvalResult {
        Err(Signal::Error(Error::IncorrectContext("<-".to_string())))
    }

    #[inline]
    fn assign(&mut self, _to: Expr, _from: Obj) -> EvalResult {
        Err(Signal::Error(Error::IncorrectContext("<-".to_string())))
    }

    fn env(&self) -> Rc<Environment>;

    #[inline]
    fn eval_call(&mut self, expr: Expr) -> EvalResult {
        self.eval(expr)
    }
    #[inline]
    fn eval_call_mut(&mut self, expr: Expr) -> EvalResult {
        Error::CannotEvaluateAsMutable(expr.clone()).into()
    }

    #[inline]
    fn eval(&mut self, expr: Expr) -> EvalResult {
        self.env().eval(expr)
    }

    #[inline]
    fn eval_mut(&mut self, expr: Expr) -> EvalResult {
        Error::CannotEvaluateAsMutable(expr.clone()).into()
    }

    #[inline]
    fn eval_in(&mut self, expr: Expr, mut env: Rc<Environment>) -> EvalResult {
        env.eval(expr)
    }

    #[inline]
    fn eval_and_finalize(&mut self, expr: Expr) -> EvalResult {
        self.eval(expr)
    }

    #[inline]
    fn eval_binary(&mut self, exprs: (Expr, Expr)) -> Result<(Obj, Obj), Signal> {
        Ok((
            self.eval_and_finalize(exprs.0)?,
            self.eval_and_finalize(exprs.1)?,
        ))
    }

    fn eval_list_lazy(&mut self, l: ExprList) -> EvalResult {
        Ok(Obj::List(List::from(
            l.into_iter()
                .map(|pair| match pair {
                    (_, Expr::Ellipsis(None)) => {
                        if let Ok(Obj::List(ellipsis)) = self.get_ellipsis() {
                            Ok(ellipsis.values.into_iter())
                        } else {
                            Ok(CowObj::from(vec![]).into_iter())
                        }
                    }
                    (_, Expr::Ellipsis(Some(name))) => {
                        if let Ok(Obj::List(more)) = self.get(name) {
                            Ok(more.values.into_iter())
                        } else {
                            internal_err!()
                        }
                    }
                    // Avoid creating a new closure just to point to another, just reuse it
                    (k, Expr::Symbol(s)) => match self.env().get(s.clone()) {
                        Ok(c @ Obj::Promise(..)) => Ok(CowObj::from(vec![(k, c)]).into_iter()),
                        _ => Ok(CowObj::from(vec![(
                            k,
                            Obj::Promise(None, Expr::Symbol(s), self.env()),
                        )])
                        .into_iter()),
                    },
                    (k, c @ Expr::Call(..)) => {
                        let elem = vec![(k, Obj::Promise(None, c, self.env()))];
                        Ok(CowObj::from(elem).into_iter())
                    }
                    (k, v) => {
                        if let Ok(elem) = self.eval(v) {
                            Ok(CowObj::from(vec![(k, elem)]).into_iter())
                        } else {
                            internal_err!()
                        }
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        )))
    }

    fn eval_list_eager(&mut self, l: ExprList) -> EvalResult {
        Ok(Obj::List(List::from(
            l.into_iter()
                .map(|pair| match pair {
                    (_, Expr::Ellipsis(None)) => {
                        if let Ok(Obj::List(ellipsis)) = self.get_ellipsis() {
                            Ok(ellipsis.values.into_iter())
                        } else {
                            Ok(CowObj::from(vec![]).into_iter())
                        }
                    }
                    (_, Expr::Ellipsis(Some(name))) => {
                        if let Ok(Obj::List(more)) = self.get(name) {
                            Ok(more.values.into_iter())
                        } else {
                            Ok(CowObj::from(vec![]).into_iter())
                        }
                    }
                    (k, v) => match self.eval_and_finalize(v) {
                        Ok(elem) => Ok(CowObj::from(vec![(k, elem)]).into_iter()),
                        Err(e) => Err(e),
                    },
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        )))
    }
}
