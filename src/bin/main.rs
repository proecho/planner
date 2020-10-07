use std::io;
use planner::Terminator;
use planner::process;

fn main() {
	let mut running = true;
	while running == true{
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
			Ok(n) => {
				let mut return_value = process(user_input,"Save_File.txt".to_string());
				match return_value {
					Ok(Terminator::Terminate) => {
						running=false
					},
					Err(_) => {
						println!("there has been an error in processing your request");
					},
					Ok(Terminator::No) => {},
				}
			},
			
			Err(_) => {
				println!("there has been an error");
			},
		}
	}
}
				
				
