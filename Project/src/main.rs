use std::env;
pub mod parser;
use crate::parser::scanner::character_stream::*;
use crate::parser::scanner::token::*;
use crate::parser::scanner::*;
use crate::parser::*;
use std::any::type_name;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    //test for part 1
    /*let mut ex = CStream::new(filename);
    ex.set_content();
    while ex.more_available() {
        let char = ex.get_next_char();
        print!("{}", char.unwrap());
    }*/

    //token tester for part 2
    //let mut token = Token::new("5".to_string(), 0, 0);
    //token.set_type();
    //println!("\n{}", token.get_token_type().as_str());

    //test for part 2
    /*let mut all_vectors: Vec<Token> = Vec::new();
    let mut s = Scanner::new(filename);
    s.get_all_tokens(&mut all_vectors);
    let mut i = 0;
    while i < all_vectors.len() {
        println!("{}", all_vectors[i].get_text());
        i += 1
    }*/
    //testing if error works
    /*let mut p = Parser::new(filename);
    let mut list: Vec<Token> = Vec::new();
    let mut t1: Token = Token::new("5".to_string(), 0, 0);
    t1.set_type();
    let mut t2: Token = Token::new("*".to_string(), 0, 1);
    t2.set_type();
    let mut t3: Token = Token::new("5".to_string(), 0, 2);
    t3.set_type();
    list.push(t1);
    list.push(t2);
    list.push(t3);
    //println!("\n{}", list[0].get_token_type().as_str());
    let r = p.fun_term(list);
    match r {
        Ok(()) => println!("Input program is syntactacilly correct"),
        Err(e) => println!("{}", e),
    }*/
    //tester for part 3
    let mut p = Parser::new(filename);
    //let r = p.tester();
    let r = p.fun_program();
    match r {
        Ok(()) => println!("Input program is syntactacilly correct"),
        Err(e) => println!("{}", e),
    }

    //rust writing to file
    let mut file = File::create("token.xhtml").expect("Creating file failed.");
    file.write_all(b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">")
        .expect("Writing to file failed.");
    file.write_all(b"\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">")
        .expect("Writing to file failed.");

    file.write_all(b"<head> <title> token.xhtml </title> </head>");
    //file.write_all(b"<title> token.xhtml </title>");

    file.write_all(b"<body bgcolor=\"navy\" text=\"orange\" link=\"orange\" vlink=\"orange\">\n<font face=\"Courier New\">\n")
        .expect("Writing to file failed.");

    let mut all_vectors: Vec<Token> = Vec::new();
    let mut s = Scanner::new(filename);
    s.get_all_tokens(&mut all_vectors);
    //let mut i = 0;

    let mut text;
    let mut last_char_num = -1;
    let mut indent_counter = 0;

    for token in all_vectors.iter() {
        text = token.get_text();
        if token.get_text().eq("{") {
            indent_counter += 1;
        } else if token.get_text().eq("}") {
            indent_counter -= 1;
        }
        if token.get_char_num() <= last_char_num {
            file.write_all(b"<br />");
            let mut i = 1;
            while i <= indent_counter {
                file.write_all(b"&nbsp;");
                file.write_all(b"&nbsp;");
                i += 1;
            }
        }
        if token.get_token_type() == TokenType::INTCONSTANT
            || token.get_token_type() == TokenType::FLOATCONSTANT
        {
            file.write_all(b"<font color=\"aqua\"> <b>")
                .expect("Not able to write aqua color to file.");
        } else if token.get_token_type() == TokenType::IDENTIFIER {
            file.write_all(b"<font color=\"yellow\">")
                .expect("Not able to write yellow color to file.");
        } else {
            file.write_all(b"<font color=\"white\"> <b>")
                .expect("Not able to write white color to file.");
        }

        file.write_all(text.as_bytes())
            .expect("Writing text failed ");
        if token.get_token_type() == TokenType::IDENTIFIER {
            file.write_all(b"</font>");
        } else {
            file.write_all(b"</b> </font>");
        }
        last_char_num = token.get_char_num();
        text = token.get_text();
        /*if token.get_char_num() == 0 {
            file.write_all(b"<br />");
            file.write_all(b"&nbsp;");
        }*/
    }

    file.write_all(b" \n </font> </body> </html>")
        .expect("Writing to file failed.");
    //while i < all_vectors.len() {
    //file.write_all(all_vectors[i].get_text());
    //i += 1
    // }
}

//test
