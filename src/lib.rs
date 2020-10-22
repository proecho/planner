use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,};
use std::boxed::Box;
use objects::*;
use std::str::FromStr;
use shared_parser::Event_parser;

	
//first stage processing of user input also returns struct which shuts down program.
pub fn process(input:Vec<&str>) -> Result<Terminator,String> {
	let processed_input = task(input);
	match processed_input {
		Ok(a) => {
			match a {
				Terminator::No => return Ok(Terminator::No),
				Terminator::Terminate => return Ok(Terminator::Terminate),
			}
		}
		Err(a) => return Err("invalid input".to_string()),
	}
}

//looks at first part of input and send to various function for further processing also returns struct which shuts down program 
//after further processing new entries are returned here and are saved
fn task(input:Vec<&str>) -> Result<Terminator,String> {
	match &(*input[0]){
		"new" => {
			let new_entry = input_parser(input);
			match new_entry {
				Ok(entrys::Todo(entry)) => {
				    let saved = entry.save();
				    return saved 
				},	
				Ok(entrys::Events(entry)) =>{
					  let saved = entry.save();
					  return saved 
				}, 
				Ok(entrys::appointments(entry)) => {
					let saved =(entry.save());
					return saved 
				},
				
				_ =>{Err("Impossible".to_string())},
			}
		}
		"Terminate" => return Ok(Terminator::Terminate),
		
		
	    _ => {
			return Err("not implemented".to_string())
	   }
    }
	
}
	
	
//finishes parsing new entrys through passing to afunction that parses internal feilds and creates object returns reminder objects rapped in an enum with varients for each object
fn input_parser(input:Vec<&str>) -> Result<entrys,String> {
	match &(*input[1]) {
		"To_do" => Ok(entrys::Todo(Event_parser(input,vec!["Title".to_string(),"DateTime".to_string(),"List".to_string(),"ATTyr".to_string()]))),

		"Event" => Ok(entrys::Events(Event_parser(input,vec!["Title".to_string(),"DateTime".to_string(),"Description".to_string(),"Attendees".to_string()]))),
		
		"appointment" => Ok(entrys::appointments(Event_parser(input,vec!["Title".to_string(),"DateTime".to_string(),"With_who".to_string(),"Description".to_string()]))),
		
	    _ =>{
			 return Err("invalid input".to_string())
		}
	}
}	 

	

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::remove_file;
	#[test]	
	fn ignores_nonsense() {
		remove_file("test_file.txt");
		let a = "bualihbviabliue".to_string();
		let i = [&a[..]].to_vec();
	    assert_eq!(process(i),Err("invalid input".to_string()));
	    
	}
	#[test]
	fn doesnt_crash_on_long(){
		remove_file("test_file.txt");
		let  a = String::from_utf8(vec![b'P',100]).unwrap();
		let words = [&a[..]].to_vec();
		assert_eq!(process(words), Err("invalid input".to_string()));
		
	}
	#[test]
	fn returns_for_Todo() {
		remove_file("test_file.txt");
		let a = "To_do".to_string();
		let b = "new".to_string();
		let words = [&b[..],&a[..]].to_vec();
		assert_eq!(process(words),Ok(Terminator::No));
	
	}
	#[test]
	fn input_parser_fails_nonsense() {
		let input = ["dummy", "feadeafdeasf"].to_vec();
		assert_eq!(input_parser(input), Err("invalid input".to_string()));
	} 
		
	
	
			
}
