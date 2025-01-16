use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

#[proc_macro_derive(IsActor)]
pub fn derive_actor(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    // Ensure it's a struct
    let fields = match input.data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(name, "IsActor can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Check for a field named `base`
    let has_base = fields
        .iter()
        .any(|f| f.ident.as_ref().map(|i| i == "base").unwrap_or(false));
    if !has_base {
        return Error::new_spanned(
            name,
            "Struct must have a field named `base` to derive IsActor",
        )
        .to_compile_error()
        .into();
    }

    // Split generics for use in impl
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the Actor trait implementation
    let trait_impl = quote! {
        impl #impl_generics Actor for #name #ty_generics #where_clause {
            fn id(&self) -> Principal {
                self.base.id()
            }

            fn name(&self) -> &str {
                self.base.name()
            }

            fn role(&self) -> ActorRole {
                ActorRole::#name
            }

            fn get_active_shipments(&self) -> &[ShipmentId] {
                self.base.get_active_shipments()
            }

            fn get_shipments_history(&self) -> &[ShipmentId] {
                self.base.get_shipments_history()
            }

            fn add_shipment(&mut self, shipment_id: ShipmentId) {
                self.base.add_shipment(shipment_id)
            }

            fn archive_shipment(&mut self, shipment_id: ShipmentId) {
                self.base.archive_shipment(shipment_id)
            }
        }
    };

    // Return the generated impl
    TokenStream::from(trait_impl)
}
