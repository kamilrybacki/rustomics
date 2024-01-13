use core::fmt;

use rayon::prelude::*;
use yaml_rust::Yaml;

use crate::dynamics::integrators::verlet::VerletIntegrator;
use crate::dynamics::neighbours::NeighboursList;
use crate::dynamics::DynamicsIntegrator;
use crate::statics::energetics::SystemEnergetics;
use crate::statics::models::PotentialModel;
use crate::system::SystemDefinition;
use crate::thermodynamics::Thermodynamics;

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
    pub energetics: SystemEnergetics,
    pub thermodynamics: Thermodynamics,
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
            clock: InternalClock::new(timestep, calculated_total_time),
            neighbours: NeighboursList::from(&yaml["neighbours"]),
            energetics: SystemEnergetics::new(),
            thermodynamics: Thermodynamics::from(&yaml["thermodynamics"]),
        }
    }
    pub fn apply_units_system(&mut self) -> () {
        self.clock.timestep *= self.system.units.time.0;
        self.clock.total_time *= self.system.units.time.0;
        self.clock.current_time *= self.system.units.time.0;
        self.system.atoms.par_iter_mut().for_each(|atom| {
            atom.mass *= self.system.units.mass.0;
            atom.charge *= self.system.units.charge.0;
            atom.current.position *= self.system.units.distance.0;
            atom.current.velocity *= self.system.units.distance.0 / self.system.units.time.0;
            atom.current.force *= self.system.units.force.0;
            atom.current.kinetic_energy *= self.system.units.energy.0;
            atom.current.potential_energy *= self.system.units.energy.0;
            atom.current.total_energy *= self.system.units.energy.0;
            atom.previous.position *= self.system.units.distance.0;
            atom.previous.velocity *= self.system.units.distance.0 / self.system.units.time.0;
            atom.previous.force *= self.system.units.force.0;
            atom.previous.kinetic_energy *= self.system.units.energy.0;
            atom.previous.potential_energy *= self.system.units.energy.0;
            atom.previous.total_energy *= self.system.units.energy.0;
        });
        self.potential_model.apply_units_system(&self.system.units);
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.system, self.integrator)
    }
}
