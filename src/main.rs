
pub mod file_cleaner;

fn main() {
    
    // ensure that files are utf8 compliant
    // raw data files
    let input_path: &str = "data/raw_data";
    let output_path: &str = "data";

    // clean the files
    file_cleaner::clean_files(input_path, output_path);

}
