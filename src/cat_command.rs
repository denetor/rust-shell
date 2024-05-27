#[derive(Debug)]
pub struct CatCommand {

}

impl CatCommand {
    pub fn do_cat() {
        println!("Performing file cat");
    }
}