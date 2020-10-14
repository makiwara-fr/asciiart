use std::path::Path;
extern crate image;
use open_image::image::GenericImageView;
use open_image::image::imageops::{Nearest};

/*
 
 Open an image / resize it for processing and printit out on standard output

*/

pub fn open_image(path : &Path) {
	
	println!("Acquiring data source");
	println!("=====================");
	println!("Opening file name : {:?}", path.to_str());
	
	//open image
	let img = image::open(path).unwrap();
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("Color type {:?}", img.color());
	
	
	
	//export dimensions
	const MAX_X : u32 = 210;
	const MAX_Y :u32 = 100;
	const MIN_X : u32 = 60;
	const MIN_Y :u32 = 40;

	//reduce size
	let modulo_x = 5;
	let modulo_y = 10;
	let mut x_target : u32 = img.dimensions().0 / modulo_x;
	let mut y_target : u32 = img.dimensions().1 / modulo_y;
	//check if size is withing boundaries
	x_target = if x_target > MAX_X { MAX_X } else { if x_target < MIN_X {MIN_X} else {x_target}};
	y_target = if y_target > MAX_Y { MAX_Y } else { if y_target < MIN_Y {MIN_Y} else {y_target}};
	
	// create char vec to get to the point
	//let mut vec : Vec<Vec<char>> = vec![vec![' '; y_target as usize]; x_target as usize];
	
	
	//turn image to black and white and resize with triangle calculation
	let grey = img.grayscale().resize_exact(x_target, y_target, Nearest); //Triangle
 	println!("Supposed {:?},{:?} Actuals {:?}", x_target, y_target, grey.dimensions());
	
	
	//list of chararcters to encode our resize image
	let encoder = " `.,-:^+/=%$@";
	// calculate the dividers to create buckets to map greyness to encoder
	let div = 256/(encoder.len());
	println!("div={} encoder={}", div, encoder.len());
	
	
	let mut res ;
	
	for y in 1..y_target {
		res =  String::new();
		for x in 1..x_target {
			//
			
			
			//vec[(x-1) as usize][(y-1) as usize] = grey_to_ascii(grey.get_pixel(x-1, y-1)[0], div, encoder);
			res.push(grey_to_ascii(grey.get_pixel(x-1, y-1)[0], div, encoder));
			//println!("y={} x={} grey={}", y, x, grey.get_pixel(x-1, y-1)[0]);
		}
		println!("{}", res);
		//y=51 x=81 grey=94

    }
	
	
	println!("");
	
}


fn grey_to_ascii(grey_value :u8, div: usize, encoder: &str) -> char{
	
	let grey = grey_value as usize;
	let len = encoder.len();
	let bucket = grey / div;
	if bucket >= len {
		encoder.chars().nth( len -1 ).unwrap()
	}
	else {
		encoder.chars().nth( bucket).unwrap()
	}
	
}	
