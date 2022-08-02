use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;

fn main() {

    // Equivalent of `~/`
    // This is deprecated now, so not the best way to achieve this functionality.
    let home = env::home_dir().expect("No home!");

    // The PathBuf is basically a String, but grows as a path as values are pushed into it.
    let mut path_to_self = PathBuf::new();
    path_to_self.push(home);
    path_to_self.push("github");
    path_to_self.push("learning_rust");

    println!("The path value is `{}`", path_to_self.display());
    println!("  It is a valid directory: {}", path_to_self.is_dir());

    path_to_self.push("blerp");
    println!("The path value is `{}`", path_to_self.display());
    println!("  It is a valid directory: {}", path_to_self.is_dir());

    // Lets fix the file path, and add a file check
    path_to_self.pop();
    path_to_self.push("3.filesystem");
    path_to_self.push("paths_and_directories.rs");
    println!("The path value is `{}`", path_to_self.display());
    println!("  It is a valid directory: {}", path_to_self.is_dir());
    println!("  It is a valid file:      {}", path_to_self.is_file());
    println!("");

    println!("Time to deplete the PathBuf:");
    let mut dupl_path = PathBuf::from(&path_to_self);
    loop {
        println!("  `{}`", dupl_path.display());

        if ! dupl_path.pop() {
            break;
        }
    }
    println!("");

    // Next thing, lets look at file metadata
    println!("`{}`", path_to_self.display());
    match path_to_self.metadata() {
        Ok(data) => {
            println!("  File type:     {:?}", data.file_type());
            println!("  Length:        {}", data.len());
            // Cross-platform attempt at permissions:
            //println!("  Permissions:   {:?}", data.permissions());
            // Unix permissions
            println!("  Permissions:   {:?}", data.permissions().mode());
            println!("  Last modified: {:?}", data.modified());
        },
        Err(e) => println!("Error {:?}", e)
    }

    // Finally, look through a directory
    //    Using a function here to remind myself about io::Result() use.
    path_to_self.pop();
    let iter_result = trawl_directory(&path_to_self);
    match iter_result {
        Ok(()) => (), // Unit acts as a handy noop...
        Err(_) => println!("Unable to parse folder {}", path_to_self.display())
    };
}

fn trawl_directory(folder: &PathBuf) -> io::Result<()> {

    println!("");
    println!("Iterating contents of `{}`", folder.display());
    for entry in fs::read_dir(folder)? { // Need to unpack here

        let entry = entry?;
        let entry_path = entry.path();
        if let Some(ex) = entry_path.extension() {
            if ex == ("rs") {
                println!("  Rust file: {:?}", entry_path);
            } else {
                println!("  Other file: {:?}", entry_path);
            }
        }
    }
    Ok(())
}