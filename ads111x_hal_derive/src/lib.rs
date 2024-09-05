use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(ConfigConversion, attributes(config_mask, config_flag))]
pub fn derive_config_conversion(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mask = get_mask_attr(&input.attrs);
    let variants = match input.data {
        Data::Enum(ref data) => &data.variants,
        _ => panic!("ConfigConversion can only be derived for enums"),
    };

    let from_enum_arms = variants.iter().map(|v| {
        let variant = &v.ident;
        let flag = get_flag_attr(&v.attrs);
        quote! {
            #name::#variant => #flag
        }
    });

    let from_config_arms = variants.iter().map(|v| {
        let variant = &v.ident;
        let flag = get_flag_attr(&v.attrs);
        quote! {
            value if value == #flag.bits() => #name::#variant,
        }
    });

    let expanded = quote! {
        impl From<#name> for ADS111xConfig {
            fn from(value: #name) -> Self {
                match value {
                    #(#from_enum_arms,)*
                }
            }
        }

        impl From<ADS111xConfig> for #name {
            fn from(config: ADS111xConfig) -> Self {
                let value = config.intersection(#mask).bits();
                match value {
                    #(#from_config_arms)*
                    _ => panic!("Invalid configuration for {}: {:#06x}", stringify!(#name), value),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
fn get_mask_attr(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("config_mask"))
        .map(|attr| attr.parse_args().unwrap())
        .unwrap_or_else(|| panic!("config_mask attribute is required"))
}

fn get_flag_attr(attrs: &[syn::Attribute]) -> proc_macro2::TokenStream {
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("config_flag"))
        .map(|attr| attr.parse_args().unwrap())
        .unwrap_or_else(|| {
            panic!("config_flag attribute is required for each variant")
        })
}
