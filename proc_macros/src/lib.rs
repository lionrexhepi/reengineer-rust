use proc_macro::TokenStream;
use quote::quote;
use syn::ItemEnum;

#[proc_macro_attribute]
pub fn count_ids(_attr: TokenStream, target: TokenStream) -> TokenStream {
    let target_enum = syn::parse::<ItemEnum>(target).unwrap();
    let name = target_enum.clone().ident;
    let fields = target_enum
        .clone()
        .variants
        .iter()
        .map(|f| f.ident.clone())
        .collect::<Vec<_>>();

    let inners = target_enum
        .variants
        .iter()
        .map(|v| {
            v.fields
                .iter()
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .ty
                .clone()
        })
        .collect::<Vec<_>>();

    let reps = (0..(fields.len() as u8)).collect::<Vec<_>>();

    let implem = quote! {
        #target_enum

        impl #name {
            fn repr(&self) -> u8 {
                match self {
                    #(Self::#fields(_) => #reps,)*
                }
            }

            fn from_ints(repr: u8, variant: u8) -> Option<Self> {
                match repr {
                    #(#reps => Some(Self::#fields(#inners::from_id(variant))),)*
                    _ => None,
                }
            }

            fn variant_id(&self) -> u8 {
                match self {
                    #(Self::#fields(inner) => inner.id(),)*
                }
            }

            fn inner_handler(&self) -> &dyn BlockHandler {
                match self {
                    #(Self::#fields(inner) => inner,)*
                }
            }
        }


    };

    implem.into()
}
