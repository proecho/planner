use std::io;
use objects::Terminator;
use planner::process;

//user input put in vector and passed into functions for parsing
//error handling
fn main() {
	let mut running = true;
	while running == true{
        let useful_input: Vec<&str>;
        let mut input = String::new();
        let mut a = true;
        while a == true{
            let mut input2 = String::new();
		    io::stdin().read_line(&mut input2);
		    if input2 =="".to_string(){
				a = false;
				input = format!("{}{}/n/n",input,input2);
			}else if input2 == "Title" || input2== "List" || input2== "DateTime" || input2== "Attendees" || input2== "With_who" || input2 == "Description" {
			    input = format!("/n{}{}",input,input2);
			}else{
				input = format!("{}{}",input,input2);
			}
		}

//break words into individual vectors for processing		     
	    useful_input = input.split_whitespace().collect();
		let return_value = process(useful_input);
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
				
				
