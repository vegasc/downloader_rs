use std::io;
use regex::Regex;

pub struct Inputer {
    regex: String
}

impl Inputer {

    pub const fn new(regex: String) -> Inputer {
        return Inputer{regex: regex};
    }

    pub fn input(&self, msg: String, err_msg: String) -> Result<String, String> {
        if !msg.trim().is_empty() {
            println!("{msg}");  
        }

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let unwraped =  input
        .trim()
        .to_string();

        let regex = Regex::new(&self.regex).unwrap();
        if regex.is_match(&unwraped) {
            return Ok(unwraped);
        } else {
            return Err(err_msg);
        }
    }

}