use crate::logic::algebra::euclidean_norm;

#[derive(Debug)]
pub struct UnitCell {
    pub vectors: [[f64; 3]; 3],      // Simulation box vectors
    pub constants: Option<[f64; 3]>, // Unit cell lattice constants
    pub volume: Option<f64>,         // Volume of the unit cell
}

impl UnitCell {
    pub fn new(origin: [f64; 3], vectors: [[f64; 3]; 3]) -> UnitCell {
        let mut initialized_cell = UnitCell {
            vectors,
            constants: None,
            volume: None,
        };
        initialized_cell.constants = Some(vectors.map(|x| {
            let mut sum = 0.0;
            for i in 0..3 {
                sum += x[i].powi(2);
            }
            sum.sqrt()
        }));
        initialized_cell.volume = Some(initialized_cell.calculate_cell_volume());
        initialized_cell
    }
    pub fn calculate_cell_volume(&self) -> f64 {
        self.vectors[2][0] * self.vectors[0][1] * self.vectors[1][2]
            + self.vectors[0][0] * self.vectors[1][1] * self.vectors[2][2]
            + self.vectors[1][0] * self.vectors[2][1] * self.vectors[0][2]
            - self.vectors[2][0] * self.vectors[1][1] * self.vectors[0][2]
            - self.vectors[1][0] * self.vectors[0][1] * self.vectors[2][2]
            - self.vectors[0][0] * self.vectors[2][1] * self.vectors[1][2]
    }
}

impl std::fmt::Display for UnitCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!("\n  Vectors: {:?}\n ", self.vectors);
        write!(f, "{}", definition)
    }
}

#[derive(Debug)]
pub struct SimulationBox {
    pub cell: UnitCell,
    pub origin: [f64; 3],       // Origin of the simulation box
    pub vectors: [[f64; 3]; 3], // Simulation box vectors
    pub versors: [[f64; 3]; 3], // Simulation box versors
    pub dimensions: [f64; 3],   // Dimensions of the simulation box
    pub replicas: [usize; 3],   // Number of replicas in each direction
    pub periodicity: [bool; 3], // Periodicity of the simulation box
}

impl SimulationBox {
    pub fn new(
        origin: [f64; 3],
        vectors: [[f64; 3]; 3],
        periodicity: [bool; 3],
        replicas: [usize; 3],
    ) -> SimulationBox {
        let mut new_box = SimulationBox {
            cell: UnitCell::new(origin, vectors),
            origin,
            vectors: [[0.0; 3]; 3],
            versors: [[0.0; 3]; 3],
            dimensions: [0.0; 3],
            replicas,
            periodicity,
        };
        new_box.calculate_box_vectors();
        new_box
    }
  fn calculate_box_vectors(&mut self) -> () {
      let mut new_vectors = self.cell.vectors;
      for i in 0..3 {
          new_vectors[i][i] *= self.replicas[i] as f64;
      }
      let new_dimensions = new_vectors.map(|x| euclidean_norm(&x));
      let mut new_versors = [[0.0; 3]; 3];
      for i in 0..3 {
          for j in 0..3 {
              new_versors[i][j] = new_vectors[i][j] / new_dimensions[i];
          }
      }
      self.vectors = new_vectors;
      self.dimensions = new_dimensions;
      self.versors = new_versors;
  }
}


impl std::fmt::Display for SimulationBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!(
            "Vectors: {:?}\n  Replicas: {:?}\n  Periodicity: {:?}\n",
            self.vectors, self.replicas, self.periodicity
        );
        write!(f, "{}", definition)
    }
}
