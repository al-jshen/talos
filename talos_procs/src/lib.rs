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
                    param_arg_name = Some(&pattype.pat);
                    break;
                }
            }
            syn::FnArg::Receiver(_) => panic!("Function must not take self as an argument."),
        }
    }
    if param_arg_name.is_none() {
        panic!("At least one argument must take a slice of Var<'a>");
    }

    let add_target: Stmt = parse_quote! {
        let mut target = #param_arg_name[0].tape.add_var(0.);
    };
    let return_target: Stmt = parse_quote! {
        return target;
    };

    input.block.stmts.insert(0, add_target);
    input.block.stmts.push(return_target);

    // let mut to_add: Vec<(usize, String)> = vec![];

    // for (i, stmt) in input.block.stmts.iter().enumerate() {
    //     let ss = stmt.to_token_stream().to_string();
    //     for dist in DISTRIBUTIONS {
    //         if ss.contains(dist) {
    //             if ss.starts_with(dist) {
    //                 // simple case
    //                 to_add.push((i, format!("target = target + {}", ss)));
    //             } else {
    //                 // somewhere in the middle of the statement
    //                 println!("{:?}", ss);
    //             }
    //         }
    //     }
    // }

    // for (i, stmt) in to_add.iter().rev() {
    //     println!("i, {:?}", stmt.to_token_stream().to_string());
    //     input.block.stmts[*i] = syn::parse_str(&stmt).unwrap();
    // }
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
