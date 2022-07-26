use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {

    // File name to be used in examples
    let target_file: String = "3.filesystem/read_me.fna".to_string();

    /* Old method and new method
     *    There's a clear difference, with the later returning a vector of lines, with the '\n' stripped
     */
    println!("Basic read:");
    println!("```");
    let content = read_file_contents(&target_file);
    println!("{}", content.unwrap());
    println!("```");

    let content = read_file_lines(&target_file);
    println!("Buffered read:");
    println!("{:?}", content.unwrap());

    let content = read_file_nonempty(&target_file);
    let unwrapped_content = content.unwrap();
    println!("Trimmed read:");
    println!("{:?}", unwrapped_content);

    // Once we have content, what do we do with it?
    // The most bioinformatic thing would be to push it to stdout:
    println!("");
    println!("Output time:");
    let mut stdout_channel = io::stdout();
    writeln!(stdout_channel, "{:?}", unwrapped_content).expect("Unable to write!");

    // But it might also be good to push it into a file directly:
    let result = write_to_file(&"my_output.fasta", &unwrapped_content);
    if result.is_ok() {
        println!("Wrote to file!");
    } else {
        println!("Failed to write!");
    }
}

//region File readers

// This is the way used in previous sections, using `read_to_string()` to dump everything into a String
fn read_file_contents(file_path: &str)-> io::Result<String> {
    let mut file_handle = File::open(&file_path)?;
    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;
    Ok(file_content)
}

// This is a better way - reading line by line
fn read_file_lines(file_path: &str) -> io::Result<Vec<String>> {
    
    // Open the file the same way, but then wrap the struct in a `BufReader`
    let file_handle = File::open(&file_path)?;
    let reader = io::BufReader::new(file_handle);

    let mut file_content: Vec<String> = Vec::new();

    // Really we could read everything straight into a vector using `collect()`, but that's not the
    //    point of this example.
    for line in reader.lines() {
        let line = line?;
        file_content.push(line);
    }
    Ok(file_content)
}

/* This is an even better way - this produces the same output, but is faster as we do not need to produce
 *    a new String on each read. Instead, the content is read into the same variable each time and does
 *    not require a new allocation.
 */
fn read_file_nonempty(file_path: &str) -> io::Result<Vec<String>> {

    let file_handle = File::open(&file_path)?;
    let mut reader = io::BufReader::new(file_handle);
    let mut file_content: Vec<String> = Vec::new();

    // We also need a String to write the file contents into.
    // Using an in-place unwrap (`?`) and length check to make sure there is content
    let mut line_content = String::new();
    while reader.read_line(&mut line_content)? > 0 {
        /* Here is the first time I use a block - isolating `content` from the rest of the function.
         *    This is absolutely optional, but a good example of how we can write the code so that
         *    it is not possible to accidentally touch the `content` variable out of scope.
         */
        {
            // Optional, but this method returns linefeeds so they can be removed with a trim operation.
            let content = line_content.trim_end();
            file_content.push(content.to_string());
        }
        line_content.clear();
    }

    Ok(file_content)
}

//endregion

//region Writing content

//
fn write_to_file(file_path: &str, file_content: &Vec<String>) -> io::Result<()> {

    let mut file_writer = File::create(file_path)?;
    for line in file_content {
        write!(file_writer, "{}\n", line)?;
    }

    Ok(())
}

//endregion