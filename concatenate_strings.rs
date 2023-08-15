fn main (){
let string1:&str="Hello ";
let string2:&str="world";
let concatenated_string=concatenate_strings(&string1[0..string1.len()],&string2[0..string2.len()]);

println!("{}",concatenated_string)
}

   
fn concatenate_strings(s1:&str,s2:&str)->String {
     let mut result = String::from(" ");
     result.push_str(s1);
     result.push_str(s2);
     return result
}
