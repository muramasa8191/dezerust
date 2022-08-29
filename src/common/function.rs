use crate::common::variable::Variable;

pub trait Function {
  fn exec(&mut self) -> Variable {
    let y = self.forward();
    Variable::new(y)
  }

  fn forward(&self) -> f32;
  fn backward(&self, gy: f32) -> f32;
}

pub struct Square<'a> {
  input: &'a Variable,
}

impl<'a> Square<'a> {
  pub fn new(input: &'a Variable) -> Square<'a> {
    Square{ input }
  }
}

impl<'a> Function for Square<'a> {
  fn forward(&self) -> f32 {
    let x = self.input.data;
    x * x
  }

  fn backward(&self, gy: f32) -> f32 {
    let x = self.input.data;
    2.0 * x * gy
  }
}

pub struct Exp<'a> {
  input: &'a Variable,
}

impl<'a> Exp<'a> {
  pub fn new(input: &'a Variable) -> Exp<'a> {
    Exp { input }
  }
}

impl<'a> Function for Exp<'a> {
  fn forward(&self) -> f32 {
    let x = self.input.data;
    x.exp()
  }

  fn backward(&self,gy: f32) -> f32 {
    let x = self.input.data;
    x.exp() * gy
  }
}
