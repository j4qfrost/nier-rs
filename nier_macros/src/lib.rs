#![feature(format_args_capture)]
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

// Some ideas

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;
use std::env;


#[derive(Debug, Deserialize)]
struct IntermediateDFA {
    name: String,
    state_name: String,
    states: Vec<String>,
    alphabet_name: String,
    alphabet: Vec<String>,
    initial_state: String,
    transitions: std::collections::HashSet<(String, String, String)>,
    accept_states: Vec<String>,
}

#[proc_macro]
pub fn deserialize_dfa(args: TokenStream) -> TokenStream {
    let strip = args.to_string().replace(" ", "");
    let file_name = strip.as_str();

    let input_path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), file_name);
    let f = File::open(&input_path).expect("Failed opening file");
    let IntermediateDFA {
            name,
            state_name,
            states,
            alphabet_name,
            alphabet,
            initial_state,
            transitions,
            accept_states,
        } = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    let mut transformed_states = "".to_string();
    for s in states {
        transformed_states.push_str(&format!("{},", s));
    }
    let mut transformed_alphabet = "".to_string();
    for a in alphabet {
        transformed_alphabet.push_str(&format!("{},", a));
    }

    let mut transformed_transitions = "".to_string();
    for (state, input, pstate) in &transitions {
        transformed_transitions.push_str(&format!("({state_name}::{state}, {alphabet_name}::{input}) => Ok({state_name}::{pstate}),"));
    }
    let mut transformed_accept_states = "".to_string();
    for state in &accept_states {
        transformed_accept_states.push_str(&format!("{state_name}::{state} => Ok({state_name}::{state}),"));
    }

    let expanded = quote! {
        use std::hash::Hash;

        struct r#name {}

        #[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
        enum r#state_name {
            r#transformed_states
        }

        #[derive(Debug, Copy, Clone)]
        enum r#alphabet_name {
            r#transformed_alphabet
        }

        impl Automaton<r#state_name> for r#name {}

        impl Deterministic<r#state_name, r#alphabet_name> for r#name {
            fn initial() -> r#state_name {
                r#state_name::r#initial_state
            }

            fn delta(
                state: &r#state_name,
                input: r#alphabet_name,
            ) -> Result<r#state_name, Reject<r#state_name, r#alphabet_name>> {
                match (state, input) {
                    r#transformed_transitions
                    _ => Err(Reject::InvalidInput(input.clone())),
                }
            }
        }

        impl Acceptor<#state_name> for r#name {
            fn accept(current: &r#state_name) -> Result<r#state_name, Reject<r#state_name, ()>> {
                match current {
                    r#transformed_accept_states
                    _ => Err(Reject::NotAccept(current.clone())),
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}