pub mod neighbours;
pub mod equations;

use crate::system::base::atom::Atom;

pub enum DynamicsIntegrator {
  Verlet(equations::verlet::VerletIntegrator)
}

impl std::fmt::Display for DynamicsIntegrator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let indent = 2;
    let integrator_description = match self {
      DynamicsIntegrator::Verlet(x) => format!("{:indent$}{}", "", x, indent=indent) 
    };
    write!(f, "Integrator:\n{}", integrator_description)
  }
}

pub struct IntegratorClock {
  pub step: i64,
  pub timestep: f64,
  pub current_time: f64,
  pub total_time: f64,
  pub unit: f64
}

impl IntegratorClock {
  pub fn new(timestep: f64, total_time: f64) -> IntegratorClock {
    IntegratorClock {
      step: 0,
      current_time: 0.0,
      unit: 1.0,
      timestep: timestep,
      total_time: total_time
    }
  }
  pub fn tick(&mut self) -> () {
    self.step += 1;
    self.current_time += self.timestep;
  }
  pub fn reset(&mut self) -> () {
    self.step = 0;
    self.current_time = 0.0;
  }
  pub fn has_finished(&self) -> bool {
    self.current_time >= self.total_time
  }
}

trait NextStepCalculation {
  fn next(
    &mut self,
    atoms: &mut Vec<Atom>,
    box_vectors: &[[f64; 3]; 3],
    periodicity: &[bool; 3]
  ) -> () {
    panic!("Not implemented");
  }
}
