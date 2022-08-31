use proc_macro2::{Ident, TokenStream};
use quote::quote;

use syn::{parse::Error, spanned::Spanned as _, Data, DeriveInput, Index, Member};

pub fn derive_clonable_default(input: DeriveInput) -> TokenStream {
    let ethers_contract = ethers_core::macros::ethers_contract_crate();
    let members = if let Data::Struct(ref s) = input.data {
        s.fields.iter().enumerate().map(|(index, field)| {
            field
                .ident
                .as_ref()
                .map_or(Member::Unnamed(Index::from(index)), |x: &Ident| Member::Named(x.clone()))
        })
    } else {
        return Error::new(input.span(), format!("ClonableDefault cannot be derived for enum"))
            .to_compile_error()
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #ethers_contract::ClonableDefault for #name #ty_generics #where_clause {
            fn clonable_default() -> Self {
                Self {
                    #(#members: #ethers_contract::ClonableDefault::clonable_default(),)*
                }
            }
        }

        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #ethers_contract::ClonableDefault::clonable_default()
            }
        }
    }
}
