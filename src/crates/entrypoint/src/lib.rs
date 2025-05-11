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

    // Collect arguments marked with #[db]
    let mut db_args = Vec::new();
    let mut all_args_names = Vec::new();
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
                if is_record {
                    db_args.push(pat_ident.ident.clone());
                    if let FnArg::Typed(pat_type) = arg {
                        let t = pat_type.ty.clone();
                        pat_type.ty = syn::parse_quote!(<#t as Record>::Key);
                    }
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

    let expanded = quote! {
        fn #fn_name(#fn_inputs) #fn_output {
            fn #inner_fn_name(#fn_inputs_original) #fn_output {
                #fn_block
            }

            #(let mut #db_args = #db_args.get().unwrap();)*
            #inner_fn_name(#(#record_references_args),*)
        }
    };

    TokenStream::from(expanded)
}
