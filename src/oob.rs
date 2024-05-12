use std::collections::HashMap;

use tes3::esp::{Cell, TES3Object};

use crate::util::{get_cell_grid, get_cell_name};

fn build_grid_map(records: &mut Vec<TES3Object>) -> HashMap<(i32, i32), *mut Cell> {
    let mut out: HashMap<(i32, i32), *mut Cell> = HashMap::new();
    for record in records {
        if let TES3Object::Cell(cell) = record {
            if cell.is_exterior() {
                out.insert(cell.data.grid, cell);
            }
        }
    }
    return out;
}

pub fn fix_oob(records: &mut Vec<TES3Object>) {
    let cells = build_grid_map(records);
    for cr in cells.values() {
        let cell = *cr;
        unsafe {
            (*cell).references.retain(|key, reference| {
                if reference.deleted.unwrap_or(false) {
                    return true;
                }
                let [x, y, _] = reference.translation;
                let actual_grid = get_cell_grid(x as f64, y as f64);
                let dx = ((*cell).data.grid.0 - actual_grid.0).abs();
                let dy = ((*cell).data.grid.1 - actual_grid.1).abs();
                if dx > 1 || dy > 1 {
                    println!(
                        "Not moving {} from {} as [{}, {}] is too far away",
                        reference.id,
                        get_cell_name(&*cell),
                        actual_grid.0,
                        actual_grid.1
                    );
                } else if dx > 0 || dy > 0 {
                    if let Some(target_cell) = cells.get(&actual_grid) {
                        println!(
                            "Moving {} from {} to [{}, {}]",
                            reference.id,
                            get_cell_name(&*cell),
                            actual_grid.0,
                            actual_grid.1
                        );
                        (*(*target_cell))
                            .references
                            .insert(key.clone(), reference.clone());
                        return false;
                    } else {
                        println!(
                            "Not moving {} from {} as cell [{}, {}] is not in this file",
                            reference.id,
                            get_cell_name(&*cell),
                            actual_grid.0,
                            actual_grid.1
                        );
                    }
                }
                return true;
            });
        }
    }
}
