use proc_macro::TokenStream;

pub(crate) mod common;

mod bodypart_data;
mod broken_data;
mod class_data;
mod mwscript_data;
mod project_data;
mod services_data;
mod spells_data;
mod supplies_data;
mod travel_data;
mod uniques_data;

#[proc_macro]
pub fn get_bodypart_data(_: TokenStream) -> TokenStream {
    bodypart_data::generate().into()
}

#[proc_macro]
pub fn get_broken_data(_: TokenStream) -> TokenStream {
    broken_data::generate().into()
}

#[proc_macro]
pub fn get_class_data(_: TokenStream) -> TokenStream {
    class_data::generate().into()
}

#[proc_macro]
pub fn get_joined_commands(_: TokenStream) -> TokenStream {
    mwscript_data::generate_joined_commands().into()
}

#[proc_macro]
pub fn get_khajiit_script(_: TokenStream) -> TokenStream {
    mwscript_data::generate_khajiit_script().into()
}

#[proc_macro]
pub fn get_project_data(_: TokenStream) -> TokenStream {
    project_data::generate().into()
}

#[proc_macro]
pub fn get_barter_classes(_: TokenStream) -> TokenStream {
    services_data::generate_barter_classes().into()
}

#[proc_macro]
pub fn get_spell_vendor_classes(_: TokenStream) -> TokenStream {
    services_data::generate_spell_vendor_classes().into()
}

#[proc_macro]
pub fn get_spell_data(_: TokenStream) -> TokenStream {
    spells_data::generate().into()
}

#[proc_macro]
pub fn get_supplies_data(_: TokenStream) -> TokenStream {
    supplies_data::generate().into()
}

#[proc_macro]
pub fn get_travel_classes(_: TokenStream) -> TokenStream {
    travel_data::generate().into()
}

#[proc_macro]
pub fn get_uniques(_: TokenStream) -> TokenStream {
    uniques_data::generate().into()
}
