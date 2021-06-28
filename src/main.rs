
// use std::array;
use std::time::{Duration, Instant};
use plotters::prelude::*;

// use std::time::Instant;

use core::f64;

use rand::Rng;

fn main() {
    
    let digits_of_interest: usize = 5;

    let pi_data: (usize, usize) = start(digits_of_interest);

}

struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl  Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n /self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }
    }
    
}

fn start(input_digits: usize) -> (usize, usize) {
        // let pi_target: f64 = 3.141592653589793;

        let pi_abs: usize = 3141592653589793;

        let piabs_digits: Vec<_> = Digits::new(pi_abs).collect();
    
        let digitsofint: usize = input_digits;
        let mut pi_vect = Vec::new();
    
        let mut zeros: usize = 1;
        for index in 0..=digitsofint {
            pi_vect.push(piabs_digits[index]);
            zeros *= 10;
        }
    
        let decimal = zeros;
        let mut pi_tar: usize = 0;
        for index in 0..pi_vect.len() {
            pi_tar = pi_tar + (pi_vect[index] * zeros);
            zeros /= 10;
        }
    
        // let pi_tar: usize = pi_vect.iter().sum();
        println!("{}", pi_tar);
        println!("{}", pi_tar / (decimal / 100));
        
        return (pi_tar, decimal)
}

// fn iterator(max_it: u32, di_target: f64, pi_target: f64) {
    
//     let mut steps: Vec<i64> = Vec::new();
//     let mut pi_cal: Vec<f64> = Vec::new();
//     let mut pi_pd: Vec<f64> = Vec::new();
//     let mut time_taken: Vec<Duration> = Vec::new();

//     for i in 0..=max_it {
//         let startwatch: Instant = Instant::now();

//         println!("{}", i);

//         let r:(i64, f64, f64) = looper(di_target, pi_target);

//         let stopwatch: Duration = startwatch.elapsed();
        
//         steps.push(r.0);
//         pi_cal.push(r.1);
//         pi_pd.push(r.2);
//         time_taken.push(stopwatch);

//         println!("After {0} steps pi has been calculated to {1} with a % diff = {2}", r.0, r.1, r.2);
//         println!("Took {:?} seconds to finish", stopwatch)

//     }
// }

// fn point() -> Vec<f64> {

//     let mut vector_return: Vec<f64> = Vec::with_capacity(4);

//     let x: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
//     let y: f64 = rand::thread_rng().gen_range(-0.5..0.500000000000001);
//     let upper = |x: f64, y: f64| -> f64{(x.powf(2.0) + y.powf(2.0)).sqrt()};

//     vector_return.push(x);
//     vector_return.push(y);
//     vector_return.push(upper(x, y));

//     return vector_return

// }

// fn looper(dt: f64, pt: f64) -> (i64, f64, f64) {

//     let diff_target: f64 = dt;
//     let pi_target: f64 = pt;

//     let mut steps: i64 = 0;

//     let new_pi = |i: f64, o: f64| -> f64 {4.0 * (i / o)};

//     let pd = |m: f64, t: f64| -> f64 {((m - t).abs() / (0.5 * (m + t))) * 100.0};

//     let diff = |m: f64, t: f64| -> f64 {(m - t).abs()};

//     let mut hits: f64 = 0.0;
//     let mut total: f64 = 0.0;

//     loop {

//         let v: Vec<f64> = point();
        
//         if v[2] <= 0.5 {
//             hits += 1.0;
//         }

//         total += 1.0;
        
//         // if diff(new_pi(hits, total), pi_target) < diff_target {
//         //     break;
//         // } else {
//         //     steps += 1
//         // }

//         steps += 1;

//         if steps > 100 {
//             // println!("Hit too many steps {0}, pi={1}", steps, new_pi(hits, total));
//             break;
//         }
//     }

//     let r: (i64, f64, f64) = (steps, new_pi(hits, total), pd(new_pi(hits, total), pi_target));

//     return r

// }