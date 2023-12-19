#[derive(Debug)]
pub struct UnitCell {
    pub origin: [f64; 3],       // Origin of the simulation box
    pub vectors: [[f64; 3]; 3], // Simulation box vectors
}

impl std::fmt::Display for UnitCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!(
            "\n  Origin: {:?}\n  Vectors: {:?}\n ",
            self.origin, self.vectors
        );
        write!(f, "{}", definition)
    }
}

#[derive(Debug)]
pub struct SimulationBox {
    pub cell: UnitCell,
    pub replicas: [usize; 3], // Number of replicas in each direction
    pub periodicity: [bool; 3], // Periodicity of the simulation box
}

impl SimulationBox {
    pub fn new(origin: [f64; 3], vectors: [[f64; 3]; 3], periodicity: [bool; 3], replicas: [usize; 3]) -> SimulationBox {
        SimulationBox {
            cell: UnitCell {
                origin,
                vectors,
            },
            replicas,
            periodicity,
        }
    }
}

impl std::fmt::Display for SimulationBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!("{}", self.cell);
        write!(f, "{}", definition)
    }
}
