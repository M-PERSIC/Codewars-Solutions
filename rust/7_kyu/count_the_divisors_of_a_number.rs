/*
Michael Persico
Aug.03, 2022
Count the divisors of a number
https://www.codewars.com/kata/542c0f198e077084c0000c2e
*/

fn divisors(n: u32) -> u32 {
    (1..=n).filter(|x| n % x == 0).count() as u32
}

fn main() {
    println!("{}", divisors(1)); // 1
    println!("{}", divisors(4)); // 3
    println!("{}", divisors(5)); // 2
    println!("{}", divisors(12)); // 6
    println!("{}", divisors(25)); // 3
    println!("{}", divisors(4096)); // 13
}
