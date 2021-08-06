use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, Stmt};

const DISTRIBUTIONS: [&str; 6] = [
    "normal!",
    "exponential!",
    "gamma!",
    "uniform!",
    "laplace!",
    "beta!",
];

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let mut param_arg_name = None;

    let generics = if input.sig.generics.lt_token.is_some() && input.sig.generics.gt_token.is_some()
    {
        input.sig.generics.params.to_token_stream().to_string()
    } else {
        "".to_string()
    };

    let genericargs = generics
        .split(", ")
        .into_iter()
        .filter_map(|g| if g.starts_with("'") { Some(g) } else { None })
        .collect::<Vec<_>>();

    assert!(genericargs.len() == 1, "Must have one lifetime parameter!");

    for fnarg in input.sig.inputs.iter() {
        match fnarg {
            syn::FnArg::Typed(pattype) => {
                let ty = pattype.ty.to_token_stream().to_string();
                if ty == format!("& [Var < {} >]", genericargs[0]) {
                    param_arg_name = Some(&pattype.pat);
                    break;
                }
            }
            syn::FnArg::Receiver(_) => panic!("Function must not take self as an argument."),
        }
    }
    if param_arg_name.is_none() {
        panic!(
            "At least one argument must take a slice of Var<{}>",
            genericargs[0]
        );
    }

    input.sig.output = syn::parse_str(&format!("-> Var < {} >", genericargs[0]))
        .expect("Output type could not be rewritten to Var<'a>!");

    let add_target: Stmt = parse_quote! {
        let mut target = #param_arg_name[0].tape.add_var(0.);
    };
    let return_target: Stmt = parse_quote! {
        return target;
    };

    input.block.stmts.insert(0, add_target);
    input.block.stmts.push(return_target);

    input.block.stmts = (input.block.stmts)
        .iter()
        .map(|s| {
            let mut ss = s.to_token_stream().to_string();
            for dist in DISTRIBUTIONS {
                if ss.contains(dist) {
                    println!("{:?}", ss);
                    ss = ss.replace(dist, &format!("target = target + {}", dist));
                }
            }
            syn::parse_str(&ss).expect(&format!("failed to convert statement: {}", ss))
        })
        .collect::<Vec<_>>();

    for l in &input.block.stmts {
        println!("{}", l.to_token_stream().to_string());
    }

    input.into_token_stream().into()
}
