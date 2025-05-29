use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

#[proc_macro_attribute]
pub fn entrypoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &mut input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;
    let other_attrs = &input.attrs; // Keep all attributes to re-apply
                                    // Collect arguments marked with #[db]
    let mut db_args = Vec::new();
    let mut db_args_with_types = Vec::new();
    let mut all_args_names = Vec::new();
    let mut all_args_names_only_names = Vec::new();
    let mut fn_inputs_original = fn_inputs.clone();

    fn_inputs.iter_mut().for_each(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            let len_before = pat_type.attrs.len();
            pat_type.attrs = pat_type
                .attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("key"))
                .cloned()
                .collect();

            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let is_record = len_before != pat_type.attrs.len();
                all_args_names.push((is_record, pat_type.pat.clone()));
                all_args_names_only_names.push(pat_ident.ident.clone());
                if is_record {
                    let arg_name = pat_ident.ident.clone();
                    let arg_type = pat_type.ty.clone();
                    db_args.push(arg_name.clone());
                    db_args_with_types.push((arg_name, arg_type.clone()));
                    pat_type.ty =
                        syn::parse_quote!(<#arg_type as apresh_store::DatabaseKeyable>::Key);
                }
            }
        }
    });

    fn_inputs_original.iter_mut().for_each(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            let len_before = pat_type.attrs.len();
            pat_type.attrs = pat_type
                .attrs
                .iter()
                .filter(|attr| !attr.path().is_ident("key"))
                .cloned()
                .collect();

            if len_before != pat_type.attrs.len() {
                // TODO: Don't change the type of the arguments here
                let t = pat_type.ty.clone();
                pat_type.ty = syn::parse_quote!(&mut #t);
            }
        }
    });

    let inner_fn_name = syn::Ident::new(&format!("inner_{}", fn_name), fn_name.span());

    let record_references_args = all_args_names
        .into_iter()
        .map(|(is_record, arg)| {
            if is_record {
                quote!(&mut #arg)
            } else {
                quote!(#arg)
            }
        })
        .collect::<Vec<_>>();

    let encode_fn_name = syn::Ident::new(&format!("encode_{}_args", fn_name), fn_name.span());
    let encode_args_expanded = quote! {
        pub fn #encode_fn_name(#fn_inputs) -> Vec<u8> {
            use candid::Encode;
            Encode!(&#(&#all_args_names_only_names),*).unwrap()
        }
    };

    // Generate individual get statements with custom error messages for each db_arg
    let db_get_statements = db_args_with_types
        .iter()
        .map(|(arg_name, arg_type)| {
            quote! {
                let mut #arg_name = match #arg_name.get() {
                    Some(value) => value,
                    None => {
                        let err_message = format!("`{}` `{}` with key {:?} does not exist",
                                stringify!(#arg_type),
                                stringify!(#arg_name),
                                #arg_name);

                        return Err(err_message.into());
                    }
                };
            }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #( #other_attrs )*
        fn #fn_name(#fn_inputs) #fn_output {
            fn #inner_fn_name(#fn_inputs_original) #fn_output {
                #fn_block
            }

            #(#db_get_statements)*
            let r = #inner_fn_name(#(#record_references_args),*);
            match &r {
                Ok(_) => {
                    #(#db_args.commit();)*
                },
                Err(_) => {
                    #(#db_args.revert();)*
                }
            }
            r
        }

        #encode_args_expanded
    };

    TokenStream::from(expanded)
}
