use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::boxed::Box;
use objects::*;
	

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
	    _ => {
			return Err("not implemented".to_string())
	   }
    }
	
}
	
	

fn input_parser<'a>(input:Vec<&str>) -> Result<entrys<'a>,String> {
	match &(*input[1]) {
		"To_do" =>{
			return Ok(entrys::Todo(Todo::new(
				None,
				None,
				Box::new(None),
			)))
		}
		"Event" =>{
			return Ok(entrys::Events(Events::new(
			    None,
			    Box::new(None),
			    None,
			    None,
			)))
		}
		"appointment" =>{
			return Ok(entrys::appointments(Appointments::new(
			    None,
			    Box::new(None),
			    None,
			    None,
			)))
		}
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
