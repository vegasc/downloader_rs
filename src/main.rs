mod inputer;

use crate::inputer::Inputer;

fn main() {
    let regex = String::from(r"[-a-zA-Z0-9@:%_\+.~#?&//=]{2,256}\.[a-z]{2,4}\b(\/[-a-zA-Z0-9@:%_\+.~#?&//=]*)?");
    let inputer = Inputer::new(regex);

    let result: Result<String, String> = inputer
    .input(String::from("url: "), String::from("error getting url"));

    match result {
        Ok(value) => {
            println!("Value: {value}");
        }
        Err(error) => {
            println!("{error}");
        }
    }
}