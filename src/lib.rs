use std::fs::remove_file;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

#[derive(Debug,PartialEq)]
pub enum Terminator{
	Terminate,
	No,
}

pub enum entrys{
	Todo,
	Events,
	appointments,
}

struct Todo{
	Title: Option(&str),
	List: Option(&str),
	DateTime: Option(DateTime),
}

struct Events;

struct appointments;

trait entry_type {
	fn save(&self) -> Result<fn,String> {}
	
}

impl Todo {
	fn new(Title: Option(&str), List: Option(&str), DateTime:Option(DateTime)) -> Todo{
		Todo{
			Title,
			List,
			DateTime,
		}
	}
	
	
	

pub fn process(input:String,file:String) -> Result<Terminator,String> {
	let output = input_parser(input);
	Ok(Terminator::No)
}	
	

fn input_parser(input:String) -> Result<entrys,String> {
	let first_step: Vec<&str> = input.split_whitespace().collect();
	match first_step[0] {
		"To_do" =>{
			return Ok(entrys::Todo)
		}
		"Event" =>{
			Err("not implemented".to_string())
		}
		"appointment" =>{
			Err("not implemented".to_string())
		}
	    _ =>{
			 Err("invalid input".to_string())
		}
	}
}	 



#[cfg(test)]
mod tests {
	use super::*;
	#[test]	
	fn ignores_nonsense() {
		remove_file("test_file.txt");
		let i = "bualihbviabliue";
	    assert_eq!(process(i.to_string(),"test_file.txt".to_string()),Err("invalid input".to_string()));
	    
	}
	#[test]
	fn doesnt_crash_on_long(){
		remove_file("test_file.txt");
		let words = String::from_utf8(vec![b'P',254]).unwrap();
		assert_eq!(process(words,"test_file.txt".to_string()), Err("invalid input".to_string()));
		
	}
	#[test]
	fn returns_for_Todo() {
		remove_file("test_file.txt");
		let words = "To_do".to_string();
		assert_eq!(process(words,"test_file.txt".to_string()),Ok(Terminator::No));
	
	}
		
	
	
			
}
