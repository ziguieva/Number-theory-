mod nombres_premiers;
mod pgcd;
mod bezout;
mod congruence;
mod chinois;
mod exponentiation_modulaire;
mod ordre;
mod nombres_perfects;

use std::io;

fn main() {
    println!("Number Theory Menu:");
    println!("1. Check if a number is prime");
    println!("2. Find all primes up to N (Sieve of Eratosthenes)");
    println!("3. Calculate GCD (Euclidean Algorithm)");
    println!("4. Bézout Coefficients and Modular Inverse");
    println!("5. Solve Linear Congruence");
    println!("6. Chinese Remainder Theorem");
    println!("7. Modular Exponentiation");
    println!("8. Calculate the order of an element in a group");
    println!("9. Check if a number is perfect");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            let num = get_input("Enter the number: ");
            println!("Is {} prime? {}", num, nombres_premiers::is_prime(num));
        }
        "2" => {
            let n = get_input("Find primes up to: ");
            println!("Primes up to {}: {:?}", n, nombres_premiers::sieve_of_eratosthenes(n));
        }
        "3" => {
            let a = get_input("Enter first number: ");
            let b = get_input("Enter second number: ");
            println!("GCD of {} and {}: {}", a, b, pgcd::gcd(a, b));
        }
        "4" => {
            let a = get_input("Enter first number: ");
            let b = get_input("Enter second number: ");
            let (gcd, x, y) = bezout::bezout_coefficients(a, b);
            println!("Bézout coefficients for {} and {}: GCD={}, x={}, y={}", a, b, gcd, x, y);
            if let Some(inverse) = bezout::modular_inverse(a, b) {
                println!("Modular inverse of {} mod {}: {}", a, b, inverse);
            } else {
                println!("No modular inverse exists.");
            }
        }
        "5" => {
            let a = get_input("Enter the coefficient of x (a): ");
            let b = get_input("Enter the constant (b): ");
            let m = get_input("Enter the modulus (m): ");
            if let Some(solution) = congruence::solve_congruence(a, b, m) {
                println!("Solution to the congruence: x ≡ {} (mod {})", solution, m);
            } else {
                println!("No solution exists.");
            }
        }
        "6" => {
            let num_eq = get_input("Enter the number of equations: ");
            let mut congruences = Vec::new();
            let mut moduli = Vec::new();
            for i in 0..num_eq {
                let b = get_input(&format!("Enter constant b for equation {}: ", i + 1));
                let m = get_input(&format!("Enter modulus m for equation {}: ", i + 1));
                congruences.push(b);
                moduli.push(m);
            }
            println!("Solution to the system of congruences: {}", chinois::chinese_remainder_theorem(&congruences, &moduli));
        }
        "7" => {
            let base = get_input("Enter the base: ");
            let exponent = get_input("Enter the exponent: ");
            let modulus = get_input("Enter the modulus: ");
            println!("Modular exponentiation result: {}", exponentiation_modulaire::mod_exp(base, exponent, modulus));
        }
        "8" => {
            let a = get_input("Enter the element: ");
            let m = get_input("Enter the modulus (group order): ");
            println!("Order of {} in group mod {}: {}", a, m, ordre::order_in_group(a, m));
        }
        "9" => {
            let num = get_input("Enter the number: ");
            println!("Is {} a perfect number? {}", num, nombres_perfects::is_perfect_number(num));
        }
        _ => println!("Invalid option!"),
    }
}

fn get_input(prompt: &str) -> i64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please type a valid number!")
}
