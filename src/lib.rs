use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, ItemEnum};

#[proc_macro_attribute]
/// Convert an enum into a set of `struct`s
/// ### Examples
/// ```no_run
/// pub enum Type {
/// Int(i32),
/// Float(f32),
/// Str(String)
/// }
/// ```
pub fn desugar_enum(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Item);
    let input = is_enum(ast);
    let vis = input.vis;
    let variants = input.variants.iter();
    let stream = quote! {
        #(#vis struct #variants;)*
    };
    TokenStream::from(stream)
}

fn is_enum(item: Item) -> ItemEnum {
    if let Item::Enum(inner) = item {
        return inner;
    } else {
        panic!("expected an enum")
    }
}
