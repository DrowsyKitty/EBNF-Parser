#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    INTCONSTANT,
    FLOATCONSTANT,
    KEYWORD,
    OPERATOR,
    IDENTIFIER,
    INVALID,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TokenType::INTCONSTANT => "IntConstant",
            TokenType::FLOATCONSTANT => "FloatConstant",
            TokenType::KEYWORD => "Keyword",
            TokenType::OPERATOR => "Operator",
            TokenType::IDENTIFIER => "Identifier",
            TokenType::INVALID => "Invalid",
        }
    }
}
#[derive(Clone)]
pub struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_num: i32,
}

impl Token {
    pub fn new(s: String, ln: i32, cn: i32) -> Token {
        Token {
            text: s,
            token_type: TokenType::INVALID,
            line_num: ln,
            char_num: cn,
        }
    }

    pub fn set_type(&mut self) {
        //checks if the word is a keyword
        if self.text.chars().nth(0) == None {
            self.token_type = TokenType::INVALID;
        } else if self.text == "unsigned"
            || self.text == "char"
            || self.text == "short"
            || self.text == "int"
            || self.text == "long"
            || self.text == "float"
            || self.text == "double"
            || self.text == "while"
            || self.text == "if"
            || self.text == "return"
            || self.text == "void"
            || self.text == "main"
        {
            self.token_type = TokenType::KEYWORD;
        }
        //checks for operator
        else if self.text == "("
            || self.text == ","
            || self.text == ")"
            || self.text == "{"
            || self.text == "}"
            || self.text == "="
            || self.text == "=="
            || self.text == "<"
            || self.text == ">"
            || self.text == "<="
            || self.text == ">="
            || self.text == "!="
            || self.text == "+"
            || self.text == "-"
            || self.text == "*"
            || self.text == "/"
            || self.text == ";"
        {
            self.token_type = TokenType::OPERATOR;
        }
        // looks to see if first char is '_' to see if it's a valid identifier
        else if self.text.chars().nth(0).unwrap() == '_' {
            let mut i = 1;
            while i < self.text.chars().count().try_into().unwrap() {
                //makes sure only alphas and digits can show up
                if !self.text.chars().nth(i).unwrap().is_ascii_alphabetic()
                    && !self.text.chars().nth(i).unwrap().is_ascii_digit()
                {
                    self.token_type = TokenType::INVALID;
                    return;
                }
                i += 1;
            }
            self.token_type = TokenType::IDENTIFIER;
        }
        // looks to see if first char is '-' to see if it's a valid constant
        else if self.text.chars().nth(0).unwrap() == '-' {
            if !self.text.chars().nth(1).unwrap().is_ascii_digit() {
                self.token_type = TokenType::INVALID;
            } else {
                let mut i = 2;
                let mut is_float = false;
                while i < self.text.chars().count().try_into().unwrap() {
                    // checks for '.' in case of float but only is done once
                    if self.text.chars().nth(i).unwrap() == '.' && is_float == false {
                        is_float = true;
                    }
                    // makes sure only digits show up or repeat '.'
                    else if !self.text.chars().nth(i).unwrap().is_ascii_digit() {
                        self.token_type = TokenType::INVALID;
                        return;
                    }
                    i += 1;
                }
                // if worked sets to float if '.' detected if not, only an int
                if is_float {
                    self.token_type = TokenType::FLOATCONSTANT;
                } else {
                    self.token_type = TokenType::INTCONSTANT;
                }
            }
        }
        // looks to see if first char is a digit to see if it's a valid constant
        else if self.text.chars().nth(0).unwrap().is_ascii_digit() {
            let mut i = 1;
            let mut is_float = false;
            while i < self.text.chars().count().try_into().unwrap() {
                // checks for '.' in case of float but only is done once
                if self.text.chars().nth(i).unwrap() == '.' && is_float == false {
                    is_float = true;
                }
                // makes sure only digits show up or repeat '.'
                else if !self.text.chars().nth(i).unwrap().is_ascii_digit() {
                    self.token_type = TokenType::INVALID;
                    return;
                }
                i += 1;
            }
            // if worked sets to float if '.' detected if not, only an int
            if is_float {
                self.token_type = TokenType::FLOATCONSTANT;
            } else {
                self.token_type = TokenType::INTCONSTANT;
            }
        }
        // checks if first char is an alpha in order to see if its a valid identifier
        else if self.text.chars().nth(0).unwrap().is_ascii_alphabetic() {
            let mut i = 1;
            while i < self.text.chars().count().try_into().unwrap() {
                // checks if chars are '_', alpha, or digit
                if self.text.chars().nth(i).unwrap() != '_'
                    && !self.text.chars().nth(i).unwrap().is_ascii_alphabetic()
                    && !self.text.chars().nth(i).unwrap().is_ascii_digit()
                {
                    self.token_type = TokenType::INVALID;
                    return;
                }
                i += 1;
            }
            self.token_type = TokenType::IDENTIFIER;
        } else {
            self.token_type = TokenType::INVALID;
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_text(&self) -> String {
        format!("{}", self.text)
    }

    pub fn get_line_num(&self) -> i32 {
        self.line_num
    }

    pub fn get_char_num(&self) -> i32 {
        self.char_num
    }
}
