#[derive(Debug)]
pub struct Variable {
  pub data: f32,
  pub grad: Option<f32>,
}

impl Variable {
  pub fn new(data: f32) -> Variable {
    Variable{ data, grad: None }
  }
}
