use super::Context;
use crate::handlers::Handler;
use clap::ArgMatches;
use tes3::esp::{Cell, EditorId, Reference};

pub struct DuplicateRefValidator {
    threshold: f32,
}

impl Handler<'_> for DuplicateRefValidator {
    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        _: &str,
        refs: &[&Reference],
        i: usize,
    ) {
        if reference.deleted == Some(true) {
            return;
        }
        for other in &refs[i + 1..] {
            if other.deleted == Some(true) {
                continue;
            }
            if other.id.eq_ignore_ascii_case(&reference.id)
                && reference.rotation == other.rotation
                && reference.scale.unwrap_or(1.) == other.scale.unwrap_or(1.)
                && self.translation(reference.translation, other.translation)
            {
                println!(
                    "Cell {} contains duplicate reference {} at position {:?} {:?}",
                    record.editor_id(),
                    reference.id,
                    reference.translation,
                    other.translation,
                );
            }
        }
    }
}

impl DuplicateRefValidator {
    pub fn new(args: &ArgMatches) -> Self {
        let threshold = args
            .get_one::<f32>("duplicatethreshold")
            .unwrap_or(&0.)
            .max(0.);
        DuplicateRefValidator { threshold }
    }

    fn translation(&self, a: [f32; 3], b: [f32; 3]) -> bool {
        if self.threshold == 0. {
            return a == b;
        }
        let [x1, y1, z1] = a;
        let [x2, y2, z2] = b;
        let d2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2);
        d2.abs() <= self.threshold
    }
}
