// use std::array;

use core::f64;

use rand::Rng;

fn main() {

    // let attempts: i32 = 10;
    let attempts: i32 = 5;

    // let mut steps: Vec<i32> = Vec::new();
    // let mut pi_coll: Vec<f64> = Vec::new();
    // let mut pi_pd: Vec<f64> = Vec::new();

    // let pi_target: f64 = 3.141592653589793;
    // let di_target: f64 = 0.0000000000000001;

    let pi_target: f64 = 3.141592;
    let di_target: f64 = 0.0000009;

    for i in 0..=attempts {

        println!("{}", i);

        let r:(i32, f64, f64) = looper(di_target, pi_target);
        
        // steps.push(r.0);
        // pi_coll.push(r.1);
        // pi_pd.push(r.2);

        println!("After {0} steps pi has been calculated to {1} with a % diff = {2}", r.0, r.1, r.2);
    
    }

    // println!("After {0} attemps we have steps = {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10},", attempts, steps[0], steps[1], steps[2], steps[3], steps[4], steps[5], steps[6], steps[7], steps[8], steps[9])
    // println!("After {0} steps pi has been calculated to {1} with a % diff = {2}", steps[0], pi_coll[0], pi_pd[0])

}

fn point() -> Vec<f64> {

    let mut vector_return: Vec<f64> = Vec::with_capacity(4);

    let x: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
    let y: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
    let upper = |x: f64, y: f64| -> f64{(x.powf(2.0) + y.powf(2.0)).sqrt()};

    vector_return.push(x);
    vector_return.push(y);
    vector_return.push(upper(x, y));

    return vector_return

}

fn looper(dt: f64, pt: f64) -> (i32, f64, f64) {

    let diff_target: f64 = dt;
    let pi_target: f64 = pt;

    let mut steps: i32 = 0;

    let new_pi = |i: f64, o: f64| -> f64 {4.0 * (i / o)};

    let pd = |m: f64, t: f64| -> f64 {((m - t).abs() / (0.5 * (m + t))) * 100.0};

    let diff = |m: f64, t: f64| -> f64 {(m - t).abs()};

    let mut hits: f64 = 0.0;
    let mut total: f64 = 0.0;

    loop {

        let v: Vec<f64> = point();
        
        if v[2] <= 0.5 {
            hits += 1.0;
        }

        total += 1.0;
        
        if diff(new_pi(hits, total), pi_target) < diff_target {
            break;
        } else {
            steps += 1
        }

        if steps > 1000000000 {
            println!("Hit too many steps {0}, pi={1}", steps, new_pi(hits, total));
            break;
        }
    }

    let r: (i32, f64, f64) = (steps, new_pi(hits, total), pd(new_pi(hits, total), pi_target));

    return r

}