use super::Context;
use crate::handler_traits::Handler;
use tes3::esp::TES3Object;

pub struct TestValidator {}

impl Handler for TestValidator {
    fn on_record(&mut self, _: &Context, _: &TES3Object, _: &String) {
        // println!("{}, {}", record.tag_str(), id);
    }
}
