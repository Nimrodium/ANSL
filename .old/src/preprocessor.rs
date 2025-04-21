// handles preprocessor
use string_interner::{DefaultStringInterner, DefaultSymbol, Symbol};

use std::{
    collections::HashMap,
    fmt::{self},
    fs::File,
    io::read_to_string,
};

use crate::{
    constant::{
        CHR_LIT, DELIMITER, IGNORE_PATTERN, PREPROCESSOR, PTR, SPECIAL_CHARS, SPLIT_PATTERN,
        STR_LIT, SYSTEM_LIB,
    },
    util::{lex_file, CompilerError, LexxedSource},
};

#[derive(Debug, Clone)]
pub struct Metadata {
    file: DefaultSymbol,
    line: DefaultSymbol,
    line_number: usize,
    line_column: usize,
}
impl Metadata {
    pub fn new(
        file: DefaultSymbol,
        line: DefaultSymbol,
        line_number: usize,
        line_column: usize,
    ) -> Self {
        Self {
            file,
            line,
            line_column,
            line_number,
        }
    }
}
pub struct OwnedMetadata {
    file: String,
    line: String,
    line_number: usize,
    line_column: usize,
}
impl OwnedMetadata {
    pub fn new(file: &str, line: &str, line_number: usize, line_column: usize) -> Self {
        Self {
            file: file.to_string(),
            line: line.to_string(),
            line_number,
            line_column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: DefaultSymbol,
    metadata: Metadata,
}
impl Token {
    fn new(kind: TokenKind, lexeme: DefaultSymbol, metadata: Metadata) -> Self {
        Self {
            lexeme,
            metadata,
            kind,
        }
    }
}
pub struct TokenStream {
    tokens: Vec<Token>,
    str_db: DefaultStringInterner,
    conditional_condition: bool,
    // preprocessor_variables: HashMap<String, usize>,
}
impl TokenStream {
    fn new(str_db: DefaultStringInterner) -> Self {
        Self {
            tokens: Vec::new(),
            str_db,
            conditional_condition: true,
        }
    }
    fn new_token(
        &mut self,
        lexeme: DefaultSymbol,
        metadata: Metadata,
    ) -> Result<&mut Token, CompilerError> {
        let deref_lex = if let Some(d) = self.str_db.resolve(lexeme) {
            d
        } else {
            return Err(CompilerError::new(&format!("lexeme dereference failed")));
        };
        let kind = TokenKind::identify(deref_lex);
        let token = Token::new(kind, lexeme, metadata);
        if self.conditional_condition {
            self.tokens.push(token.clone());
        }
        Ok(self.tokens.last_mut().unwrap())
    }
    /// falls back if dereference fails
    fn dereference_metadata_fallback(&self, metadata: &Metadata) -> OwnedMetadata {
        let file = self.str_db.resolve(metadata.file).unwrap_or("CORRUPT");
        let line = self.str_db.resolve(metadata.line).unwrap_or("CORRUPT");
        OwnedMetadata::new(file, line, metadata.line_number, metadata.line_column)
    }
    /// returns Err if dereference fails
    fn dereference_metadata(&self, metadata: &Metadata) -> Result<OwnedMetadata, CompilerError> {
        let file = self
            .str_db
            .resolve(metadata.file)
            .ok_or(CompilerError::new(
                "metadata file reference could not be resolved",
            ))?;
        let line = self
            .str_db
            .resolve(metadata.line)
            .ok_or(CompilerError::new(
                "metadata file reference could not be resolved",
            ))?;
        Ok(OwnedMetadata::new(
            file,
            line,
            metadata.line_number,
            metadata.line_column,
        ))
    }
    fn dereference_string(&self, symbol: DefaultSymbol) -> Result<&str, CompilerError> {
        self.str_db
            .resolve(symbol)
            .ok_or(CompilerError::new("String Dereference failed"))
    }
    fn dereference_string_fallback(&self, symbol: DefaultSymbol) -> &str {
        self.str_db.resolve(symbol).unwrap_or("CORRUPT")
    }

    fn include_file(&mut self, path: &str) -> Result<(), CompilerError> {
        let token_stream = tokenize_file(path, self.str_db.clone())?;
        self.merge_tokenstream(token_stream);
        Ok(())
    }

    fn merge_tokenstream(&mut self, token_stream: TokenStream) {
        self.str_db = token_stream.str_db;
        self.tokens.extend_from_slice(&token_stream.tokens);
    }

    fn execute_preprocesor_command(&mut self, cmd: &[String]) -> Result<(), CompilerError> {
        println!("executing {:?}", cmd);
        let command = if let Some(c) = cmd.get(0) {
            c
        } else {
            return Err(CompilerError::new("preprocessor command missing"));
        };

        match command.as_str() {
            "#include" => {
                let location = if let Some(c) = cmd.get(1) {
                    c
                } else {
                    return Err(CompilerError::new("#include <location> missing"));
                };
                let name = if let Some(c) = cmd.get(2) {
                    c
                } else {
                    return Err(CompilerError::new(&format!(
                        "#include {location} <name> missing"
                    )));
                };

                let included_file_path = match location.as_str() {
                    "system" => (SYSTEM_LIB.to_string() + name) + ".ansl",
                    "module" => ("./".to_string() + name) + ".ansl",
                    "absoulute" => name.to_string(),
                    _ => {
                        return Err(CompilerError::new(&format!(
                            "invalid include argument {}",
                            location
                        )))
                    }
                };

                self.include_file(&included_file_path)?;
            }
            "#if" => todo!(),
            "#else" => todo!(),
            "#fi" => self.conditional_condition = true,
            _ => return Err(CompilerError::new("invalid command")),
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum PreprocessorCommand {
    None,
    Include,
    Define,
    If,
    Else,
    Fi,
}

impl fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}
impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for token in &self.tokens {
            let dereferenced_lexeme = self.dereference_string_fallback(token.lexeme);
            let dereferenced_metadata = self.dereference_metadata_fallback(&token.metadata);
            let composed = format!(
                "[{:?} : \"{dereferenced_lexeme}\" ({}:{})]\n",
                token.kind, dereferenced_metadata.line_number, dereferenced_metadata.line_column
            );
            s.push_str(composed.as_str());
        }
        write!(f, "{s}")
    }
}
enum CompilationRules {}

pub fn tokenize_(source_file: LexxedSource) -> Result<TokenStream, CompilerError> {
    let mut token_stream: TokenStream = TokenStream::new(source_file.str_db);
    // let mut skip_comment = false;
    let mut inside_preprocessor = false;
    for line in source_file.inner {
        for (lexeme, metadata) in line {
            let token = token_stream.new_token(lexeme, metadata)?;
            if inside_preprocessor {
                if token.kind == TokenKind::Identifier {
                    token.kind = TokenKind::PreProcessorArgument;
                } else if token.kind == TokenKind::ECCbrace {
                    inside_preprocessor = false;
                }
            }
            if token.kind == TokenKind::PreProcessorInstruction {
                inside_preprocessor = true;
            }
        }
    }
    Ok(token_stream)
}

pub fn tokenize(source_file: LexxedSource) -> Result<TokenStream, CompilerError> {
    let mut token_stream = TokenStream::new(source_file.str_db.clone());
    let mut collect_preprocessor = false;
    let mut preprocessor_command: Vec<String> = Vec::new();
    for line in source_file.inner {
        for (lexeme, metadata) in line {
            let dereferenced_lexeme = if let Some(s) = source_file.str_db.resolve(lexeme) {
                s
            } else {
                return Err(CompilerError::new("string dereference failed"));
            };
            if dereferenced_lexeme.chars().nth(0).unwrap() == '#' {
                collect_preprocessor = true;
            }
            if !collect_preprocessor {
                let token = token_stream.new_token(lexeme, metadata)?;
            } else {
                if dereferenced_lexeme != DELIMITER.to_string() {
                    preprocessor_command.push(dereferenced_lexeme.to_string());
                } else {
                    collect_preprocessor = false;
                    token_stream.execute_preprocesor_command(&preprocessor_command)?;
                }
            }
        }
    }
    Ok(token_stream)
}

pub fn tokenize_file(
    path: &str,
    str_db: DefaultStringInterner,
) -> Result<TokenStream, CompilerError> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(CompilerError::new(&format!("{err}"))),
    };
    let contents = match read_to_string(file) {
        Ok(s) => s,
        Err(err) => return Err(CompilerError::new(&format!("{err}"))),
    };

    // let contents: &'static str = Box::leak(contents_owned.into_boxed_str());

    let lexxed: LexxedSource = lex_file(
        &contents,
        path,
        IGNORE_PATTERN,
        SPLIT_PATTERN,
        SPECIAL_CHARS,
        str_db,
    );
    tokenize(lexxed)
}
