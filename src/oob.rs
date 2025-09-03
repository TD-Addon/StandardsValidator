use std::collections::HashMap;

use tes3::esp::{Cell, Plugin};

use crate::util::get_cell_grid;

pub fn fix_oob(plugin: &mut Plugin) {
    let mut exteriors: HashMap<_, _> = plugin
        .objects_of_type_mut::<Cell>()
        .filter_map(|cell| Some((cell.exterior_coords()?, cell)))
        .collect();

    let mut out_of_bounds = vec![];

    for (grid, cell) in &exteriors {
        for (key, reference) in &cell.references {
            if reference.deleted == Some(true) || reference.moved_cell.is_some() {
                continue;
            }

            let [x, y, _] = reference.translation;
            let actual_grid = get_cell_grid(x as f64, y as f64);
            let dx = (grid.0 - actual_grid.0).abs();
            let dy = (grid.1 - actual_grid.1).abs();

            // In the correct cell
            if dx == 0 && dy == 0 {
                continue;
            }

            // More than 1 cell away
            if dx > 1 || dy > 1 {
                println!(
                    "Not moving {} {:?} from cell {:?} as cell {:?} is too far away",
                    reference.id, key, grid, actual_grid
                );
                continue;
            }

            // In an undefined cell
            if !exteriors.contains_key(&actual_grid) {
                println!(
                    "Not moving {} {:?} from cell {:?} as cell {:?} is not in this file",
                    reference.id, key, grid, actual_grid
                );
                continue;
            }

            // In a neighboring cell
            println!(
                "Moving {} {:?} from {:?} to {:?}",
                reference.id, key, grid, actual_grid
            );
            out_of_bounds.push((*grid, actual_grid, *key));
        }
    }

    out_of_bounds
        .into_iter()
        .try_for_each(|(old_grid, new_grid, key)| {
            let old_cell = exteriors.get_mut(&old_grid)?;
            let reference = old_cell.references.remove(&key)?;

            let new_cell = exteriors.get_mut(&new_grid)?;
            new_cell.references.insert(key, reference);

            Some(())
        });
}
