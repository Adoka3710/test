fn calculate_lenght(s: &String) -> usize{
s.len()
}

fn main(){
   let s1 = String::from("hello world");
   let len = calculate_lenght(&s1);
   println!("The length of '{}' is {}.", s1, len);
}
