mod rounding;

mod printer {
    use rust_pilib::monte_carlo_pi;
    use crate::rounding::round;
    
    pub fn pretty_print_pi_approx(iterations: usize) {
        let pi = monte_carlo_pi(iterations);
        let places: usize = 2;

        println!("Pi is ~ {} and rounded to {} places {}", pi, places, round(pi, places));
    }
}

use printer::pretty_print_pi_approx;

fn main() {
    pretty_print_pi_approx(100_000);
}
