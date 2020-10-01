use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(State)]
pub fn derive_state(input: TokenStream) -> TokenStream { 
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics nier::State for #name #ty_generics #where_clause {}
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Alphabet)]
pub fn derive_alphabet(input: TokenStream) -> TokenStream { 
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics nier::Alphabet for #name #ty_generics #where_clause {}
    };

    proc_macro::TokenStream::from(expanded)
}

// use ron::de::from_reader;
// use serde::Deserialize;

// use nier::*;

// fn main() {
//     let input_path = format!("{}/examples/example.ron", env!("CARGO_MANIFEST_DIR"));
//     let f = File::open(&input_path).expect("Failed opening file");
//     let config: Config = match from_reader(f) {
//         Ok(x) => x,
//         Err(e) => {
//             println!("Failed to load config: {}", e);

//             std::process::exit(1);
//         }
//     };

//     println!("Config: {:?}", &config);
// }

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }