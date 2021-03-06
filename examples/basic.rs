#![feature(phase)]
#![feature(if_let)]

#[phase(plugin, link)]
extern crate incidents;

use incidents::{Error, FResult, print_traceback};

#[deriving(Clone)]
struct BadBehavior;

impl Error for BadBehavior {
    fn name(&self) -> &str {
        "Bad Behavior"
    }
}

#[deriving(Clone)]
struct FileNotFound {
    file: Option<Path>,
}

impl Error for FileNotFound {
    fn name(&self) -> &str {
        "File Not Found"
    }

    fn detail(&self) -> Option<String> {
        match self.file {
            Some(ref file) => {
                Some(format!("file={}", file.display()))
            },
            None => None,
        }
    }
}

fn testing() -> FResult<(), FileNotFound> {
    fail!(FileNotFound { file: Some(Path::new("/missing.txt")) });
}

fn bubble() -> FResult<(), FileNotFound> {
    try!(testing());
    Ok(())
}

fn bar() -> Result<(), BadBehavior> {
    Err(BadBehavior)
}

fn test() -> Result<(), BadBehavior> {
    try!(bar());
    Ok(())
}

fn downgrade() -> Result<(), FileNotFound> {
    let _ = try!(bubble());
    Ok(())
}

fn main() {
    let _ = test();
    match bubble() {
        Ok(x) => println!("Produced {}", x),
        Err(ref err) => {
            print_traceback(err);
            match err.file {
                Some(ref f) => println!("filename = {}", f.display()),
                None => {}
            }
        }
    }

    match downgrade() {
        Err(err) => println!("Error: {}", err.name()),
        Ok(_) => {}
    }
}
