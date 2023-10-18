// Create a Rust function that takes a sentence as input, splits it into words, and
// returns a vector of unique words in alphabetical order.


fn main() {
    let s:String=String::from("Hello how are u my name is ABC I hope your doing good");
    println!("{:?}",unique_words(&s))
}

fn unique_words(s:&String)->Vec<String>{
    let mut words:Vec<&str>=s.split_whitespace().collect();
    words.sort();
    words.dedup();
    words.iter().map(|&s|s.to_string()).collect()
}
