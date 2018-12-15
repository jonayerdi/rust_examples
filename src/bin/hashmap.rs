use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // Variables moved to map
    // println!("{}", field_name);
    println!("{:?}", map);

    let mut letters = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", letters);

    Ok(())
}
