fn main() {
	for i in 0..5 
	{
    	println!("{}", i * 2);
	}

	for i in std::iter::repeat(5) 
	{
	    println!("turns out {} never stops being 5", i);
	    break; // would loop forever otherwise
	}

	'outer: for x in 5..50 {
	    for y in 0..10 {
	        if x == y {
	            break 'outer;
	        }
	    }
	}
}