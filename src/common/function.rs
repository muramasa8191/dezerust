use std::cell::RefCell;
use std::rc::Rc;
use crate::common::variable::Variable;

#[derive(PartialEq, Debug)]
pub enum Function {
  Square{input: Rc<RefCell<Variable>>},
  Exp{input: Rc<RefCell<Variable>>},
  None
}

impl Function {
  pub fn square(input: Rc<RefCell<Variable>>) -> Variable {
    let s = Function::Square{input};
    s.exec()
  }

  pub fn exp(input: Rc<RefCell<Variable>>) -> Variable {
    let e = Function::Exp{input};
    e.exec()
  }

  fn exec(mut self) -> Variable {
    let y = self.forward();
    Variable::new(y, self)
  }

  fn forward(&mut self) -> f32 {
    match self {
      Function::Square{input} => {
        let x = input.borrow().data;
        x * x
      },
      Function::Exp{input} => {
        let x = input.borrow().data;
        x.exp()
      },
      _ => 0.0
    }
  }

  pub fn backward(&self, gy: f32) -> f32 {
    match self {
      Function::Square{input} => {
        let x = input.borrow().data;
        2.0 * x * gy
      },
      Function::Exp{input} => {
        let x = input.borrow().data;
        x.exp() * gy
      },
      _ => 0.0
    }
  }

  pub fn input(&mut self) -> Rc<RefCell<Variable>> {
    match self {
      Function::Square{ input } => input.clone(),
      Function::Exp{ input } => input.clone(),
      Function::None => panic!(),
    }
  }
}
