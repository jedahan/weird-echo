//! weird echo - take a line of input, and print the last 3 lines of input

use std::{env, io};

fn main() -> Result<(), std::io::Error> {

    // let people seed the first couple of lines from args
    let mut messages = if env::args().count() > 1 {
      env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .split("\n")
        .collect::<Vec<_>>()
    } else {
      vec!["some", "default", "messages", "ok"]
    };

    println!("initial: {:?}", messages);

    loop {
        while messages.iter().count() > 3 {
            if let Some(last_message) = messages.pop() {
              println!("popping '{}'", last_message);
              println!("{}", messages[0]);
              println!("{}", messages[1]);
              println!("{}", messages[2]);
              println!();
              // when last_message goes out of scope here, input should be dropped
            }
        }

        print!("> ");

        let mut input = String::new();
        // this string needs to live as long as it is referenced by messages
        match io::stdin().read_line(&mut input) {
            Ok(_) => messages.insert(0, &input),
            Err(error) => println!("error: {}", error),
        }
    }
}
