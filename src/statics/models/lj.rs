use crate::statics::models::CalculatePotential;
use yaml_rust::Yaml;

pub struct LennardJonesModel {
    epsilon: f64,
    sigma: f64,
    cutoff: f64,
}

impl LennardJonesModel {
    pub fn initialize(definition: &Yaml) -> LennardJonesModel {
        LennardJonesModel {
            epsilon: definition["parameters"]["epsilon"].as_f64().unwrap(),
            sigma: definition["parameters"]["sigma"].as_f64().unwrap(),
            cutoff: match &definition["cutoff"] {
                Yaml::Real(cutoff) => match cutoff.parse::<f64>() {
                    Ok(cutoff) => cutoff,
                    Err(_) => panic!("Cutoff must be a real number"),
                },
                Yaml::BadValue => definition["parameters"]["sigma"].as_f64().unwrap() * 2.5,
                _ => panic!("Cutoff must be a real number"),
            },
        }
    }
    fn calculate_potential_at_distance(&self, r: f64) -> f64 {
        let r6 = r.powi(6);
        let r12 = r6.powi(2);
        4.0 * self.epsilon * ((self.sigma.powi(12) / r12) - (self.sigma.powi(6) / r6))
    }
}

impl CalculatePotential for LennardJonesModel {
    fn calculate_potential(&self, distance: f64) -> f64 {
        match distance < self.cutoff {
            true => match distance > 0.0 {
                true => self.calculate_potential_at_distance(distance),
                // Lennard-Jones potential is infinite at r = 0
                false => f64::INFINITY,
            },
            // Lennard-Jones potential is 0 at r > cutoff
            false => 0.0,
        }
    }
    fn calculate_force(&self, distance: f64) -> f64 {
        match distance < self.cutoff {
            true => match distance > 0.0 {
                true => {
                    let r7 = distance.powi(3);
                    let r13 = distance.powi(2);
                    -48.0
                        * self.epsilon
                        * ((self.sigma.powi(12) / r13) - 0.5 * (self.sigma.powi(6) / r7))
                }
                // Lennard-Jones potential is infinite at r = 0
                false => f64::INFINITY,
            },
            // Lennard-Jones potential is 0 at r > cutoff
            false => 0.0,
        }
    }
}
