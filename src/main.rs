use tes3::esp::Plugin;
use validators::{Context, Mode, Validator};

mod handler_traits;
mod validators;

fn main() -> std::io::Result<()> {
    let mut plugin = Plugin::new();
    plugin.load_path("D:\\Program Files (x86)\\Morrowind\\Data Files\\TR_Mainland.esm")?;
    let mut validator = Validator::new(Context { mode: Mode::PT });
    validators::test::TestValidator::register(&mut validator);
    validator.validate(&plugin.objects);
    Ok(())
}
