use std::env;
use std::path::Path;
mod open_image;

fn main() {
    
	println!("Analysing parameters");
	println!("====================");
	
	//general parameters to launch the right processing
	let mut video_mode: bool = false;

	
	
	//parsing the arguments
	//=====================
	let args : Vec<String> = env::args().collect();
	let mut files_path: Vec<&str> = Vec::new();


	// launching processing
	// ====================
	for argument in &args[1..args.len()] {
		match argument.as_str() {
			"-v" => {
				println!("video mode"); 
				video_mode = true;
				//launched=True;
			},
			x => files_path.push(x),
		}
		
	}

	/*if !launched {
		// use a default file
		println!("No parameters given");
		println!("Will use default one");
		open_image::open_image(Path::new("fuck-movies1.jpg"))
	}*/
		
	if !video_mode {
		println!("Image only\n");
		if files_path.len() > 1 {
			for file in files_path {
				open_image::open_image(Path::new(file));
			}
		}
		else {
			open_image::open_image(Path::new("fuck-movies1.jpg"));
		}
		
	}
	else {
		println!("Video mode still underdevelopment");
		println!("Stopping");
	}
	
	
	
	
	
}
