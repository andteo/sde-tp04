//use std::fs::File;
//use std::process::Command;
use std::{fs, env, os::{macos::fs::MetadataExt, unix::prelude::{PermissionsExt, FileTypeExt}}};
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

fn owner (file:&String){
    let uid = std::fs::metadata(file).unwrap().st_uid();
    let gid = std::fs::metadata(file).unwrap().st_gid();
    println!("Proprietarul fisierului este: {} {}",uid,gid);
}

fn mode_number(file:&String){
    let perm = std::fs::metadata(file).unwrap().permissions();
    println!("Permisiunile fisierului sunt: {:o}",perm.mode());
}

fn tip (file:&String){
    let tip=std::fs::metadata(file).unwrap().file_type();
    if tip.is_dir(){
        println!("Este director!");
    }
    else if tip.is_file(){
        println!("Este fisier!");
    }
    else if tip.is_char_device(){
        println!("Este char device!");
    }
    else if tip.is_block_device(){
        println!("E bloc device!");
    }
    else if tip.is_symlink(){
        println!("E system link");
    }
    else if tip.is_socket(){
        println!("E socket");
    }
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
        "owner"=>owner(file),
        "mode_number"=>mode_number(file),
        "type"=>tip(file),
        _=> std::process::exit(-1)
    }

   
}
