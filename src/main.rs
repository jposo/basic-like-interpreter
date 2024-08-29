mod scanner;
//mod parser;
mod executer;
use std::fs;

//fn print_tree(root: &parser::Node, level: u8) {
  //  println!("{}> {}", "-".repeat(level as usize), root);
    //for child in &root.children {
      //  print_tree(&child, level + 1);
    //}
//}

            //let ast = parser::parse(&tokens);
            //match ast {
              //  Ok(root) => {
                //        print_tree(&root, 0);
                //},
                //Err(message) => {
                  //  println!("ERROR ON PARSER: {}", message);
                //}
fn main() {
    let contents = fs::read_to_string("src/buzz.jpo")
        .expect("Should have been able to read the file");
    
    let tokens = scanner::scan(&contents);
    match tokens {
        Ok(tokens) => {
            let result = executer::execute(&tokens);
            match result {
                Err(message) => {
                    println!("{}", message);
                },
                Ok(_) => {},
            }
        },
        Err(message) => {
            println!("ERROR ON SCANNER: {}", message)
        }
    }
}
