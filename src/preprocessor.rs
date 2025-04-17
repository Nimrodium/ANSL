// handles preprocessor

use std::fmt::{self, write};

use crate::{
    constant::{CHR_LIT, PREPROCESSOR, PTR, STR_LIT},
    util::CompilerError,
};

#[derive(Debug, Clone)]
pub struct Metadata<'a> {
    file: &'a str,
    line: &'a str,
    line_number: usize,
    line_column: usize,
}
impl<'a> Metadata<'a> {
    pub fn new(file: &'a str, line: &'a str, line_number: usize, line_column: usize) -> Self {
        Self {
            file,
            line,
            line_column,
            line_number,
        }
    }
}
#[derive(Debug, PartialEq)]
enum TokenKind {
    PreProcessorInstruction,
    PreProcessorArgument,
    Comment,
    //enclosures
    ECObrace,
    ECCbrace,
    ECOparen,
    ECCparen,
    ECObracket,
    ECCbracket,
    Delimiter,

    // values
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    FloatLiteral,
    Identifier,
    Pointer,
    // Keywords
    KWfn,
    KWlet,
    KWreturn,
    KWif,
    KWelse,
    KWmatch,
    KWwhile,
    KWfor,
    KWin,
    KWbreak,
    KWcontinue,
    KWstruct,
    KWenum,

    // Operations
    // assignment
    OPassign,
    //std
    OPplus,
    OPsub,
    OPmult,
    OPDiv,
    OPmod,
    //conditional
    OPgreater,
    OPlesser,
    OPgreatereq,
    OPlessereq,
    OPequal,
    OPnotequal,
    //bit
    OPand,
    OPor,
    OPnot,
    OPxor,
    // syntax
    Arrow,
    Colon,
    Dot,
    Comma,
}

impl TokenKind {
    fn identify(lexeme: &str) -> Self {
        let first_char = lexeme.chars().nth(0).unwrap();
        match first_char {
            PREPROCESSOR => return Self::PreProcessorInstruction,
            STR_LIT => return Self::StringLiteral,
            CHR_LIT => return Self::CharLiteral,
            PTR => return Self::Pointer,
            c if c.is_digit(10) => return Self::NumberLiteral,
            _ => match lexeme {
                "//" => Self::Comment,
                "{" => Self::ECObrace,
                "}" => Self::ECCbrace,
                "(" => Self::ECOparen,
                ")" => Self::ECCparen,
                "[" => Self::ECObracket,
                "]" => Self::ECCbracket,
                ";" => Self::Delimiter,
                // keywords
                "fn" => Self::KWfn,
                "let" => Self::KWlet,
                "return" => Self::KWreturn,
                "if" => Self::KWif,
                "else" => Self::KWelse,
                "match" => Self::KWmatch,
                "while" => Self::KWwhile,
                "for" => Self::KWfor,
                "in" => Self::KWin,
                "break" => Self::KWbreak,
                "continue" => Self::KWcontinue,
                "struct" => Self::KWstruct,
                "enum" => Self::KWenum,
                // operators
                "=" => Self::OPassign,
                "+" => Self::OPplus,
                "-" => Self::OPsub,
                "*" => Self::OPmult,
                "/" => Self::OPDiv,
                "%" => Self::OPmod,
                ">" => Self::OPgreater,
                "<" => Self::OPlesser,
                ">=" => Self::OPgreatereq,
                "<=" => Self::OPlessereq,
                "==" => Self::OPequal,
                "!=" => Self::OPnotequal,
                "&&" => Self::OPand,
                "|" => Self::OPor,
                "!" => Self::OPnot,
                "^" => Self::OPxor,
                "->" => Self::Arrow,
                ":" => Self::Colon,
                "." => Self::Dot,
                "," => Self::Comma,
                _ => Self::Identifier,
            },
        }
    }
}
#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    metadata: &'a Metadata<'a>,
}
impl<'a> Token<'a> {
    fn new(lexeme: &'a str, metadata: &'a Metadata<'a>) -> Self {
        Self {
            lexeme,
            metadata,
            kind: TokenKind::identify(lexeme),
        }
    }
}

pub struct TokenStream<'a> {
    inner: Vec<Token<'a>>,
}
impl<'a> TokenStream<'a> {
    fn new(inner: Vec<Token<'a>>) -> Self {
        Self { inner }
    }
}
impl<'a> fmt::Debug for TokenStream<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

enum CompilationRules {}

/// parses preprocessor and outputs the modified source file and compilation rules
// pub fn preprocessor_engine(&str) -> Result<(str,Vec<CompilationRules>),CompilerError>{
//     todo!()
// }
/// tokenize source code
pub fn tokenize<'a>(source_file: &'a Vec<Vec<(String, Metadata)>>) -> TokenStream<'a> {
    let mut stream: Vec<Token> = Vec::new();
    let mut skip_comment = false;
    let mut inside_preprocessor = false;
    for line in source_file {
        for (lexeme, metadata) in line {
            // let token = {
            //     let tmp_token = Token::new(&lexeme, metadata);
            //     if inside_preprocessor {
            //         if tmp_token.kind == TokenKind::ECCbrace {
            //             inside_preprocessor = false;
            //             tmp_token
            //         } else {
            //             if tmp_token.kind != TokenKind::ECObrace {
            //                 Token {
            //                     kind: TokenKind::PreProcessorArgument,
            //                     lexeme,
            //                     metadata,
            //                 }
            //             } else {
            //                 tmp_token
            //             }
            //         }
            //     } else {
            //         tmp_token
            //     }
            // };
            let token = {
                let tmp_token = Token::new(&lexeme, metadata);
                if inside_preprocessor {
                    if tmp_token.kind == TokenKind::Identifier {
                        Token {
                            kind: TokenKind::PreProcessorArgument,
                            lexeme,
                            metadata,
                        }
                    } else {
                        tmp_token
                    }
                } else {
                    tmp_token
                }
            };

            if token.kind == TokenKind::Comment {
                // println!("comment");
                skip_comment = true;
            }
            if token.kind == TokenKind::PreProcessorInstruction {
                inside_preprocessor = true;
            }
            if !skip_comment {
                println!("{token:?}");
                stream.push(token);
            } else {
                // println!("refusing to push {:?}", token)
            }
        }
        skip_comment = false;
    }
    TokenStream::new(stream)
}
