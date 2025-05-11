use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat};

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
    let fn_inputs_original = fn_inputs.clone();

    fn_inputs.iter_mut().for_each(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            let len_before = pat_type.attrs.len();
            pat_type.attrs = pat_type.attrs.iter().filter(|attr| !attr.path().is_ident("key")).cloned().collect();
            // pat_type.ty = syn::parse_quote!(<#(pat_type.ty) as Record>::Key);
            let t = pat_type.ty.clone();
            pat_type.ty = syn::parse_quote!(<#t as Record>::Key);

            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                all_args_names.push(pat_type.pat.clone());
                if len_before != pat_type.attrs.len() {
                    db_args.push(pat_ident.ident.clone());
                }
            }

        }
    });

    let expanded = quote! {
        fn #fn_name(#fn_inputs) #fn_output {
            #(let #db_args = #db_args.get().unwrap();)*
            // fn #fn_name(#fn_inputs_original) #fn_output {
                #fn_block
            // }
            // #fn_name(#(#all_args_names),*)
        }
    };

    TokenStream::from(expanded)
}
