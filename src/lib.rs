
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::boxed::Box;

#[derive(Debug,PartialEq)]
pub enum Terminator{
	Terminate,
	No,
}

#[derive(Debug,PartialEq)]
enum entrys<'a>{
	Todo(Todo <'a>),
	Events(Events <'a>),
	appointments(Appointments <'a>),
}

#[derive(Debug,PartialEq)]
struct Todo<'a>{
	Title: Option<&'a str>,
	List: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
}

#[derive(Debug,PartialEq)]
struct Events<'a>{
	Title: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
	Description: Option<&'a str>,
	Attendees:Option<&'a str>,
}
	
#[derive(Debug,PartialEq)]
struct Appointments<'a>{
	Title: Option<&'a str>,
	DateTime: Box<Option<DateTime<Utc>>>,
	With_who: Option<&'a str>,
	Description: Option<&'a str>,
}
	

trait entry_type {
	fn save(&self) -> Result<Terminator,String> {
		Ok(Terminator::No)
		}
	
}

impl <'a> entry_type for Todo<'a> {}

impl <'a> entry_type for Events<'a> {}

impl <'a> entry_type for Appointments<'a> {}

impl<'a> Todo <'a> {
	fn new(Title: Option<&'a str>, List: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>) -> Todo<'a>{
		Todo{
			Title,
			List,
			DateTime,
		}
	}
}

impl<'a> Events<'a> {
	fn new(Title: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>, Description: Option<&'a str>, Attendees:Option<&'a str>) -> Events<'a> {
		Events{
			Title,
			DateTime,
			Description,
			Attendees,
		}
	}
}

impl<'a> Appointments<'a>{
	fn new(Title: Option<&'a str>, DateTime: Box<Option<DateTime<Utc>>>, With_who: Option<&'a str>, Description: Option<&'a str>) -> Appointments<'a> {
		Appointments{
			Title, 
	        DateTime,
	        With_who,
	        Description,
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
	    assert_eq!(process(i,"test_file.txt".to_string()),Err("invalid input".to_string()));
	    
	}
	#[test]
	fn doesnt_crash_on_long(){
		remove_file("test_file.txt");
		let  a = String::from_utf8(vec![b'P',100]).unwrap();
		let words = [&a[..]].to_vec();
		assert_eq!(process(words,"test_file.txt".to_string()), Err("invalid input".to_string()));
		
	}
	#[test]
	fn returns_for_Todo() {
		remove_file("test_file.txt");
		let a = "To_do".to_string();
		let b = "new".to_string();
		let words = [&b[..],&a[..]].to_vec();
		assert_eq!(process(words,"test_file.txt".to_string()),Ok(Terminator::No));
	
	}
	#[test]
	fn input_parser_fails_nonsense() {
		let input = ["dummy", "feadeafdeasf"].to_vec();
		assert_eq!(input_parser(input), Err("invalid input".to_string()));
	} 
		
	
	
			
}
