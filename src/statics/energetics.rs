
pub struct SystemEnergetics {
  pub potential_energy: f64,
  pub kinetic_energy: f64,
  pub total_energy: f64,
  pub temperature: f64,
}

impl SystemEnergetics {
  pub fn new() -> SystemEnergetics {
    SystemEnergetics {
      potential_energy: 0.0,
      kinetic_energy: 0.0,
      total_energy: 0.0,
      temperature: 0.0,
    }
  }
}
