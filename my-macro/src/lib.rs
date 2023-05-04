use proc_macro::TokenStream;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(_: TokenStream) -> TokenStream {
    let warning = proc_macro_warning::Warning::new_deprecated("test").old("foo").new("bar").build();

    quote::quote!{ #warning }.into()
}
