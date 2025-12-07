
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

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    
    // Check if num is divisible by any number from 2 to the square root of num
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false; // If num is divisible by any number other than 1 and itself, it's not prime
        }
    }
    
    true // If num is not divisible by any number other than 1 and itself, it's prime
}
