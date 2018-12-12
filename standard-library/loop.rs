fn main() {
	let mut i = 0;
	loop {
	    println!("i is {}", i);
	    if i > 10 {
	        break;
	    }
	    i += 1;
	}
}