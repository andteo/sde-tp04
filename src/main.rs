//use std::fs::File;
//use std::process::Command;
use std::{fs, env};
//use std::os::unix::fs::MetadataExt;
//use std::os::unix::io::AsRawFd;
//use std::io::Read;

fn print (file:&String){
    println!("In file {}", file);

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn size (file:&String)->u64{
    let size = std::fs::metadata(file).unwrap().len();
    size
}



fn main() {
    let args: Vec<String> = env::args().collect();
    //let folder = &args[1];ex1

    let file = &args[1];
    let command= &args[2];
    

    
    
    /*let paths = fs::read_dir(folder).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }*/
    //ex1

    

    match command.as_str(){
        "print"=>print(file),
        "size"=>println!("Marimea fisierului este: {} bytes",size(file)),
        _=> std::process::exit(-1)
    }

   
}
