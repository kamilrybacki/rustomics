use crate::simulation::Simulation;

use crate::io::input::parse_yaml;
use crate::utils::logger::SimulationLogger;

pub struct SimulationRunnerEngine {
    simulation: Simulation,
    logger: SimulationLogger,
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
        };
        return sim;
    }

    pub fn run(&mut self) -> () {
        if self.simulation.clock.current_step > 1 {
            self.simulation.clock.reset();
        }
        self.simulation
            .neighbors
            .update(&mut self.simulation.system);
        while !self.simulation.clock.has_finished() {
            self.simulation.integrator.next_step(
                &mut self.simulation.system.atoms,
                &self.simulation.potential_model,
                &mut self.simulation.neighbors,
                &self.simulation.system.units,
            );
            self.simulation
                .neighbors
                .update(&mut self.simulation.system);
            self.simulation.thermodynamics.update();
            self.logger.log_simulation_state(&self.simulation);
            if self.simulation.neighbors.log {
                self.logger
                    .construct_neighbors_list_log(&self.simulation.neighbors);
            }
            self.simulation.clock.tick();
        }
    }
}
