fn main() {
    let first_string:String = String::from("Hello ");
    let second_string:String = String::from("World!");
    
    println!("{}", concatenate_strings(&first_string, &second_string));

}

fn concatenate_strings(first_string: &String, second_string : &String)->String{
    let mut result = String::new();
    result.push_str(first_string);
    result.push_str(second_string);
    result
}