use quote::quote;

fn pack_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        let ty = &bi.ast().ty;
        quote! {
            <#ty as knapsack::Pack>::pack(&#bi, &mut packer)?;
        }
    });

    s.gen_impl(quote! {
        gen impl knapsack::Pack for @Self {
            fn pack<P: knapsack::PackBytes>(&self, mut packer: P) -> Result<(), knapsack::PackError> {
                #body

                Ok(())
            }
        }
    })
}

fn unpack_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        let ty = &bi.ast().ty;
        quote! {
            <#ty as knapsack::Unpack>::unppack(&#bi, &mut unpacker)?,
        }
    });

    s.gen_impl(quote! {
        gen impl knapsack::Unpack for @Self {

            fn unpack<U: knapsack::UnpackBytes>(mut unpacker: U) -> Result<Self, knapsack::UnpackError> {
                Ok(Self { #body })
            }
        }
    })
}

synstructure::decl_derive!([Pack] => pack_derive);
synstructure::decl_derive!([Unpack] => unpack_derive);
