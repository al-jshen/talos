use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, parse_quote, Stmt};

const DISTRIBUTIONS: [&str; 13] = [
    "normal!",
    "exponential!",
    "gamma!",
    "uniform!",
    "laplace!",
    "beta!",
    "bernoulli!",
    "binomial!",
    "poisson!",
    "cauchy!",
    "lognormal",
    "rayleigh",
    "pareto!",
];

#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> TokenStream {
    let macro_args = parse_macro_input!(args as syn::AttributeArgs);

    assert!(macro_args.len() == 1, "Macro must only have one argument.");

    let macro_arg = match &macro_args[0] {
        syn::NestedMeta::Meta(_) => panic!("Put quotation marks around the argument."),
        syn::NestedMeta::Lit(lit) => match lit {
            syn::Lit::Str(litstr) => litstr.value(),
            _ => panic!("Argument must be a string with quotation marks."),
        },
    };

    assert!(
        (macro_arg.starts_with("Var")) || (macro_arg == "f64"),
        "Macro argument must be Var or f64."
    );

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

    let add_target: Stmt = match genericargs.len() {
        0 => {
            assert!(
                macro_arg == "f64",
                "If there are no lifetimes then the macro argument must be f64."
            );
            for fnarg in input.sig.inputs.iter() {
                match fnarg {
                    syn::FnArg::Typed(pattype) => {
                        let ty = pattype.ty.to_token_stream().to_string();
                        if ty == "& [f64]" {
                            param_arg_name = Some(&pattype.pat);
                            break;
                        }
                    }
                    syn::FnArg::Receiver(_) => {
                        panic!("Function must not take self as an argument.")
                    }
                }
            }
            if param_arg_name.is_none() {
                panic!("At least one argument must take a slice of f64");
            }

            input.sig.output =
                syn::parse_str("-> f64").expect("Output type could not be rewritten to f64");

            parse_quote! {
                let mut target: f64 = 0.;
            }
        }
        1 => {
            assert!(
                macro_arg.starts_with("Var"),
                "If there is one lifetime parameter {} then the macro argument must be Var<{}>.",
                genericargs[0],
                genericargs[0]
            );

            for fnarg in input.sig.inputs.iter() {
                match fnarg {
                    syn::FnArg::Typed(pattype) => {
                        let ty = pattype.ty.to_token_stream().to_string();
                        if ty == format!("& [Var < {} >]", genericargs[0]) {
                            param_arg_name = Some(&pattype.pat);
                            break;
                        }
                    }
                    syn::FnArg::Receiver(_) => {
                        panic!("Function must not take self as an argument.")
                    }
                }
            }
            if param_arg_name.is_none() {
                panic!(
                    "At least one argument must take a slice of Var<{}>",
                    genericargs[0]
                );
            }

            input.sig.output =
                syn::parse_str(&format!("-> Var < {} >", genericargs[0])).expect(&format!(
                    "Output type could not be rewritten to Var<{}>!",
                    genericargs[0]
                ));

            parse_quote! {
                let mut target = #param_arg_name[0].tape.add_var(0.);
            }
        }
        _ => panic!("Wrong number of lifetimes!"),
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
                    // println!("{:?}", ss);
                    ss = ss.replace(dist, &format!("target = target + {}", dist));
                }
            }
            syn::parse_str(&ss).expect(&format!("failed to convert statement: {}", ss))
        })
        .collect::<Vec<_>>();

    input.into_token_stream().into()
}
