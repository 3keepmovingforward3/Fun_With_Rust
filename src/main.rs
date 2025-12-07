mod math;
pub use math::my_math::is_prime;
fn main() {
    //Let's find Mersenne Primes
    // M_n = 2^n - 1
    let max = std::u32::MAX;
    for i in 0..max{
        if is_prime(i) == true {
            if is_prime(u32::pow(2, i as u32) - 1) == true {
               println!("Mersenne Prime: {}", u32::pow(2, i as u32) - 1); 
            }
        }  
    }
}
