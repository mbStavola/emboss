use emboss_common::EmbossingOptions;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;

use crate::emboss_token;
use crate::internals::env::EnvVarFallback;

#[derive(Deserialize)]
struct EnvVarsEmbossing {
    env_vars: Vec<EnvVarSpec>,

    #[serde(flatten)]
    options: EmbossingOptions,
}

#[derive(Deserialize)]
struct EnvVarSpec {
    env_var: String,
    
    key: Option<String>,

    #[serde(default)]
    fallback: EnvVarFallback,
}

pub fn emboss_envs(input: TokenStream) -> TokenStream {
    let EnvVarsEmbossing { env_vars, options } =
        match serde_tokenstream::from_tokenstream::<EnvVarsEmbossing>(&input.into()) {
            Ok(val) => val,
            Err(err) => return err.to_compile_error().into(),
        };

    let mut blocks = Vec::with_capacity(env_vars.len());
    for EnvVarSpec { env_var, key, fallback } in env_vars {
        let key = key.unwrap_or_else(|| env_var.clone());
        let value = match fallback {
            EnvVarFallback::Fail => quote! { env!(#env_var) },
            EnvVarFallback::Empty => quote! {{ 
                match option_env!(#env_var) { 
                    Some(val) => val, 
                    None => "",
                } 
            }},
            EnvVarFallback::Value(fallback) => quote! {{ 
                match option_env!(#env_var) { 
                    Some(val) => val, 
                    None => #fallback,
                } 
            }},
        };
        
        let tokens = emboss_token(&key, value, options.clone());
        blocks.push(tokens);
    }

    quote! {
        #(#blocks)*
    }.into()
}
