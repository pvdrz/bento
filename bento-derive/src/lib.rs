use quote::quote;

fn pack_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| {
        let ty = &bi.ast().ty;
        quote! {
            <#ty as bento::Pack>::pack(&#bi, &mut packer)?;
        }
    });

    s.gen_impl(quote! {
        gen impl bento::Pack for @Self {
            fn pack<P: bento::PackBytes>(&self, mut packer: P) -> Result<(), bento::PackError> {
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
            <#ty as bento::Unpack>::unppack(&#bi, &mut unpacker)?,
        }
    });

    s.gen_impl(quote! {
        gen impl bento::Unpack for @Self {

            fn unpack<U: bento::UnpackBytes>(mut unpacker: U) -> Result<Self, bento::UnpackError> {
                Ok(Self { #body })
            }
        }
    })
}

synstructure::decl_derive!{
    [Pack] => 
    /// Derive macro available if bentō is built with `features = ["derive"]`.
    pack_derive
}
synstructure::decl_derive!{
    [Unpack] => 
    /// Derive macro available if bentō is built with `features = ["derive"]`.
    unpack_derive
}
