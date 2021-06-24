// use std::array;

use rand::Rng;

fn main() {
    let pi_target: f64 = 3.141592653589793;
    // let pi_diff = pi_target*0.0001;
    // let pi_max = pi_target + pi_diff;
    // let pi_min = pi_target - pi_diff;
    let mut hits: f64 = 0.0;
    let mut total: f64 = 0.0;
    let mut new_pi = 0.0;
    let mut pd = 0.0;
    // let steps = 1000;
    let mut steps = 0;
    println!("We want to find Pi = {0}", pi_target);
    println!("How many steps does this require?");
    let threshold = 0.001;
    // for _ in 1..=steps {
    //     let v = point();
    //     if v[2] <= 0.5 {
    //         hits += 1.0;
    //         total += 1.0;
    //     } else {
    //         total += 1.0;
    //     }
    //     new_pi = 4.0 * hits/total;
    //     pd = percent_diff(new_pi, pi_target);
    // }
    loop {
        let v = point();
        
        if v[2] <= 0.5 {
            hits += 1.0;
            total += 1.0;
        } else {
            total += 1.0;
        }
        
        new_pi = 4.0 * hits/total;
        pd = percent_diff(new_pi, pi_target);

        if pd < threshold {
            break;
        } else {
            steps += 1
        }
    }

    println!("After {0} steps the value of pi is {1} with % diff of {2}", steps, new_pi, pd);
}

fn point() -> Vec<f64> {
    let mut vector_return = Vec::with_capacity(4);

    let x = rand::thread_rng().gen_range(-0.5..0.500000000000001);
    let y = rand::thread_rng().gen_range(-0.5..0.500000000000001);
    let upper = |x: f64, y: f64| -> f64{(x.powf(2.0) + y.powf(2.0)).sqrt()};

    vector_return.push(x);
    vector_return.push(y);
    vector_return.push(upper(x, y));

    return vector_return

}

fn percent_diff(m: f64, t: f64) -> f64 {
   let ave = 0.5*(m + t);
   let diff = (m - t).abs();
   let pd = diff / ave;
   return pd
}