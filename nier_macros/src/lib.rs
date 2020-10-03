#![feature(format_args_capture)]
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, DeriveInput, Lit, Meta, MetaList, MetaNameValue};

use cache_macro::cache;
use lru_cache::LruCache;
use ron::de::from_str;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;

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

#[proc_macro_derive(Automaton, attributes(state, alphabet, nier))]
pub fn derive_automaton(input: TokenStream) -> TokenStream {
    derive_get_attrs(input, gen_automaton_impl)
}

#[proc_macro_derive(Deterministic, attributes(state, alphabet, source, nier))]
pub fn derive_deterministic(input: TokenStream) -> TokenStream {
    derive_get_attrs(input, gen_deterministic_impl)
}

#[proc_macro_derive(Acceptor, attributes(state, alphabet, source, nier))]
pub fn derive_acceptor(input: TokenStream) -> TokenStream {
    derive_get_attrs(input, gen_acceptor_impl)
}

fn set_nier_attrs(option: &Meta, mut attrs: &mut HashMap<String, String>) {
    match option {
        Meta::NameValue(MetaNameValue {
            ref path, ref lit, ..
        }) => match (path.get_ident(), lit) {
            (Some(id), Lit::Str(lit))
                if { id == "state" || id == "alphabet" || id == "source" } =>
            {
                attrs.insert(id.to_string(), lit.value());
            }
            _ => {}
        },
        Meta::List(MetaList {
                ref path,
                ref nested,
                ..
            }) => match path.get_ident() {
                Some(id) if { id == "nier" } => {
                    for p in nested.pairs() {
                        if let syn::NestedMeta::Meta(option) = p.value() {
                           set_nier_attrs(option, &mut attrs);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
    }
}

fn derive_get_attrs<F>(tokens: TokenStream, gen_attrs_impl: F) -> TokenStream
where
    F: Fn(&syn::Ident, HashMap<String, String>) -> TokenStream,
{
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let mut attrs = HashMap::new();
    for option in ast.attrs.into_iter() {
        let option = option.parse_meta().unwrap();
        set_nier_attrs(&option, &mut attrs);
    }
    gen_attrs_impl(&ast.ident, attrs)
}

fn gen_automaton_impl(name: &syn::Ident, attrs: HashMap<String, String>) -> TokenStream {
    let state_name = syn::Ident::new(&attrs.get("state").unwrap(), syn::export::Span::call_site());

    let gen = quote! {
        impl Automaton<#state_name> for #name {}
    };
    gen.into()
}

#[derive(Debug, Deserialize)]
struct IntermediateDeterministic {
    initial_state: String,
    transitions: std::collections::HashSet<(String, String, String)>,
}

#[derive(Debug, Deserialize)]
struct IntermediateAcceptor {
    accept_states: std::collections::HashSet<String>,
}

#[cache(LruCache : LruCache::new(20))]
fn load_intermediate_source(file_name: String) -> String {
    let input_path = format!(
        "{}/{}",
        env::current_dir().unwrap().to_str().unwrap(),
        file_name
    );
    let mut result = String::new();
    if let Err(e) = File::open(&input_path)
        .expect("Failed opening file")
        .read_to_string(&mut result)
    {
        println!("{:?}", e);
    }
    result
}

fn gen_deterministic_impl(name: &syn::Ident, attrs: HashMap<String, String>) -> TokenStream {
    let state_name = syn::Ident::new(&attrs.get("state").unwrap(), syn::export::Span::call_site());
    let alphabet_name = syn::Ident::new(
        &attrs.get("alphabet").unwrap(),
        syn::export::Span::call_site(),
    );
    let file_name = attrs.get("source").unwrap().to_string();

    let IntermediateDeterministic {
        initial_state,
        transitions,
    } = match from_str(&load_intermediate_source(file_name)) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    let initial_state = syn::Ident::new(&initial_state, syn::export::Span::call_site());
    let transitions = transitions.iter().map(|(pstate, alpha, qstate)| {
        let pstate = syn::Ident::new(&pstate, syn::export::Span::call_site());
        let alpha = syn::Ident::new(&alpha, syn::export::Span::call_site());
        let qstate = syn::Ident::new(&qstate, syn::export::Span::call_site());
        quote!(
            (#state_name::#pstate, #alphabet_name::#alpha) => Ok(#state_name::#qstate),
        )
    });

    let gen = quote! {
        impl Deterministic<#state_name, #alphabet_name> for #name {
            fn initial() -> #state_name {
                #state_name::#initial_state
            }

            fn delta(
                state: &#state_name,
                input: #alphabet_name,
            ) -> Result<#state_name, Reject<#state_name, #alphabet_name>> {
                match (state, input) {
                    #(#transitions)*
                    _ => Err(Reject::InvalidInput(input.clone())),
                }
            }
        }
    };
    gen.into()
}

fn gen_acceptor_impl(name: &syn::Ident, attrs: HashMap<String, String>) -> TokenStream {
    let state_name = syn::Ident::new(&attrs.get("state").unwrap(), syn::export::Span::call_site());
    let file_name = attrs.get("source").unwrap().to_string();

    let IntermediateAcceptor { accept_states } =
        match from_str(&load_intermediate_source(file_name)) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            }
        };

    let accept_states = accept_states.iter().map(|fstate| {
        let fstate = syn::Ident::new(fstate, syn::export::Span::call_site());
        quote!(
            #state_name::#fstate => Ok(#state_name::#fstate),
        )
    });

    let gen = quote! {
        impl Acceptor<#state_name> for #name {
            fn accept(current: &#state_name) -> Result<#state_name, Reject<#state_name, ()>> {
                match current {
                    #(#accept_states)*
                    _ => Err(Reject::NotAccept(current.clone())),
                }
            }
        }
    };
    gen.into()
}
