fn is_palindrome(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

fn main() {
    let input1 = "racecar";
    let input2 = "hello";
    let input3 = "121";
    println!("Is '{}' a palindrome? {}", input1, is_palindrome(input1));
    println!("Is '{}' a palindrome? {}", input2, is_palindrome(input2));
    println!("Is '{}' a palindrome? {}", input3, is_palindrome(input3));
}