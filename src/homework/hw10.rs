use std::io;

fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let mut input = String::new();
    println!("Введіть число:");
    io::stdin().read_line(&mut input).unwrap();

    let n: u32 = input.trim().parse().expect("Введіть коректне число!");

    if is_palindrome(n) {
        println!("{n} — це паліндром.");
    } else {
        println!("{n} — не паліндром.");
    }
}