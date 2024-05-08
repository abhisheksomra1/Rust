fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

fn main() {
    let original_string = "Abhisehk Somra!";
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}
