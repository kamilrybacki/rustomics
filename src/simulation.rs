extern crate yaml_rust;

use core::fmt;

use yaml_rust::Yaml;


use crate::dynamics::integrators::verlet::VerletIntegrator;
use crate::dynamics::neighbours::NeighboursList;
use crate::dynamics::DynamicsIntegrator;
use crate::statics::models::PotentialModel;
use crate::statics::energetics::SystemEnergetics;
use crate::system::SystemDefinition;

pub struct InternalClock {
    // Definition of the simulation length and time step
    pub timestep: f64,
    pub total_time: f64,

    // Runtime variables i.e. rolling time and step
    pub current_step: u64,
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
    pub neighbours: NeighboursList,
    pub energetics: SystemEnergetics
}

impl Simulation {
    pub fn from(yaml: &yaml_rust::Yaml) -> Simulation {
        let dynamics_setup = &yaml["dynamics"];
        let system_definition = &yaml["system"];

        let timestep = dynamics_setup["timestep"].as_f64().unwrap();
        let calculated_total_time = match &dynamics_setup["total_time"] {
            Yaml::Real(x) => x.parse::<f64>().unwrap(),
            Yaml::BadValue => dynamics_setup["steps"].as_i64().unwrap() as f64 * timestep,
            _ => dynamics_setup["steps"].as_i64().unwrap() as f64 * timestep,
        };

        Simulation {
            system: SystemDefinition::from(system_definition),
            potential_model: PotentialModel::from(&yaml["potential"]),
            integrator: match dynamics_setup["integrator"]["type"].as_str().unwrap() {
                "verlet" => DynamicsIntegrator::Verlet(VerletIntegrator::from(&dynamics_setup)),
                _ => panic!("Unknown integrator"),
            },
            clock: InternalClock::new(
                timestep,
                calculated_total_time,
            ),
            neighbours: NeighboursList::from(&yaml["neighbours"]),
            energetics: SystemEnergetics::new(),
        }
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.system, self.integrator)
    }
}
