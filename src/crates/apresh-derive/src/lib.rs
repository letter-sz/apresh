use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

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
            fn id(&self) -> ActorId {
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

#[proc_macro_derive(DeriveKey, attributes(table))]
pub fn derive_key(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    // Look for the table attribute to get the record type
    let table_value = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .and_then(|attr| attr.parse_args::<syn::LitInt>().ok())
        .map(|lit| lit.base10_parse::<u8>().unwrap_or(1))
        .unwrap_or(1);

    // Ensure it's a struct
    let fields = match input.data {
        Data::Struct(ref data_struct) => &data_struct.fields,
        _ => {
            return Error::new_spanned(name, "DeriveKey can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Get the first field
    let (first_field, first_field_type) = match fields {
        Fields::Named(fields) => {
            if let Some(field) = fields.named.iter().next() {
                if let Some(ident) = &field.ident {
                    (ident, field.ty.clone())
                } else {
                    return Error::new_spanned(
                        name,
                        "Couldn't find field identifier for first field",
                    )
                    .to_compile_error()
                    .into();
                }
            } else {
                return Error::new_spanned(
                    name,
                    "Struct must have at least one field to derive DeriveKey",
                )
                .to_compile_error()
                .into();
            }
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.iter().next().is_some() {
                return Error::new_spanned(
                    name,
                    "DeriveKey doesn't support tuple structs, use named fields",
                )
                .to_compile_error()
                .into();
            } else {
                return Error::new_spanned(
                    name,
                    "Struct must have at least one field to derive DeriveKey",
                )
                .to_compile_error()
                .into();
            }
        }
        Fields::Unit => {
            return Error::new_spanned(
                name,
                "Unit structs cannot derive DeriveKey as they have no fields",
            )
            .to_compile_error()
            .into();
        }
    };

    // Split generics for use in impl
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let key_type = syn::Ident::new(&format!("{}Key", name), name.span());

    // Generate the key() method implementation
    let key_impl = quote! {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct #key_type(pub #first_field_type);

        impl #key_type {
            pub fn get(self) -> Option<#name #ty_generics #where_clause> {
                <#name as store::Record>::get(self)
            }
        }

        impl #impl_generics store::Record for #name #ty_generics #where_clause {
            const SCOPE: u8 = #table_value;
            type Key = #key_type;

            fn key(&self) -> Self::Key {
                #key_type(self.#first_field.clone())
            }
        }
    };

    // Return the generated impl
    TokenStream::from(key_impl)
}
