use super::Context;
use crate::handlers::Handler;
use tes3::esp::TES3Object;

pub struct TestValidator {}

impl Handler<'_> for TestValidator {
    fn on_record(&mut self, _: &Context, _: &TES3Object, _: &'static str, _: &String) {
        // println!("{}, {}", record.tag_str(), id);
    }
}
