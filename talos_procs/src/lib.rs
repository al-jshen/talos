use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, Stmt};

const DISTRIBUTIONS: [&str; 5] = ["normal!", "exponential!", "gamma!", "uniform!", "laplace!"];

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let mut param_arg_name = None;

    for fnarg in input.sig.inputs.iter() {
        match fnarg {
            syn::FnArg::Typed(pattype) => {
                let ty = pattype.ty.to_token_stream().to_string();
                if ty == "& [Var < 'a >]" {
                    param_arg_name = Some(pattype.pat.clone());
                    break;
                }
            }
            syn::FnArg::Receiver(_) => panic!("Function must not take self as an argument."),
        }
    }
    if param_arg_name.is_none() {
        panic!("At least one argument must take a slice of Var<'a>");
    }

    // let add_target = format!(
    //     r#"
    //     assert!({}.len() > 0, "Parameter slice must not be empty.");
    //     let mut target = {}.tape.add_var(0.);
    // "#,
    //     param_arg_name, param_arg_name
    // )
    // .parse::<TokenStream>()
    // .unwrap();

    let add_target: Stmt = parse_quote! {
        let mut target = #param_arg_name[0].tape.add_var(0.);
    };
    let return_target: Stmt = parse_quote! {
        return target;
    };

    input.block.stmts.insert(0, add_target);
    input.block.stmts.push(return_target);

    let mut to_add: Vec<(usize, String)> = vec![];

    for (i, stmt) in input.block.stmts.iter().enumerate() {
        let ss = stmt.to_token_stream().to_string();
        for dist in DISTRIBUTIONS {
            if ss.contains(dist) {
                if ss.starts_with(dist) {
                    // simple case
                    to_add.push((i, format!("target = target + {}", ss)));
                } else {
                    // somewhere in the middle of the statement
                }
            }
        }
    }

    for (i, stmt) in to_add.iter().rev() {
        input.block.stmts[*i] = syn::parse_str(&stmt).unwrap();
    }

    for l in &input.block.stmts {
        println!("{}", l.to_token_stream().to_string());
    }

    input.into_token_stream().into()
}
