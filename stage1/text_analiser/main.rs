// Borrowwing study

fn count_characters(text:&str){
   println!("{}",text.len());
}

fn lowercase(text:&str){
   println!("{}",text.to_lowercase());
}

fn analyze(target:&str){
   println!("{}",target);
   lowercase(target);
   count_characters(target);
}


fn main(){
  let text = String::from("The Industrial Revolution and its consequences were the greatest misfortune to befall humankind.");
  analyze(&text);
}
