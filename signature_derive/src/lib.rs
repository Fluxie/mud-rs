use syn::__private::quote;
use syn::{Data, Ident};

#[proc_macro_derive(Signable)]
pub fn signable_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse.
    let input: syn::DeriveInput = syn::parse_macro_input!(input);
    let input_struct = &input.ident;
    let input_members: Vec<Ident> = match input.data {
        Data::Struct(fields) => fields
            .fields
            .iter()
            .filter(|f| f.ident.is_some())
            .map(|f| f.ident.as_ref().unwrap().clone())
            .collect(),
        _ => todo!("Unsupported"),
    };
    let input_member_count = input_members.len();

    // Implement "Signable" trait for the input struct.
    let signable_trait = quote::quote!(
        impl signature::Signable for #input_struct {

            fn sign(&self) -> signature::Signature {

                let signatures: [signature::Signature;#input_member_count] = [
                    #( signature::Signable::sign( &&self.#input_members ) ),*
                ];
                signature::Signable::sign( &signatures[0..#input_member_count] )
            }
        }

        impl signature::Signable for &#input_struct {

            fn sign(&self) -> signature::Signature {

                let signatures: [signature::Signature;#input_member_count] = [
                    #( signature::Signable::sign( &self.#input_members ) ),*
                ];
                signature::Signable::sign( &signatures[0..#input_member_count] )
            }
        }
    );
    signable_trait.into()
}
