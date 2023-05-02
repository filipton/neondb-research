fn main() {
    println!("Enter char count: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let count: u32 = input.trim().parse().unwrap();
    let output: String = (0..count).map(|_| '=').collect();

    std::fs::write("chars.txt", output).unwrap();
    println!("Done! Saved to chars.txt");
}
