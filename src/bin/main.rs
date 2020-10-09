use std::io;
use objects::Terminator;
use planner::process;


fn main() {
	let mut running = true;
	while running == true{
        let useful_input: Vec<&str>;
        let mut input = String::new();
		io::stdin().read_line(&mut input); 
	    useful_input = input.split_whitespace().collect();
		let return_value = process(useful_input,"Save_File.txt".to_string());
		match return_value {
			Ok(Terminator::Terminate) => {
				running=false
			},
			Err(_) => {
				println!("there has been an error in processing your request");
			},
			Ok(Terminator::No) => {},
		}
		
	   	
	}
}
				
				
