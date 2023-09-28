//Imports
use std::fs;
use std::path::Path;

//main function
fn main() {
	//Defiine the directory path you want to delete/clean
	let directory_path = "C:\\Windows\\Temp";
	
	//Attempt to list all files in the directory
	match fs::read_dir(&directory_path) {
		Ok(entries) => {
			//Looping Through directory
			for entry in entries {
				if let Ok(entry) = entry {
					//Checking If It's a File
					let file_path = entry.path();
					if file_path.is_file() {
						//Deleting the files
						if let Err(err) = fs::remove_file(&file_path) {
							eprintln!("Error Deleting file {:?}: {}" ,file_path, err);
						} else {
							println!("Deleted file {:?}" , file_path);
						}
					}
				}
			}
		}
		//Error Handling for Directory Reading:
		Err(err) => {
			eprintln!("Error reading directory {:?}: {}", directory_path, err);
		}
	}
}