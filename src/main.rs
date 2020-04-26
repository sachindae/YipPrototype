#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::response::content;
use rocket::request::Form;
use rocket::response::NamedFile;
use std::fs;

use std::fs::OpenOptions;
use std::io::Write;

use std::fs::File;
use std::io::{self, BufRead};

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(FromForm)]
struct Review {
	title: String,
	review: String,
}

// Load home page for empty file path
#[get("/")]
fn home() -> Option<NamedFile> {
	// Empty file path, give home page
	NamedFile::open(Path::new("static/pages/homepage.html")).ok()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn kennel_exists(kennel: &str) -> bool {
	// Check if kennel in db
	if let Ok(lines) = read_lines("db/kennels.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(rev) = line {
            	if rev.contains(&kennel){  // Kennel exists, go to page
					return true;
    			}
            	
            }
        }
    }

    // Kennel not found in db
    return false;
}

// Load review content
#[get("/load_review?<kennel>&<id>")]
fn load_review_content(kennel: String, id: String) -> String {

	let mut reviews = "".to_owned();

	// Open file to read reviews
    if let Ok(lines) = read_lines("db/reviews.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(rev) = line {
            	if ( rev.contains(&kennel) && rev.contains(&id) ){ // Review exists
            		let tokens:Vec<&str>= rev.split("-").collect();
					reviews = reviews + tokens[1] + "," + tokens[2] + "," + tokens[4];
                }
            }
        }
    }

    return reviews;
}

// Go to review page
#[get("/k/<kennel>/comments/<id>", rank=1)]
fn get_review_page(kennel: String, id: String) -> Option<NamedFile> {
	// Open file to read reviews
    if let Ok(lines) = read_lines("db/reviews.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(rev) = line {
            	if ( rev.contains(&kennel) && rev.contains(&id) ){ // Review exists
            		let s = format!("{}", "static/pages/review.html");
					return NamedFile::open(s).ok()
                }
            }
        }
    }

  	// No review exists page
	NamedFile::open("static/pages/dne.html").ok()
}

// Return list of reviews in kennel
#[get("/load_reviews?<kennel>")]
fn get_kennel_reviews(kennel: String) -> String {
	// Open file to read revies

	let mut reviews = "".to_owned();

	// File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("db/reviews.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(rev) = line {
            	if rev.contains(&kennel){
                	let tokens:Vec<&str>= rev.split("-").collect();
    				reviews = reviews + "<li><a href=http://localhost:8000/k/" + tokens[0] + "/comments/" + tokens[3] + ">" + tokens[1] + "</a></li>";
    			}
            	
            }
        }
    }

    return reviews;
}

// Hash function
fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}


#[derive(Hash)]
struct RevTest {
	title: String,
	review: String,
}

// Post review
#[post("/k/<kennel>/submit", data = "<input>")]
fn post(kennel: String, input: Form<Review>) -> String {
	// Check if kennel exists
	if ( kennel_exists(&kennel) ){

		// URL to return to after posting if successful
		let s = format!("k/{}", kennel);
		
		let rev = RevTest {
			title: input.title.clone(),
			review: input.review.clone(),
		};

		// Add review to database
		let mut file = OpenOptions::new().append(true).open("db/reviews.txt").expect("fail");
		let review = format!("{}-{}-{}-{}-0\n", kennel, &input.title, &input.review, my_hash(rev));
		file.write_all(review.as_bytes()).expect("faild");

		return s;

	} else {

		// URL to return to if failed posting
		let s = format!("{}", "static/pages/dne.html");
		return s;

	}
}

// Load create review page
#[get("/k/<kennel>/submit", rank = 1)]
fn create(kennel: String) -> Option<NamedFile> {
	// Check if kennel exists
	if ( kennel_exists(&kennel) ){ // Kennel exists, go to submit page
		let s = format!("{}", "static/pages/submit.html");
		NamedFile::open(s).ok()
	} else { // No kennel exist page
		NamedFile::open("static/pages/dne.html").ok()
	}

}

// Return list of kennels
#[get("/home_reviews", rank = 1)]
fn top_reviews() -> String {
	// Get list of reviews in database
    let mut reviews = "".to_owned();

	// File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("db/reviews.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(k) = line {
    			let tokens:Vec<&str>= k.split("-").collect();
    			reviews = reviews + "<li><a href=http://localhost:8000/k/" + tokens[0] + "/comments/" + tokens[3] + ">" + tokens[1] + "</a></li>";
            }
        }
    }

    return reviews;
}

// Return list of kennels
#[get("/home_kennels", rank = 1)]
fn top_kennels() -> String {
	// Get list of kennels in database
    let mut kennels = "".to_owned();

	// File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("db/kennels.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(k) = line {
    			kennels = kennels + "<li><a href=k/" + &k + ">" + &k + "</a></li>";
            }
        }
    }

    return kennels;
}

// Unfollow a kennel 
#[post("/k/<kennel>/unfollow")]
fn unfollow(kennel: String) -> () {
	// Make database calls

}

// Follow a kennel 
#[post("/k/<kennel>/follow")]
fn follow(kennel: String) -> () {
	// Make database calls
	
}

// Load kennel page
#[get("/k/<kennel>", rank = 2)]
fn kennels(kennel: String) -> Option<NamedFile> {
	// Check if kennel exists
    if kennel_exists(&kennel) { // Return kennel page
    	let p = format!("{}", "static/pages/kennel.html");
        NamedFile::open(p).ok()
    } else {
    	// Default no kennel exist page
		NamedFile::open("static/pages/dne.html").ok()
    }

}

// Load kennel page
#[get("/testss", rank = 2)]
fn testss() -> Option<NamedFile> {
	
	NamedFile::open("static/pages/test.html").ok()
    

}

// Generic static file access
#[get("/<file..>", rank = 3)]
fn files(file: PathBuf) -> Option<NamedFile> {
	// Static file access
	NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![files, home, kennels, top_kennels, top_reviews, follow, unfollow, create, post, get_kennel_reviews, get_review_page, load_review_content, testss])
}

fn main() {
    rocket().launch();
}
