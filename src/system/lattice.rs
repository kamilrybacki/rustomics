use rayon::prelude::*;

use nalgebra::Vector3;

use crate::system::atom::Atom;
use crate::system::r#box::SimulationBox;

pub fn scale_cell_basis(atoms: &mut Vec<Atom>, simulation_box: &SimulationBox) -> () {
    println!("Scaling cell basis");
    atoms.par_iter_mut().for_each(|atom| {
        let mut scaled_position = Vector3::<f64>::zeros();
        for i in 0..3 {
            scaled_position[i] = atom.current.position[i] * simulation_box.cell.vectors[(i, i)];
        }
        atom.current.position = scaled_position;
    });
}

pub fn generate_lattice(atoms: &mut Vec<Atom>, simulation_box: &SimulationBox) -> () {
    println!("Generating lattice");
    let original_atoms_length = atoms.len();
    let x_replicas = simulation_box.replicas[0];
    let y_replicas = simulation_box.replicas[1];
    let z_replicas = simulation_box.replicas[2];

    let x_vector: Vector3<f64> = Vector3::new(
        simulation_box.cell.vectors[(0, 0)],
        simulation_box.cell.vectors[(0, 1)],
        simulation_box.cell.vectors[(0, 2)],
    );

    let y_vector: Vector3<f64> = Vector3::new(
        simulation_box.cell.vectors[(1, 0)],
        simulation_box.cell.vectors[(1, 1)],
        simulation_box.cell.vectors[(1, 2)],
    );

    let z_vector: Vector3<f64> = Vector3::new(
        simulation_box.cell.vectors[(2, 0)],
        simulation_box.cell.vectors[(2, 1)],
        simulation_box.cell.vectors[(2, 2)],
    );

    let mut generated_atoms = Vec::new();
    for x in 0..x_replicas {
        for y in 0..y_replicas {
            for z in 0..z_replicas {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                let mut replica_atoms: Vec<Atom> = atoms
                    .par_iter()
                    .map(|atom| {
                        let mut new_atom = atom.clone();
                        new_atom.current.position +=
                            x as f64 * x_vector;
                        new_atom.current.position +=
                            y as f64 * y_vector;
                        new_atom.current.position +=
                            z as f64 * z_vector;
                        new_atom
                    })
                    .collect::<Vec<Atom>>();
                generated_atoms.append(&mut replica_atoms);
            }
        }
    }
    atoms.append(&mut generated_atoms);
    let new_atoms_length = atoms.len();
    if new_atoms_length == original_atoms_length {
        return;
    }
    println!(
        "Generated {} atoms",
        new_atoms_length - original_atoms_length
    );
    atoms
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, atom)| atom.id = index as u64);
}
