use crate::common::variable::Variable;

pub trait Function {
  fn exec(&mut self, input: Variable) -> Variable {
    let y = self.forward(input);
    Variable::new(y)
  }

  fn forward(&mut self, input: Variable) -> f32;
  fn backward(&self, gy: f32) -> f32;
}

pub struct Square {
  input: Variable,
}

impl Square {
  pub fn new() -> Square {
    Square{ input: Variable::new(0.0) }
  }
}

impl Function for Square {
  fn forward(&mut self, input: Variable) -> f32 {
    let x = input.data;
    self.input = input;
    x * x
  }

  fn backward(&self, gy: f32) -> f32 {
    let x = self.input.data;
    2.0 * x * gy
  }
}

pub struct Exp {
  input: Variable,
}

impl Exp {
  pub fn new() -> Exp {
    Exp { input: Variable::new(0.0) }
  }
}

impl Function for Exp {
  fn forward(&mut self, input: Variable) -> f32 {
    let x = input.data;
    self.input = input;
    x.exp()
  }

  fn backward(&self,gy: f32) -> f32 {
    let x = self.input.data;
    x.exp() * gy
  }
}
