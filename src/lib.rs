
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::boxed::Box;

#[derive(Debug,PartialEq)]
pub enum Terminator{
	Terminate,
	No,
}

enum entrys<'a>{
	Todo(Todo <'a>),
	Events(Events),
	appointments(appointments),
}

struct Todo<'a>{
	Title: Option<&'a str>,
	List: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
}

struct Events;

struct appointments;

trait entry_type {
	fn save(&self) -> Result<Terminator,String> {
		Err("not implemented".to_string())
		}
	
}

impl <'a> entry_type for Todo<'a> {}

impl entry_type for Events {}

impl entry_type for appointments {}

impl<'a> Todo <'a> {
	fn new(Title: Option<&'a str>, List: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>) -> Todo<'a>{
		Todo{
			Title,
			List,
			DateTime,
		}
	}
}
	
	

pub fn process(input:Vec<&str>,file:String) -> Result<Terminator,String> {
	let processed_input = task(input,file);
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

fn task(input:Vec<&str>,file:String) -> Result<Terminator,String> {
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
	use std::fs::remove_file;
	#[test]	
	fn ignores_nonsense() {
		remove_file("test_file.txt");
		let a = "bualihbviabliue".to_string();
		let i = [&a[..]].to_vec();
	    assert_eq!(process(i,"test_file.txt".to_string()),Err("invalid input".to_string()));
	    
	}
	#[test]
	fn doesnt_crash_on_long(){
		remove_file("test_file.txt");
		let  a = String::from_utf8(vec![b'P',254]).unwrap();
		let words = [&a[..]].to_vec();
		assert_eq!(process(words,"test_file.txt".to_string()), Err("invalid input".to_string()));
		
	}
	#[test]
	fn returns_for_Todo() {
		remove_file("test_file.txt");
		let a = "To_do".to_string();
		let words = [&a[..]].to_vec();
		assert_eq!(process(words,"test_file.txt".to_string()),Ok(Terminator::No));
	
	}
		
	
	
			
}
