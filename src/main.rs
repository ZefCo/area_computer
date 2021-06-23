use std::array;

use rand::Rng;

fn main() {
    // println!("Hello, world!");
    let pi_target: f64 = 3.141592653589793;
    let pi_diff = pi_target*0.0001;
    let pi_max = pi_target + pi_diff;
    let pi_min = pi_target - pi_diff;
    let mut hits: i64 = 0;
    let mut total: i64 = 0;
    let mut prob: i64 = 0;
    let steps = 100;
    println!("We want to find Pi = {0} between {1} and {2}", pi_target, pi_max, pi_min);
    println!("How many steps does this require?");
    // random_number()
    let v = point();
    // println!("The values generated are {0}, {1}, {2}, {3}", v[0], v[1], v[2], v[3]);
    // let prob = inside_outside(total, hits, v[2], v[3]);
    // println!("The value for pi is {0} with {1} hits after {2} shots", prob, hits, total)
    for _ in 0..steps {
        if v[2] <= 1.0 && v[3] >= -1.0 {
            hits += 1;
            total += 1;
        } else {
            total += 1;
        }
        prob = hits / total;
        // println!("{}", prob)
    }
    let new_pi = 4 * prob;
    // println!("After {0} steps the value of pi is {1}", steps, new_pi)
    println!("{0}, {1}", total, hits)
}

fn random_number() -> f64 {
    let rng:f64 = rand::thread_rng().gen_range(-1.0..1.0);
    // println!("Random number gen: {}", rng)
    return rng
}

fn point() -> Vec<f64> {
    // let x = random_number();
    // let y = random_number();
    // println!("Generated two random numbers: {0} and {1}", x, y)
    let mut vector_return = Vec::with_capacity(4);

    let x = rand::thread_rng().gen_range(-1.0..1.000000000000001);
    let y = rand::thread_rng().gen_range(-1.0..1.000000000000001);
    let upper = |x: f64, y: f64| -> f64{x.powf(2.0) + y.powf(2.0)};
    let lower = |x: f64, y: f64| -> f64{-(x.powf(2.0) + y.powf(2.0))};

    vector_return.push(x);
    vector_return.push(y);
    vector_return.push(upper(x, y));
    vector_return.push(lower(x, y));

    return vector_return

    // println!("Generated points {0} & {1} and their r is {2} & {3}", x, y, upper(x, y), lower(x, y))
}

fn inside_outside(mut t: f64, mut i: f64, u: f64, l: f64) -> f64 {
    let prob: f64;
    if u <= 1.0 && l <= 1.0 {
        i += 1.0;
        t += 1.0;
    } else {
        t += 1.0
    }
    prob = i / t;
    return prob
}
