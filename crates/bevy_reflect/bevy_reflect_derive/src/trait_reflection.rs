use bevy_macro_utils::BevyManifest;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Attribute, ItemTrait, Token};

pub(crate) struct TraitInfo {
    item_trait: ItemTrait,
}

impl Parse for TraitInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![pub]) || lookahead.peek(Token![trait]) {
            let mut item_trait: ItemTrait = input.parse()?;
            item_trait.attrs = attrs;
            Ok(TraitInfo { item_trait })
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) fn reflect_trait(_args: &TokenStream, input: TokenStream) -> TokenStream {
    let trait_info = parse_macro_input!(input as TraitInfo);
    let item_trait = &trait_info.item_trait;
    let trait_ident = &item_trait.ident;
    let trait_vis = &item_trait.vis;
    let reflect_trait_ident = crate::utility::get_reflect_ident(&item_trait.ident.to_string());
    let bevy_reflect_path = BevyManifest::default().get_path("bevy_reflect");
    TokenStream::from(quote! {
        #item_trait

        #[derive(Clone)]
        #trait_vis struct #reflect_trait_ident {
            get_func: fn(&dyn #bevy_reflect_path::Reflect) -> Option<&dyn #trait_ident>,
            get_mut_func: fn(&mut dyn #bevy_reflect_path::Reflect) -> Option<&mut dyn #trait_ident>,
        }

        impl #reflect_trait_ident {
            pub fn get<'a>(&self, reflect_value: &'a dyn #bevy_reflect_path::Reflect) -> Option<&'a dyn #trait_ident> {
                (self.get_func)(reflect_value)
            }

            pub fn get_mut<'a>(&self, reflect_value: &'a mut dyn #bevy_reflect_path::Reflect) -> Option<&'a mut dyn #trait_ident> {
                (self.get_mut_func)(reflect_value)
            }
        }

        impl<T: #trait_ident + #bevy_reflect_path::Reflect> #bevy_reflect_path::FromType<T> for #reflect_trait_ident {
            fn from_type() -> Self {
                Self {
                    get_func: |reflect_value| {
                        reflect_value.downcast_ref::<T>().map(|value| value as &dyn #trait_ident)
                    },
                    get_mut_func: |reflect_value| {
                        reflect_value.downcast_mut::<T>().map(|value| value as &mut dyn #trait_ident)
                    }
                }
            }
        }
    })
}
