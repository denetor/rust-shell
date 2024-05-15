use std::path::PathBuf;


#[derive(Debug)]
pub struct CatCommand {

}

impl CatCommand {
    pub fn do_cat(target_file: PathBuf) {
        println!("Performing file cat");
    }
}