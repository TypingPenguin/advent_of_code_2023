use std::fs::File;
use std::io::Read;

// Load a txt file and return a string
pub(crate) fn load_txt_file(path : &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}