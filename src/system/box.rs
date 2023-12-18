pub struct SimulationBox {
    pub origin: [f64; 3],       // Origin of the simulation box
    pub vectors: [[f64; 3]; 3], // Simulation box vectors
    pub periodicity: [bool; 3], // Periodicity of the simulation box
}

impl SimulationBox {
    pub fn new(origin: [f64; 3], vectors: [[f64; 3]; 3], periodicity: [bool; 3]) -> SimulationBox {
        SimulationBox {
            origin,
            vectors,
            periodicity,
        }
    }
}

impl std::fmt::Display for SimulationBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!(
            "\n  Origin: {:?}\n  Vectors: {:?}\n ",
            self.origin, self.vectors
        );
        write!(f, "{}", definition)
    }
}
