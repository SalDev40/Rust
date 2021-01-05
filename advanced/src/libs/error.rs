#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod Errors {
    use std::io::Read;

    pub fn Basic() {
        let filePath = "./src/libs/hello1.txt";
        match std::fs::File::open(filePath) {
            Ok(f) => println!("File Opened: {:?}", f),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => match std::fs::File::create(filePath) {
                    Ok(f) => println!("File created: {:?}", f),
                    Err(e) => println!("Error in creating file: {:?}", e),
                },
                otherError => panic!("Problem opening the file: {:?}", otherError),
            },
        };
    }

    pub fn ShortCuts() {
        let filePath = "./src/libs/hello1.txt";
        // let f = std::fs::File::open(filePath).unwrap();
        let f = std::fs::File::open(filePath).expect("Error in opening file");
        println!("File: {:?}", f);
    }

    pub fn ReadFromFile() -> Result<String, std::io::Error> {
        let filePath = "./src/libs/hello.txt";
        let mut f = match std::fs::File::open(filePath) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut data = String::new();
        match f.read_to_string(&mut data) {
            Ok(_) => return Ok(data),
            Err(e) => return Err(e),
        }
    }

    pub fn PropogateReadFromFile() -> Result<String, std::io::Error> {
        let mut s = String::new();
        let filePath = "./src/libs/hello.txt";
        std::fs::File::open(filePath)?.read_to_string(&mut s)?;
        Ok(s)
    }
}

pub fn errMain() {
    Errors::Basic();
    Errors::ShortCuts();
    let data = Errors::ReadFromFile().expect("error opening file");
    println!("{}", data);
    let data = Errors::PropogateReadFromFile().expect("error opening file");
    println!("{}", data);
}
