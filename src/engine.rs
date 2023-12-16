use crate::setup::Simulation;

use crate::data::load::parse_yaml;
use crate::system::thermodynamics::Thermodynamics;
use crate::utils::logger::SimulationLogger;

pub struct SimulationRunnerEngine {
    simulation: Simulation,
    logger: SimulationLogger,
    thermodynamics: Thermodynamics,
}

impl SimulationRunnerEngine {
    pub fn from_script(script_filepath: &str) -> SimulationRunnerEngine {
        let extension = script_filepath.split(".").last().unwrap();
        let script: yaml_rust::Yaml = match extension {
            "yaml" => parse_yaml(script_filepath),
            _ => panic!("Unknown file extension"),
        };
        let sim = SimulationRunnerEngine {
            simulation: Simulation::from(&script),
            logger: match &script["logger"] {
                yaml_rust::Yaml::BadValue => SimulationLogger::default(),
                _ => SimulationLogger::from(&script["logger"]),
            },
            thermodynamics: Thermodynamics::from(&script["thermodynamics"]),
        };
        return sim;
    }
    pub fn run(&mut self) -> () {
        if self.simulation.clock.current_step > 1 {
            self.simulation.clock.reset();
        }
        while !self.simulation.clock.has_finished() {
            self.simulation.integrator.next(
                &mut self.simulation.system.atoms,
                &self.simulation.system.vectors,
                &self.simulation.system.periodicity,
            );
            self.simulation
                .potential_model
                .update(&self.simulation.system.atoms);
            self.thermodynamics.update(&self.simulation);
            self.logger.log(&self.simulation);
            self.simulation.clock.tick();
        }
    }
}
