use super::Context;
use crate::handlers::Handler;
use tes3::esp::{EditorId, TES3Object, TypeInfo};

pub struct LightValidator {}

impl Handler<'_> for LightValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object) {
        if let TES3Object::Light(light) = record {
            if light.data.time > 10000 {
                println!(
                    "{} {} lasts for {} seconds",
                    record.type_name(),
                    record.editor_id(),
                    light.data.time
                );
            }
        }
    }
}
