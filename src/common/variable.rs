use crate::common::function::Function;

#[derive(PartialEq, Debug)]
pub struct Variable {
  pub data: f32,
  pub grad: Option<f32>,
  creator: Box<Function>,
}

impl Variable {
  pub fn new(data: f32, func: Function) -> Variable {
    Variable{ data, grad: None, creator: Box::new(func), }
  }

  pub fn backward(&mut self) {
    if *self.creator != Function::None {
      let grad = Some(self.creator.backward(self.grad.unwrap()));
      let rc_input = self.creator.input();
      rc_input.borrow_mut().grad = grad;
      rc_input.borrow_mut().backward();
    }
  }
}
