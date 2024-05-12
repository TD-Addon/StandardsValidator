use super::Context;
use crate::{handlers::Handler, util::get_cell_name};
use clap::ArgMatches;
use rayon::prelude::*;
use tes3::esp::{Cell, Reference};

pub struct DuplicateRefValidator {
    threshold: f32,
}

fn equals(a: &[f32; 3], b: &[f32; 3]) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    return true;
}

impl Handler<'_> for DuplicateRefValidator {
    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        _: &String,
        refs: &Vec<&Reference>,
        i: usize,
    ) {
        if reference.deleted.unwrap_or(false) {
            return;
        }
        unsafe { refs.get_unchecked(i + 1..) }
            .par_iter()
            .for_each(|other| {
                if other.deleted.unwrap_or(false) {
                    return;
                }
                if other.id.eq_ignore_ascii_case(&reference.id)
                    && equals(&reference.rotation, &other.rotation)
                    && reference.scale.unwrap_or(1.) == other.scale.unwrap_or(1.)
                    && self.translation(&reference.translation, &other.translation)
                {
                    println!(
                    "Cell {} contains duplicate reference {} at position [{}, {}, {}] [{}, {}, {}]",
                    get_cell_name(record),
                    reference.id,
                    reference.translation[0],
                    reference.translation[1],
                    reference.translation[2],
                    other.translation[0],
                    other.translation[1],
                    other.translation[2]
                );
                }
            });
    }
}

impl DuplicateRefValidator {
    pub fn new(args: &ArgMatches) -> Self {
        let threshold = args
            .get_one::<f32>("duplicatethreshold")
            .unwrap_or(&0.)
            .max(0.);
        return DuplicateRefValidator { threshold };
    }

    fn translation(&self, a: &[f32; 3], b: &[f32; 3]) -> bool {
        if self.threshold == 0. {
            return equals(a, b);
        }
        let [x1, y1, z1] = a;
        let [x2, y2, z2] = b;
        let d2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2);
        return d2.abs() <= self.threshold;
    }
}
