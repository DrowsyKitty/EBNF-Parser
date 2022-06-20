pub mod scanner;
extern crate custom_error;
use crate::parser::scanner::token::*;
use crate::parser::scanner::*;
use custom_error::custom_error;

custom_error! {#[derive(PartialEq)] pub MyError
    General{ln: i32, cn:i32, s:String} = "Error at Line {ln} Character {cn}. The syntax should be {s}"
}

pub struct Parser {
    scanner: Scanner,
    counter: usize,
    err_string: String,
}

impl Parser {
    pub fn new(filename: &str) -> Parser {
        Parser {
            scanner: Scanner::new(filename),
            counter: 0,
            err_string: "".to_string(),
        }
    }
    // ~ = non-terminal
    // \ = TokenType

    // {Declaration} MainDeclaration {FunctionDefinition}
    pub fn fun_program(&mut self) -> Result<(), MyError> {
        //get make Vec of Tokens and sets it using the scanner
        let mut v: Vec<Token> = Vec::new();
        self.scanner.get_all_tokens(&mut v);
        let mut t = &v[self.counter];
        //checks if first token is following since Declaration always starts with those tokens
        //while loop since it can be 0 or more
        while t.get_text().eq("unsigned")
            || t.get_text().eq("char")
            || t.get_text().eq("short")
            || t.get_text().eq("int")
            || t.get_text().eq("long")
            || t.get_text().eq("float")
            || t.get_text().eq("double")
        {
            let e: Result<(), MyError> = self.fun_declaration(v.clone());
            //if error occurs in Declaration, returns it
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //if no tokens remain, cause an error since MainDeclaration has to happen
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Program := {Declaration} MainDeclaration {FunctionDefinition}".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            // gets next token to see if it's another Declaration
            t = &v[self.counter];
        }
        //calls MainDeclaration and returns error if error occured within
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Program := {Declaration} MainDeclaration {FunctionDefinition}".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        let mut e: Result<(), MyError> = self.fun_main_declaration(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            return Ok(());
        }
        t = &v[self.counter];
        //checks for following tokens since function definition starts with those tokens
        //while loop since it's 0+ times
        while t.get_text().eq("unsigned")
            || t.get_text().eq("char")
            || t.get_text().eq("short")
            || t.get_text().eq("int")
            || t.get_text().eq("long")
            || t.get_text().eq("float")
            || t.get_text().eq("double")
        {
            e = self.fun_function_definition(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            if self.no_tokens_left(v.clone()) {
                return Ok(());
            }
            t = &v[self.counter];
        }
        //may need to add a check to make sure no tokens remain
        Ok(())
    }
    // DeclarationType (VariableDeclaration | FunctionDeclaration)
    pub fn fun_declaration(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        let mut e = self.fun_declaration_type(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Declaration := DeclarationType (VariableDeclaration | FunctionDeclaration)"
                    .to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        e = self.fun_variable_declaration(v.clone());
        if e.is_err() {
            e = self.fun_function_declaration(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            return Ok(());
        }
        return Ok(());
    }
    // ~void ~main ~( ~) Block
    pub fn fun_main_declaration(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        if !t.get_text().eq("void") {
            self.err_string = "MainDeclaration := void main ( ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter = self.counter + 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string = "MainDeclaration := void main ( ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        if !t.get_text().eq("main") {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter = self.counter + 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string = "MainDeclaration := void main ( ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        if !t.get_text().eq("(") {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter = self.counter + 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string = "MainDeclaration := void main ( ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        if !t.get_text().eq(")") {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter = self.counter + 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string = "MainDeclaration := void main ( ) Block".to_string();

            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        let mut e = self.fun_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        } else {
            return Ok(());
        }
    }
    // DeclarationType ParameterBlock Block
    pub fn fun_function_definition(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        let mut e = self.fun_declaration_type(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "FunctionDefinition := DeclarationType ParameterBlock Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        e = self.fun_parameter_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "FunctionDefinition := DeclarationType ParameterBlock Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        e = self.fun_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        return Ok(());
    }
    // DataType \Identifer
    pub fn fun_declaration_type(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //Run DataType, error if DataType is error
        let mut t = &v[self.counter];
        let e = self.fun_data_type(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //check if tokens left, error since Identifier needed
        if self.no_tokens_left(v.clone()) {
            self.err_string = "DeclarationType := DataType Identifier".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, error if not identifier
        t = &v[self.counter];
        if t.get_token_type() != TokenType::IDENTIFIER {
            self.err_string = "DeclarationType := DataType Identifier".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        return Ok(());
    }
    // [~= Constant] ~;
    pub fn fun_variable_declaration(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        if t.get_text().eq("=") {
            self.counter = self.counter + 1;
            if self.no_tokens_left(v.clone()) {
                self.err_string = "VariableDeclaration := [= Constant] ;".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            t = &v[self.counter];
            let e = self.fun_constant(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string = "VariableDeclaration := [= Constant] ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        if !t.get_text().eq(";") {
            self.err_string = "VariableDeclaration := [= Constant] ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter = self.counter + 1;
        return Ok(());
    }
    // ParameterBlock ~;
    pub fn fun_function_declaration(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        let e = self.fun_parameter_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string = "FunctionDeclaration := ParameterBlock ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        let mut t = &v[self.counter];
        if !t.get_text().eq(";") {
            self.err_string = "FunctionDeclaration := ParameterBlock ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        return Ok(());
    }
    // ~{ {Declaration} {Statement} {FunctionDefinition} ~}
    pub fn fun_block(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //get first token, if not {, then error
        let mut t = &v[self.counter];
        if !t.get_text().eq("{") {
            self.err_string =
                "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment, check if tokens left, error since }
        self.counter += 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, enter while loop if starts with DataType
        t = &v[self.counter];
        while t.get_text().eq("unsigned")
            || t.get_text().eq("int")
            || t.get_text().eq("char")
            || t.get_text().eq("short")
            || t.get_text().eq("long")
            || t.get_text().eq("double")
            || t.get_text().eq("float")
        {
            //call declaration, return error if declaration was error
            let mut e = self.fun_declaration(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //error if no tokens left because }
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, in loop if data type
            t = &v[self.counter];
        }
        //go in loop if any of the following, gotten from items needed for statement
        while t.get_token_type() == TokenType::IDENTIFIER
            || t.get_token_type() == TokenType::INTCONSTANT
            || t.get_token_type() == TokenType::FLOATCONSTANT
            || t.get_text().eq("while")
            || t.get_text().eq("if")
            || t.get_text().eq("return")
            || t.get_text().eq("(")
        {
            //call Statment, error if Statement was error
            let mut e = self.fun_statement(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //error if no tokens left because }
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, in loop if any of the previous
            t = &v[self.counter];
        }
        //enter loop if token is DataType
        while t.get_text().eq("unsigned")
            || t.get_text().eq("int")
            || t.get_text().eq("char")
            || t.get_text().eq("short")
            || t.get_text().eq("long")
            || t.get_text().eq("double")
            || t.get_text().eq("float")
        {
            //call FunctionDefinition, return error if FunctionDefinition was error
            let mut e = self.fun_function_definition(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //error if no tokens left because }
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, in loop if data type
            t = &v[self.counter];
        }
        if !t.get_text().eq("}") {
            self.err_string =
                "Block := { {Declaration} {Statement} {FunctionDefinition} }".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        return Ok(());
    }
    // ~( [Parameter {~, Parameter}] ~)
    pub fn fun_parameter_block(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //get first token, err if not (, incrememnt if so
        let mut t = &v[self.counter];
        if !t.get_text().eq("(") {
            self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        //error if no tokens left, needed for )
        if self.no_tokens_left(v.clone()) {
            self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, run parameter, if error, continue on since optional
        t = &v[self.counter];
        let mut e = self.fun_parameter(v.clone());
        if e.is_ok() {
            //returns err if no tokens because of )
            if self.no_tokens_left(v.clone()) {
                self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, go into while loop if ,
            t = &v[self.counter];
            while t.get_text().eq(",") {
                //check if any tokens left, error because of Parameter )
                self.counter += 1;
                if self.no_tokens_left(v.clone()) {
                    self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
                    return Err(MyError::General {
                        ln: t.get_line_num(),
                        cn: t.get_char_num(),
                        s: self.err_string.clone(),
                    });
                }
                //get next token, run Parameter, error if error returned
                t = &v[self.counter];
                e = self.fun_parameter(v.clone());
                if e.is_err() {
                    self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
                    return Err(MyError::General {
                        ln: t.get_line_num(),
                        cn: t.get_char_num(),
                        s: self.err_string.clone(),
                    });
                }
                //check for more tokens, error because of )
                if self.no_tokens_left(v.clone()) {
                    self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
                    return Err(MyError::General {
                        ln: t.get_line_num(),
                        cn: t.get_char_num(),
                        s: self.err_string.clone(),
                    });
                }
                //get next token, stays in loop if ,
                t = &v[self.counter];
            }
        }
        if !t.get_text().eq(")") {
            self.err_string = "ParameterBlock := ( [Parameter {, Parameter}] )".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment and ok
        self.counter += 1;
        return Ok(());
    }
    // IntegerType | FloatType
    pub fn fun_data_type(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //set token and call FloatType
        let t = &v[self.counter];
        let mut e: Result<(), MyError> = self.fun_float_type(v.clone());
        if e.is_ok() {
            return Ok(());
        }
        //if fails, check IntegerType first
        else {
            e = self.fun_integer_type(v.clone());
            //happens if both FloatType and IntType fail, should return error
            if e.is_err() {
                self.err_string = "DataType := IntegerType | FloatType".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            } else {
                Ok(())
            }
        }
    }
    // \IntConstant | \FloatConstant
    pub fn fun_constant(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //get token and see if it's an IntConstant or Float, if not return error
        let mut t = &v[self.counter];
        if t.get_token_type() != TokenType::INTCONSTANT
            && t.get_token_type() != TokenType::FLOATCONSTANT
        {
            self.err_string = "Constant := IntegerConstant | FloatConstant".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //if so, incremement counter and say it's ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ~;)
    pub fn fun_statement(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        if t.get_token_type() == TokenType::IDENTIFIER {
            let mut e = self.fun_assignment(v.clone());
            if e.is_ok() {
                return Ok(());
                /*return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });*/
            }
            //return Ok(());
        } else if t.get_text().eq("while") {
            let mut e = self.fun_while_loop(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            return Ok(());
        } else if t.get_text().eq("if") {
            let mut e = self.fun_if_statement(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            return Ok(());
        } else if t.get_text().eq("return") {
            let mut e = self.fun_return_statement(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            return Ok(());
        }
        let mut e = self.fun_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        if self.no_tokens_left(v.clone()) {
            self.err_string = "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        if !t.get_text().eq(";") {
            self.err_string = "Statement := Assignment | WhileLoop | IfStatement | ReturnStatement | (Expression ;)".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        return Ok(());
    }
    // DataType \Identifier
    pub fn fun_parameter(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //Run DataType, error if DataType is error
        let mut t = &v[self.counter];
        let e = self.fun_data_type(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //check if tokens left, error since Identifier needed
        if self.no_tokens_left(v.clone()) {
            self.err_string = "Parameter := DataType Identifier".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, error if not identifier
        t = &v[self.counter];
        if t.get_token_type() != TokenType::IDENTIFIER {
            self.err_string = "Parameter := DataType Identifier".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        self.counter += 1;
        return Ok(());
    }
    // [~unsigned] ( ~char | ~short | ~int | ~long)
    pub fn fun_integer_type(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        //check if token is "unsigned", if so, incremement counter
        if t.get_text().eq("unsigned") {
            self.counter += 1;
            //if no tokens left, call an error since just having "unsigned" is not valid
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "IntegerType := [unsigned] ( char | short | int | long);".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get the next token
            t = &v[self.counter];
        }
        //checks if the token is any of the following, if not return an error
        if !t.get_text().eq("char")
            && !t.get_text().eq("short")
            && !t.get_text().eq("int")
            && !t.get_text().eq("long")
        {
            self.err_string = "IntegerType := [unsigned] ( char | short | int | long)".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //if so, increment counter to get next token and return Ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // ~float | ~double
    pub fn fun_float_type(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        //check if token is float or double, if not return error
        if !t.get_text().eq("float") && !t.get_text().eq("double") {
            self.err_string = "FloatType := float | double".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //if so, incrememnt for next token and return Ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // \Identifier ~= {\Identifier ~=} Expression ~;
    pub fn fun_assignment(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        //check if first token is Identifier, return error if not
        if t.get_token_type() != TokenType::IDENTIFIER {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //if so incrememnt counter
        else {
            self.counter += 1;
        }
        //checks if there are still tokens, if not, return error since more parts are needed for assignment
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //else, get the next token and see if it's an =, return error if not and increment if so
        t = &v[self.counter];
        if !t.get_text().eq("=") {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        } else {
            self.counter += 1;
        }
        //return error if no tokens left since Expression and ; are still needed
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token and check if it's an identifier, if not, no error since it's 0+ times
        t = &v[self.counter];
        //while loop for Identifier =
        while t.get_token_type() == TokenType::IDENTIFIER {
            self.counter += 1;
            //return error if no tokens left since = needed in the loop
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token and see if it's =, if not return error
            t = &v[self.counter];
            if !t.get_text().eq("=") {
                /*self.err_string =
                    "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });*/
                self.counter -= 2;
            }
            //incremenet counter to get next token
            else {
                self.counter += 1;
            }
            //check if tokens left, if not, return error since Expression ; are still needed
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, if it's an Identifier it stays in the loop
            t = &v[self.counter];
        }
        self.counter += 1;
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, if it's an Identifier it stays in the loop
        t = &v[self.counter];
        //call Expression and if error was returned within, returns an error
        let e: Result<(), MyError> = self.fun_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //checks if tokens left, if no, return error since ; needed
        if self.no_tokens_left(v.clone()) {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token and see if it's ;, return error if not
        t = &v[self.counter];
        if !t.get_text().eq(";") {
            self.err_string =
                "Assignment := Identifier = {{Identifier =}} Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment counter and return Ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // ~while ~( Expression ~) Block
    pub fn fun_while_loop(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //check if first token is while, if not return error
        let mut t = &v[self.counter];
        if !t.get_text().eq("while") {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment if it is while
        else {
            self.counter += 1;
        }
        //check if tokens left, if not return error since (Expression) Block needed
        if self.no_tokens_left(v.clone()) {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        // get next token and see if it is (, return error if not
        t = &v[self.counter];
        if !t.get_text().eq("(") {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment for next token
        else {
            self.counter += 1;
        }
        //call Expression, if error returned from function, return error
        let mut e: Result<(), MyError> = self.fun_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //check if tokens left, if not throw error since ) Block needed
        if self.no_tokens_left(v.clone()) {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, check if it's ), return error if not
        t = &v[self.counter];
        if !t.get_text().eq(")") {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment counter for next token
        else {
            self.counter += 1;
        }
        //call block, if error returned in fun, return error
        if self.no_tokens_left(v.clone()) {
            self.err_string = "WhileLoop := while ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        e = self.fun_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        Ok(())
    }
    // ~if ~( Expression ~) Block
    pub fn fun_if_statement(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //check if first token is if, return error if not
        let mut t = &v[self.counter];
        if !t.get_text().eq("if") {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment counter for next token
        else {
            self.counter += 1;
        }
        //check if tokens left, if not return error because of ( Expression ) Block needed
        if self.no_tokens_left(v.clone()) {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, check if (, return error if not
        t = &v[self.counter];
        if !t.get_text().eq("(") {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment for next token
        else {
            self.counter += 1;
        }
        //call Expression, returns error if function returned an error
        let mut e: Result<(), MyError> = self.fun_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //check if no tokens, if so returns error because of ) Block
        if self.no_tokens_left(v.clone()) {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, see if ), return error if not
        t = &v[self.counter];
        if !t.get_text().eq(")") {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //incrememt for next token
        else {
            self.counter += 1;
        }
        //call block, returns error if block returned error
        if self.no_tokens_left(v.clone()) {
            self.err_string = "IfStatement := if ( Expression ) Block".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        t = &v[self.counter];
        e = self.fun_block(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        Ok(())
    }
    // ~return Expression ~;
    pub fn fun_return_statement(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //see if first token is return, returns error if not
        let mut t = &v[self.counter];
        if !t.get_text().eq("return") {
            self.err_string = "ReturnStatement := return Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment for next token
        else {
            self.counter += 1;
        }
        //call Expression, returns error if Expression returned error
        let e: Result<(), MyError> = self.fun_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //check if tokens remain, call error if not for ;
        if self.no_tokens_left(v.clone()) {
            self.err_string = "ReturnStatement := return Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //get next token, check if ;, error if not
        t = &v[self.counter];
        if !t.get_text().eq(";") {
            self.err_string = "ReturnStatement := return Expression ;".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //increment and return ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // SimpleExpression [ RelationOperator SimpleExpression]
    pub fn fun_expression(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //Call SimpleExpression, error if SimpleExpression was error
        let mut t = &v[self.counter];
        let mut e: Result<(), MyError> = self.fun_simple_expression(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        } //check if tokens remain, if not Ok because that's the last required token
        if self.no_tokens_left(v.clone()) {
            return Ok(());
        }
        //get next token, call RelationOperator
        t = &v[self.counter];
        e = self.fun_relation_operator(v.clone());
        //if Ok, check if tokens remain, since needed for SimpleExpression
        if e.is_ok() {
            if self.no_tokens_left(v.clone()) {
                self.err_string =
                    "Expression := SimpleExpression [ RelationOperator SimpleExpression]"
                        .to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, and run SimpleExpression, error if SimpleExpression was error
            t = &v[self.counter];
            e = self.fun_simple_expression(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
        }
        //if not RelationOperator, still return Ok
        return Ok(());
    }
    //Term {AddOperator Term}
    pub fn fun_simple_expression(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //call Term, error if Term was Error
        let mut t = &v[self.counter];
        let mut e: Result<(), MyError> = self.fun_term(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //if no tokens, return ok since no required items left
        if self.no_tokens_left(v.clone()) {
            return Ok(());
        }
        //get next token, check if text was + or - in order to enter while loop
        t = &v[self.counter];
        while t.get_text().eq("+") || t.get_text().eq("-") {
            e = self.fun_add_operator(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //check if tokens left, if not return error because Term needed
            if self.no_tokens_left(v.clone()) {
                self.err_string = "Simple Expression := Term {AddOperator Term}".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //call Term, error if Term was error
            t = &v[self.counter];
            e = self.fun_term(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //if no Tokens remain, ok because no other items needed
            if self.no_tokens_left(v.clone()) {
                return Ok(());
            }
            //else increment counter, stay in while if + or -
            t = &v[self.counter];
        }
        return Ok(());
    }
    // Factor {MultOperator Factor }
    pub fn fun_term(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //call Factor, error if Factor called error
        let mut t = &v[self.counter];
        let mut e: Result<(), MyError> = self.fun_factor(v.clone());
        if e.is_err() {
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
        //return ok if no tokens left since last required token
        if self.no_tokens_left(v.clone()) {
            return Ok(());
        }
        //check if next token is * or / in order to go into loop
        t = &v[self.counter];
        while t.get_text().eq("*") || t.get_text().eq("/") {
            e = self.fun_mult_operator(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //check if tokens left, error if not since Factor needed
            if self.no_tokens_left(v.clone()) {
                self.err_string = "Term := Factor {MultOperator Factor}".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token, call Factor, error if Factor was error
            t = &v[self.counter];
            e = self.fun_factor(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //check if tokens left, ok since no tokens needed afterwards
            if self.no_tokens_left(v.clone()) {
                return Ok(());
            }
            //get next token, if * or /, stay in loop
            t = &v[self.counter];
        }
        Ok(())
    }
    // ( ~(Expression ~) ) | Constant | (/Identifier [ ~( [ Expression {~, Expression}] ~) ] )
    pub fn fun_factor(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        let mut t = &v[self.counter];
        //check if first token is (, if so, call Expression, error if Expression is error
        if t.get_text().eq("(") {
            self.counter += 1;
            let e: Result<(), MyError> = self.fun_expression(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //check if tokens left, if not throw error for )
            if self.no_tokens_left(v.clone()) {
                self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //get next token check if ), error if not
            t = &v[self.counter];
            if !t.get_text().eq(")") {
                self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            //increment counter and ok
            else {
                self.counter += 1;
                Ok(())
            }
        }
        //check if first token is IntConstant or FloatConstant
        else if t.get_token_type() == TokenType::INTCONSTANT
            || t.get_token_type() == TokenType::FLOATCONSTANT
        {
            //call Constant, error if Constant was error
            let e: Result<(), MyError> = self.fun_constant(v.clone());
            if e.is_err() {
                return Err(MyError::General {
                    ln: t.get_line_num(),
                    cn: t.get_char_num(),
                    s: self.err_string.clone(),
                });
            }
            return Ok(());
        }
        //check if first token is Identifier, increment if so
        else if t.get_token_type() == TokenType::IDENTIFIER {
            self.counter += 1;
            //check if tokens remain, ok since no required tokens left
            if self.no_tokens_left(v.clone()) {
                return Ok(());
            }
            //get next token, see if it's (, for ( [ Expression {, Expression}] ), if not ignore
            t = &v[self.counter];
            if t.get_text().eq("(") {
                self.counter += 1;
                //check if tokens left, error since ) needed
                if self.no_tokens_left(v.clone()) {
                    self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                    return Err(MyError::General {
                        ln: t.get_line_num(),
                        cn: t.get_char_num(),
                        s: self.err_string.clone(),
                    });
                }
                //get next token, run expression, if no error if Expression is error since optional
                t = &v[self.counter];
                let mut e: Result<(), MyError> = self.fun_expression(v.clone());
                if e.is_ok() {
                    //if no tokens remain, error beacuse ) needed
                    if self.no_tokens_left(v.clone()) {
                        self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                        return Err(MyError::General {
                            ln: t.get_line_num(),
                            cn: t.get_char_num(),
                            s: self.err_string.clone(),
                        });
                    }
                    //get next token, see if it's , , if so enter while loop
                    t = &v[self.counter];
                    while t.get_text().eq(",") {
                        self.counter += 1;
                        //check if tokens remain, if not error for Expression )
                        if self.no_tokens_left(v.clone()) {
                            self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                            return Err(MyError::General {
                                ln: t.get_line_num(),
                                cn: t.get_char_num(),
                                s: self.err_string.clone(),
                            });
                        }
                        //get next token, call Expression, error if Expression was error
                        t = &v[self.counter];
                        let mut e: Result<(), MyError> = self.fun_expression(v.clone());
                        if e.is_err() {
                            return Err(MyError::General {
                                ln: t.get_line_num(),
                                cn: t.get_char_num(),
                                s: self.err_string.clone(),
                            });
                        }
                        //check if tokens, if not error because of )
                        if self.no_tokens_left(v.clone()) {
                            self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                            return Err(MyError::General {
                                ln: t.get_line_num(),
                                cn: t.get_char_num(),
                                s: self.err_string.clone(),
                            });
                        }
                        //get next token, if , stay in loop
                        t = &v[self.counter];
                    }
                }
                //if token not ), return error
                if !t.get_text().eq(")") {
                    self.err_string = "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
                    return Err(MyError::General {
                        ln: t.get_line_num(),
                        cn: t.get_char_num(),
                        s: self.err_string.clone(),
                    });
                }
                self.counter += 1;
            }

            Ok(())
        }
        //if first token none of the above, return error
        else {
            self.err_string =
                "Factor := ( (Expression ) ) | Constant | (Identifier [ ( [ Expression {, Expression}] ) ] )".to_string();
            return Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            });
        }
    }
    // ( ~== ) | ~< | ~> | ( ~<= ) | ( ~>= ) | ( ~!= )
    pub fn fun_relation_operator(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //if token not of the following, return error
        let mut t = &v[self.counter];
        if !t.get_text().eq("==")
            && !t.get_text().eq("<")
            && !t.get_text().eq(">")
            && !t.get_text().eq("<=")
            && !t.get_text().eq(">=")
            && !t.get_text().eq("!=")
        {
            self.err_string =
                "RelationOperator := ( == ) | < | > | ( <= ) | ( >= ) | ( != )".to_string();
            Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            })
        }
        //increment and ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // + | -
    pub fn fun_add_operator(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //if token not of the following, return error

        let mut t = &v[self.counter];
        if !t.get_text().eq("+") && !t.get_text().eq("-") {
            self.err_string = "AddOperator := +| -".to_string();
            Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            })
        }
        //increment and ok
        else {
            self.counter += 1;
            Ok(())
        }
    }
    // * | /
    pub fn fun_mult_operator(&mut self, v: Vec<Token>) -> Result<(), MyError> {
        //if token not of the following, return error
        let mut t = &v[self.counter];
        if !t.get_text().eq("*") && !t.get_text().eq("/") {
            self.err_string = "MultOperator := * | /".to_string();
            Err(MyError::General {
                ln: t.get_line_num(),
                cn: t.get_char_num(),
                s: self.err_string.clone(),
            })
        }
        //increment and ok
        else {
            self.counter += 1;
            Ok(())
        }
    }

    pub fn no_tokens_left(&self, v: Vec<Token>) -> bool {
        if self.counter >= v.len() {
            true
        } else {
            false
        }
    }

    pub fn tester(&mut self) -> Result<(), MyError> {
        let mut v: Vec<Token> = Vec::new();
        self.scanner.get_all_tokens(&mut v);
        let e = self.fun_program();
        return e;
    }
}
