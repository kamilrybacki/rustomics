use crate::system::base::atom::Atom;

use crate::dynamics::NextStepCalculation;
use crate::dynamics::IntegratorClock;

pub struct VerletIntegrator {
  clock: IntegratorClock
}

impl VerletIntegrator {
  pub fn from(yaml: &yaml_rust::Yaml) -> VerletIntegrator {
    let simulation_timestep = yaml["timestep"].as_f64().unwrap();
    let simulation_total_time = yaml["total_time"].as_f64().unwrap();
    VerletIntegrator {
      clock: IntegratorClock::new(
        simulation_timestep,
        simulation_total_time
      )
    }
  }
}

impl NextStepCalculation for VerletIntegrator {
  fn next(
    &mut self,
    atoms: &mut Vec<Atom>,
    box_vectors: &[[f64; 3]; 3],
    periodicity: &[bool; 3]
  ) -> () {
    for _ in 0..10 {
      self.clock.tick();
    }
    println!("Verlet integrator")
  }
}

impl std::fmt::Display for VerletIntegrator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Verlet integrator")
  }
}