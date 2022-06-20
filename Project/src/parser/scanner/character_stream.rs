use std::fs::File;
use std::io;
use std::io::prelude::*;

//struct items
pub struct CStream {
    filename: String,
    line_num: i32,
    char_pos: i32,
    content: String,
    overall_pos: i32,
    size: i32,
}

impl CStream {
    //initializer
    pub fn new(f: &String) -> CStream {
        CStream {
            filename: f.to_string(),
            line_num: -1,
            char_pos: -1,
            content: String::new(),
            overall_pos: -1,
            size: 0,
        }
    }
    //sets the content based on what's in the file
    pub fn set_content(&mut self) -> io::Result<()> {
        let file = File::open(self.filename.as_str())?;
        let mut buf_reader = io::BufReader::new(file);
        buf_reader.read_to_string(&mut self.content);
        self.size = self.content.chars().count() as i32;
        Ok(())
    }
    //returns true if more characters availble
    pub fn more_available(&self) -> bool {
        self.overall_pos < self.size - 1
    }
    //returns the next char in the file
    pub fn get_next_char(&mut self) -> Option<char> {
        let cur = self.content.chars().nth((self.overall_pos) as usize);
        match cur {
            None => {
                self.char_pos += 1;
                self.line_num += 1;
            }
            Some(x) => {
                if x == '\n' {
                    self.char_pos = 0;
                    self.line_num += 1;
                } else {
                    self.char_pos += 1;
                    if self.overall_pos == -1 {
                        self.line_num += 1;
                    }
                }
            }
        }
        self.overall_pos += 1;
        self.content.chars().nth((self.overall_pos) as usize)
    }

    pub fn get_char_pos(&self) -> i32 {
        self.char_pos
    }

    pub fn get_line_num(&self) -> i32 {
        self.line_num
    }

    pub fn reduce_char_pos(&mut self) {
        self.char_pos -= 1;
        self.overall_pos -= 1;
    }

    pub fn get_overall_pos(&self) -> i32 {
        self.overall_pos
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}
