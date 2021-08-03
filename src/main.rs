use std::collections::HashMap;

use reverse::*;
use talos::*;

fn main() {
    let g = Graph::new();

    let mut params = [1., 1., 1., 0.]
        .iter()
        .map(|&x| g.add_var(x))
        .collect::<Vec<_>>();

    let mut data = HashMap::new();
    data.insert("x", vec![0., 1., 2., 3., 4.]);
    data.insert("y", vec![0.1, 2.05, 3.9, 6.2, 7.8]);

    for _ in 0..1200 {
        println!(
            "{:?}",
            [params[0].val(), params[1].val(), params[2].val().exp()]
        );
        let res = -lnlik(&params, &data);
        let grad = res.grad().wrt(&params[..3]);
        println!("{:?}", res.val());
        for i in 0..3 {
            params[i] = params[i] - 1e-3 * grad[i]
        }
        params[3] = g.add_var(0.);
    }
}

fn lnlik<'a>(params: &[Var<'a>], data: &HashMap<&str, Vec<f64>>) -> Var<'a> {
    let (m, b, s, mut l) = (params[0], params[1], params[2], params[3]);

    for (&xi, &yi) in data["x"].iter().zip(&data["y"]) {
        l = l + normal!(yi ; xi * m + b, s.exp());
    }

    l
}