fn main() {
	let mut greeting = String::from("Hello, ");
	println!("{}",greeting);

	greeting.push('w');
	greeting.push_str("orld!");
	println!("{}", greeting);
}