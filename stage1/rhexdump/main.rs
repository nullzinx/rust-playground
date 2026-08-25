use std::fs;
use clap::Parser;

#[derive(Parser,Debug)]
struct Args {
   target:String,
}

fn convert_and_print_bytes(data:&str){
   for byte in data.bytes() {
       print!("{:02x} ",byte);
   }
}
fn convert_and_print_hex(data:&str){
  let data = data.as_bytes();
  for (offset, _chunk) in data.chunks(8).enumerate() {
    print!(" {:08x}", offset * 8);
  }
}

fn main() {
    println!("rhexdump");
    let args = Args::parse();
    let filedata = fs::read_to_string(args.target).expect("Error in read the file");
    convert_and_print_bytes(&filedata);
    convert_and_print_hex(&filedata);
    println!(" {}",filedata);


}
