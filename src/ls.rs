use std::fs;

#[derive(Debug)]
pub struct Ls {
}

impl Ls {
    pub fn run() {
        // if let Ok(entries) = fs::read_dir(".") {
        //     for entry in entries {
        //         if let Ok(entry) = entry {
        //             // Here, `entry` is a `DirEntry`.
        //             if let Ok(metadata) = entry.metadata() {
        //                 // Now let's show our entry's permissions!
        //                 println!("{:?}: {:?}", entry.path(), metadata.permissions());
        //             } else {
        //                 println!("Couldn't get metadata for {:?}", entry.path());
        //             }
        //         }
        //     }
        // }

        // println!("Performing ls");
        let Ok(entries) = fs::read_dir(".");
        for entry in entries {
            println!("{:?}", entry.path());
        }
    }

    
}
