
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn clean_files(input_path: &str, output_path: &str) {
    // convert to path
    let input_path: &Path = Path::new(input_path);
    let output_path: &Path = Path::new(output_path);

    // get vec of &Path to .txt files in the input directory
    let file_paths: Vec<PathBuf> = input_path.read_dir().unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() && path.extension().unwrap() == "txt")
        .collect::<Vec<_>>();

    // print the number of files and the files
    let num_files: usize = file_paths.len();
    println!("Number of files found: {num_files}");

    for file_path in file_paths {

        println!("Cleaning file: {file_path:?}");

        let cleaned_file: String = read_and_clean_file(&file_path).unwrap();

        // create the output file
        let output_file: PathBuf = {
            let mut file_name = file_path.file_stem().unwrap().to_os_string();
            file_name.push("_cleaned");
            let mut new_file_name = PathBuf::from(file_name);
            new_file_name.set_extension(file_path.extension().unwrap());
            output_path.join(new_file_name)
        };

        // write the cleaned file
        write_cleaned_file(cleaned_file, &output_file);
        println!("File cleaned and saved to: {output_file:?}");
    }

}

fn read_and_clean_file(file_path: &PathBuf) -> io::Result<String> {
    // read the file
    let mut file: File = File::open(file_path).unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    // convert bytes to string replacing invalid utf8 characters
    let content: String = String::from_utf8_lossy(&content).to_string();

    // split the file into lines
    let lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    // clean the file
    let cleaned_file = clean_file(lines);

    // return the cleaned file
    Ok(cleaned_file)
}

fn clean_file(lines: Vec<String>) -> String {
    // Clean the file by replacing non-UTF-8 characters
    let cleaned_file: String = lines
        .join("\n")
        .chars()
        .filter(|&c| c.is_ascii() || c.is_alphabetic() || c.is_numeric() || c.is_whitespace() || c.is_ascii_punctuation())
        .collect();

    // Return the cleaned file
    cleaned_file
}

fn write_cleaned_file(cleaned_file: String, output_file: &PathBuf) {
    // write the cleaned file
    let mut file: File = File::create(output_file).unwrap();
    file.write_all(cleaned_file.as_bytes()).unwrap();
}