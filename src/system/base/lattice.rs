use rayon::prelude::*;

use crate::system::base::atom::Atom;
use crate::system::r#box::SimulationBox;

pub fn generate_lattice(
    atoms: &mut Vec<Atom>,
    simulation_box: &SimulationBox,
) -> () {
  println!("Generating lattice");
  let original_atoms_length = atoms.len();
  let mut generated_atoms = atoms
    .par_iter()
    .map(|atom| {
      let x_replicas = simulation_box.replicas[0];
      let y_replicas = simulation_box.replicas[1];
      let z_replicas = simulation_box.replicas[2];
      let mut new_atoms = Vec::new();
      for x in 0..x_replicas {
        for y in 0..y_replicas {
          for z in 0..z_replicas {
            if x == 0 && y == 0 && z == 0 {
              continue;
            }
            let mut new_atom = atom.clone();
            new_atom.position[0] += x as f64 * simulation_box.cell.vectors[0][0];
            new_atom.position[1] += y as f64 * simulation_box.cell.vectors[1][1];
            new_atom.position[2] += z as f64 * simulation_box.cell.vectors[2][2];
            new_atoms.push(new_atom);
          }
        }
      }
      new_atoms
    })
    .flatten()
    .collect::<Vec<Atom>>();
  atoms.append(&mut generated_atoms);
  let new_atoms_length = atoms.len();
  println!("Generated {} atoms", new_atoms_length - original_atoms_length);
  atoms
    .par_iter_mut()
    .enumerate()
    .for_each(|(index, atom)| atom.id = index as u64);
}
