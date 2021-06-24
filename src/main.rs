// use std::array;

use rand::Rng;

fn main() {

    let attempts: i32 = 10;
    let mut steps: Vec<i32> = Vec::new();
    let mut pi_coll: Vec<f64> = Vec::new();

    let threshold: f64 = 0.01;

    for _ in 1..=attempts {

        let r = looper(threshold);
        steps.push(r.0);
        pi_coll.push(r.1);
    
    }

    // println!("After {0} attemps we have steps = {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10},", attempts, steps[0], steps[1], steps[2], steps[3], steps[4], steps[5], steps[6], steps[7], steps[8], steps[9])

}

fn point() -> Vec<f64> {

    let mut vector_return = Vec::with_capacity(4);

    let x: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
    let y: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
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

fn looper(t: f64) -> (i32, f64) {

    let threshold = t;
    let mut steps = 0;

    let pi_target: f64 = 3.141592653589793;
    let mut new_pi = 0.0;
    let mut pd = 0.0;

    let mut hits: f64 = 0.0;
    let mut total: f64 = 0.0;

    // println!("We want to find Pi = {0}", pi_target);
    // println!("How many steps does this require?");

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

    // println!("After {0} steps the value of pi is {1} with % diff of {2}", steps, new_pi, pd);

    let r = (steps, new_pi);

    return r

}