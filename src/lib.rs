use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,};
use std::boxed::Box;
use objects::*;
use regex::Regex;
use std::str::FromStr;
	
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

//puts informtion in each objects and internal feild and creates objects
fn Event_parser< T: entry_type>(input:Vec<&str>,heading_list:Vec<String>) -> T {
	let mut output:Vec<Option<String>> = Vec::new();
	
	for heading in heading_list{
	    let mut interim_output: Vec<String> = Vec::new();
		
		for value in input.clone() {
			let mut catch = false;
		    if value == heading{
				catch = true;				
			}else if value == "\n"{
				catch=false;
			}
			if catch == true{
			    interim_output.push(value.to_string());
			}
		}
		if interim_output == (Vec::<String>::new()){
			output.push(None)
		}else{
		    output.push(Some(unifier(interim_output)));
	    }
			
	    
	}
    let Title = output[0].clone();
    let DateTime = output[1].clone();
    let List = match output[2].clone(){
		Some(a) => Box::new(Some(DateTime::<Utc>::from_str(&a).unwrap())),
		None => Box::new(None),
	};
    let Other = output[3].clone();
			
	
	return T::new(
	    Title,
	    List,
	    DateTime,
	    Other,
	);
}

//converts vectors of strings into single string
fn unifier(vector:Vec<String>)->String{
	let mut output = String::new(); 
	for a in vector {
		output = format!("{} {}", output, a);
	}
	
	output
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
