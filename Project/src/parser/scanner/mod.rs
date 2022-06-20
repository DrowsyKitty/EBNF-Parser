pub mod character_stream;
pub mod token;

use crate::CStream;
use crate::Token;

pub struct Scanner {
    op_list: Vec<char>,
    cstream: CStream,
}

impl Scanner {
    pub fn new(filename: &str) -> Scanner {
        let mut s = Scanner {
            op_list: [
                '(', ',', ')', '{', '}', '=', '<', '>', '!', '+', '-', '*', '/', ';',
            ]
            .to_vec(),
            //https://docs.rs/to_vec/latest/to_vec/
            cstream: CStream::new(&filename.to_string()),
        };
        s.cstream.set_content();
        return s;
    }
    pub fn get_next_token(&mut self) -> Token {
        //let mut t = Token::new();
        //create a new empty string to store token
        let mut s: String = "".to_string();
        let mut char_pos = 0;
        let mut line_num = 0;
        //goes until no more characters in string
        while self.cstream.more_available() {
            let mut next_char = self.cstream.get_next_char();
            //sets the char_pos and line_num to be at the start of the string
            if s.is_empty() {
                char_pos = self.cstream.get_char_pos();
                line_num = self.cstream.get_line_num();
            }
            //declares the token if empty space or new line
            if next_char == Some(' ') || next_char == Some('\n') {
                //if statement in order to not add anything if empty string
                if !s.is_empty() {
                    let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
                    t.set_type();
                    return t;
                }
            }
            //stops getting new chars if an operator char is detected
            else if self.op_list.contains(&next_char.unwrap()) {
                // returns the string if operator was the stoping point
                if !s.is_empty() {
                    let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
                    t.set_type();
                    self.cstream.reduce_char_pos();
                    return t;
                }
                //first adds the char into the string
                else {
                    s.push(next_char.unwrap());
                    //checks if the char is a symbol that can have = after it and still be valid
                    if next_char == Some('=')
                        || next_char == Some('<')
                        || next_char == Some('>')
                        || next_char == Some('!')
                    {
                        next_char = self.cstream.get_next_char();
                        // if next charatcer is =, add it to the token string
                        if next_char == Some('=') {
                            s.push(next_char.unwrap());
                        }
                        // if not reduce char_pos by one to continue correct scanning
                        else {
                            self.cstream.reduce_char_pos();
                        }
                        // creates and returns token
                        let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
                        t.set_type();
                        return t;
                    }
                    // checks if char is - in case its a constant
                    else if next_char == Some('-') {
                        next_char = self.cstream.get_next_char();
                        //if it is a digit, add to token string and contiue as normal
                        if next_char.unwrap().is_ascii_digit() {
                            s.push(next_char.unwrap());
                        }
                        //if not return the token
                        else {
                            let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
                            t.set_type();
                            return t;
                        }
                    }
                    // returns the token if char not -, =, !, <, or >
                    else {
                        let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
                        t.set_type();
                        return t;
                    }
                }
            }
            // if not a special character, add to list and continue
            else {
                s.push(next_char.unwrap());
            }
        }
        //if program ends at a character, use the string created and make that a token
        let mut t: Token = Token::new(s.to_string(), line_num, char_pos);
        t.set_type();
        return t;
    }

    pub fn get_all_tokens(&mut self, v: &mut Vec<Token>) {
        let mut i = 0;
        while self.cstream.more_available() {
            let mut t = self.get_next_token();
            v.push(t);
            i += 1;
        }
    }
}

//pub fn get_next_token
/*fn checkTokenType (text: String) -> TokenType{
    let token_type: TokenType;
    if text ==
}*/

/*fn get_next_token(&mut self) -> Token{
    while let Some(char) = self.get_cur_char
    {
        if ch.is_ascii_digit
        {
            return Token::INTCONSTANT(self.integer()) //create a integer var to change into int
        }
        match char
        {
            '(' => {
                self.get_next_char();
                return Token::KEYWORD
            }
        }


    }
}*/

/*fn pass_whitespace(&mut self)
{
    while ex.more_available() {
        let char = ex.get_next_char();
        if char.is_whitespace()
        {

            self.get_next_char
        }
        else if char.get_token_type = Token::OPERATOR
        {

        }
        else
        {
            break;
        }
    }

}*/
