extern crate yaml_rust;

use core::fmt;

use yaml_rust::Yaml;

use crate::data::load::load_atoms;
use crate::data::load::to_vec_f64;
use crate::data::metrics::UnitSystem;
use crate::statics::models::PotentialModel;
use crate::system::base::atom::Atom;

use crate::dynamics::equations::verlet::VerletIntegrator;
use crate::dynamics::DynamicsIntegrator;

pub struct SystemDefinition {
    pub origin: [f64; 3],       // Origin of the simulation box
    pub vectors: [[f64; 3]; 3], // Simulation box vectors
    pub periodicity: [bool; 3], // Periodicity of the simulation box
    pub atoms: Vec<Atom>,       // Atom type, position, velocity, etc.
    pub units: UnitSystem,      // Unit systems i.e. conversion factors
}

pub struct InternalClock {
    // Definition of the simulation length and time step
    pub timestep: f64,
    pub total_time: f64,

    // Runtime variables i.e. rolling time and step
    pub current_step: i64,
    pub current_time: f64,
}

impl InternalClock {
    pub fn new(timestep: f64, total_time: f64) -> InternalClock {
        InternalClock {
            current_step: 1,
            current_time: 0.0,
            timestep: timestep,
            total_time: total_time,
        }
    }
    pub fn tick(&mut self) -> () {
        self.current_step += 1;
        self.current_time += self.timestep;
    }
    pub fn reset(&mut self) -> () {
        self.current_step = 1;
        self.current_time = 0.0;
    }
    pub fn has_finished(&self) -> bool {
        self.current_time >= self.total_time
    }
}

pub struct Simulation {
    pub system: SystemDefinition,       // System definition
    pub integrator: DynamicsIntegrator, // Equations of motion numerical integrator
    pub clock: InternalClock,           // Internal clock for the simulation runtime
    pub potential_model: PotentialModel,
}

impl Simulation {
    pub fn from(yaml: &yaml_rust::Yaml) -> Simulation {
        let dynamics_setup = &yaml["dynamics"];

        let system_definition = &yaml["system"];
        let box_origin = to_vec_f64::<3>(&system_definition["origin"]);
        let box_vectors = system_definition["vectors"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| to_vec_f64::<3>(x))
            .collect::<Vec<[f64; 3]>>()
            .try_into()
            .unwrap();

        Simulation {
            system: SystemDefinition {
                origin: box_origin,
                vectors: box_vectors,
                atoms: load_atoms(&system_definition["atoms"]),
                units: UnitSystem::new(&yaml["units"]),
                periodicity: match &system_definition["periodicity"] {
                    Yaml::BadValue => [false, false, false],
                    Yaml::String(x) => match x.as_str() {
                        "xyz" => [true, true, true],
                        "xy" => [true, true, false],
                        "xz" => [true, false, true],
                        "yz" => [false, true, true],
                        "x" => [true, false, false],
                        "y" => [false, true, false],
                        "z" => [false, false, true],
                        _ => panic!("Unknown periodicity"),
                    },
                    Yaml::Array(x) => match x.len() {
                        3 => [
                            x[0].as_bool().unwrap(),
                            x[1].as_bool().unwrap(),
                            x[2].as_bool().unwrap(),
                        ],
                        _ => panic!("Unknown periodicity"),
                    },
                    _ => panic!("Unknown periodicity"),
                },
            },
            integrator: match dynamics_setup["integrator"]["type"].as_str().unwrap() {
                "verlet" => DynamicsIntegrator::Verlet(VerletIntegrator::from(&dynamics_setup)),
                _ => panic!("Unknown integrator"),
            },
            clock: InternalClock::new(
                dynamics_setup["timestep"].as_f64().unwrap(),
                dynamics_setup["total_time"].as_f64().unwrap(),
            ),
            potential_model: PotentialModel::from(&yaml["potential"]),
        }
    }
}

impl fmt::Display for SystemDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let definition = format!(
            "\n  Origin: {:?}\n  Vectors: {:?}\n  {:?}",
            self.origin, self.vectors, self.units
        );
        let atoms_description: String = self
            .atoms
            .iter()
            .map(|x| format!("  {}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "System definition: {}\nAtoms:\n{}",
            definition, atoms_description
        )
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.system, self.integrator)
    }
}
