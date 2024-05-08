use tes3::esp::TES3Object;

use crate::handler_traits::RecordHandler;

use super::{Context, Handlers};

pub struct TestValidator {}

impl RecordHandler for TestValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, id: &String) {
        // println!("{}, {}", record.tag_str(), id);
    }
}

impl TestValidator {
    pub fn register(handlers: &mut Handlers) {
        handlers.register_record_handler(Box::new(TestValidator {}));
    }
}
