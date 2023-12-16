use crate::system::base::atom::Atom;

use crate::dynamics::NextStepCalculation;

pub struct VerletIntegrator {
    pub timestep: f64,
    pub flavor: String,
}

impl VerletIntegrator {
    pub fn from(yaml: &yaml_rust::Yaml) -> VerletIntegrator {
        let timestep = yaml["timestep"].as_f64().unwrap();
        let flavor = match yaml["flavor"].as_str() {
            Some(x) => String::from(x),
            None => String::from("velocity"),
        };
        VerletIntegrator::new(timestep, flavor)
    }
    pub fn new(timestep: f64, flavor: String) -> VerletIntegrator {
        VerletIntegrator { timestep, flavor }
    }
}

impl std::fmt::Display for VerletIntegrator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Verlet integrator ({} flavor)", self.flavor)
    }
}

impl NextStepCalculation for VerletIntegrator {
    fn next(
        &mut self,
        atoms: &mut Vec<Atom>,
        box_vectors: &[[f64; 3]; 3],
        periodicity: &[bool; 3],
    ) -> () {
        match self.flavor.as_str() {
            "basic" => basic_verlet(atoms, box_vectors, periodicity),
            "velocity" => velocity_verlet(atoms, box_vectors, periodicity),
            "leapfrog" => leapfrog_verlet(atoms, box_vectors, periodicity),
            _ => panic!("Unknown Verlet integrator flavor"),
        }
    }
}

pub fn basic_verlet(
    atoms: &mut Vec<Atom>,
    box_vectors: &[[f64; 3]; 3],
    periodicity: &[bool; 3],
) -> () {
    println!("Basic Verlet integrator")
}

pub fn velocity_verlet(
    atoms: &mut Vec<Atom>,
    box_vectors: &[[f64; 3]; 3],
    periodicity: &[bool; 3],
) -> () {
    println!("Velocity Verlet integrator")
}

pub fn leapfrog_verlet(
    atoms: &mut Vec<Atom>,
    box_vectors: &[[f64; 3]; 3],
    periodicity: &[bool; 3],
) -> () {
    println!("Leapfrog Verlet integrator")
}
