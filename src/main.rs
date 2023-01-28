use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::Read;
use nix::sys::wait::wait;
use nix::libc::execlp;
use nix::libc::perror;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write}}; 
use nix::unistd::{getpid, getppid, execv};
use std::process::Command;   
use std::ffi::CString;
use std::os::raw::c_char;    
use std::ptr; 
use std::process::exit; 
use std::ffi::CStr;

fn main(){
	let file = File::open("input.txt").expect("Unable to open File");
	let reader = BufReader::new(file);
	let mut API: String = "https://api.open-meteo.com/v1/forecast?latitude=&longitude=&current_weather=True".to_owned(); 
	let line: String;
 
	let folder = CString::new("/usr/bin/curl").expect("Fail"); 
	let command = CString::new("curl").expect("Fail");
	let file_input = CString::new("file.json").expect("Faol"); 
	let option = CString::new("-o").expect("Fail");
	let error = CString::new("Error").expect("fail");  		

	for line in reader.lines() {
		let data: String = line.expect("Error in data"); 
		let line_split = data.split(" "); 
		let dataPoints: Vec<&str> = line_split.collect();

		match unsafe {fork()}{
			Ok(ForkResult::Child) => { 
				API = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=True", dataPoints[0], dataPoints[0]);
				let	API_f = CString::new(API).expect("Fail");
				unsafe {

					let exe_value = execlp(folder.as_ptr(), command.as_ptr(), option.as_ptr(), file_input.as_ptr(), API_f.as_ptr(), std::ptr::null::<*const libc::c_char>()); 
					if exe_value < 0 {
						unsafe{perror(error.as_ptr())}; 
						panic!("Error occured {}", exe_value);
						exit(1);
					} 
				}
			 }
 
			Ok(ForkResult::Parent {child}) => {
				wait().expect("Error");
				println!("Hello from parent {}", getpid());  

			}

			Err(_) => println!("Fork Failed"), 
		}
	} 
 }

