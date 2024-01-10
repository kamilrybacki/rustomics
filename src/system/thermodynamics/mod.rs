pub mod ensemble;

use crate::simulation::Simulation;

pub struct Thermodynamics {
    pub ensemble: ensemble::Ensemble,
}

impl Thermodynamics {
    pub fn from(yaml: &yaml_rust::Yaml) -> Thermodynamics {
        Thermodynamics {
            ensemble: ensemble::Ensemble::from(&yaml["ensemble"]),
        }
    }
    pub fn update(&mut self, simulation: &Simulation) -> () {}
}
