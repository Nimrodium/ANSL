use std::{
    collections::{HashMap, VecDeque},
    fmt,
    fs::File,
    io::read_to_string,
};

use colorize::AnsiColor;

use crate::{
    constant::{COMMENT_CHAR, SOURCE_FILE_EXTENSION, SYSTEM_LIB_ROOT},
    verbose_println, very_verbose_println, very_very_verbose_println,
};

pub struct CompileError {
    error: String,
    metadata: Option<MetadataReference>,
    token: Option<Token>,
    dereferenced_metadata_str: Option<String>,
}
impl CompileError {
    pub fn new(error: String) -> Self {
        Self {
            error,
            metadata: None,
            token: None,
            dereferenced_metadata_str: None,
        }
    }
    pub fn attach_metadata(mut self, metadata: MetadataReference) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn attach_token(mut self, token: Token) -> Self {
        self.token = Some(token);
        self
    }
    pub fn fmt_metadata(mut self, source: &Source) -> Self {
        very_very_verbose_println!("formatting incoming error");
        if self.dereferenced_metadata_str.is_some() {
            // avoid applying if already fmt
            very_very_verbose_println!("metadata already formatted");
            return self;
        }

        let metadata = self
            .metadata
            .clone()
            .or_else(|| self.token.clone().and_then(|t| Some(t.metadata)));

        if let Some(m) = metadata {
            let line = source.get_line(&m.file_name, m.line_number);
            let lexeme_len = if let Some(t) = &self.token {
                t.lexeme.len()
            } else {
                1
            };
            let highlight = {
                let mut s = String::new();
                for _ in 0..m.column {
                    s.push(' ');
                }
                s.push('^');
                for _ in 1..lexeme_len {
                    s.push('~');
                }
                s
            };
            if let Some(l) = line {
                very_very_verbose_println!("line: {l}\nhighlight: {highlight}");
                self.dereferenced_metadata_str = Some(format!("{l}\n\t\t{highlight}"))
            } else {
                very_very_verbose_println!("could not find line {}", m.line_number);
            }
        }
        self
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut header = format!("Error in compilation: {}", self.error);
        if let Some(t) = &self.token {
            header += format!(" >> {t} :").as_str();
        }
        let metadata = self
            .metadata
            .clone()
            .or_else(|| self.token.clone().and_then(|t| Some(t.metadata)));
        let body = if let Some(m) = metadata {
            let mut b = format!("at {}:{} in file {}", m.line_number, m.column, m.file_name);
            if let Some(deref_met) = &self.dereferenced_metadata_str {
                b += format!(":\n\t\t {}", deref_met).as_str();
            }
            b
        } else {
            String::new()
        };

        // let sub = if let Some(fmt_str) = &self.dereferenced_metadata_str {
        //     &format!("at ")
        // };
        write!(f, "{header}\n\t{body}")
    }
}
#[derive(Clone)]
pub struct MetadataReference {
    file_name: String,
    line_number: usize,
    column: usize,
}
impl MetadataReference {
    fn new(file_name: &str, line_number: usize, column: usize) -> Self {
        Self {
            file_name: file_name.to_string(),
            line_number,
            column,
        }
    }
}
impl fmt::Debug for MetadataReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}:{}]",
            self.file_name, self.line_number, self.column
        )
    }
}
impl Default for MetadataReference {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            line_number: 1,
            column: 1,
        }
    }
}

struct SourceFile {
    file_name: String,
    source: Vec<String>,
}
impl SourceFile {
    fn new(file_name: &str) -> Result<Self, CompileError> {
        let f = File::open(file_name)
            .map_err(|err| CompileError::new(format!("could not open {file_name} :: {err}")))?;
        let file_contents = read_to_string(f)
            .map_err(|err| CompileError::new(format!("could not read {file_name} :: {err}")))?;
        let source = file_contents.lines().map(|s| s.to_string()).collect();
        Ok(Self {
            file_name: file_name.to_string(),
            source,
        })
    }

    fn get_line(&self, line_number: usize) -> Option<&String> {
        println!("{line_number}");
        self.source.get(line_number - 1)
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<source file :: {}>", self.file_name)
    }
}
pub struct Source {
    sources: HashMap<String, SourceFile>,
}
impl Source {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }
    fn open_file(&mut self, path: &str) -> Result<&SourceFile, CompileError> {
        let src = SourceFile::new(path)?;
        self.sources.insert(path.to_string(), src);
        Ok(&self.sources[path])
    }
    fn add_file(&mut self, file: SourceFile) {
        self.sources.insert(file.file_name.clone(), file);
    }
    fn merge(&mut self, other: Self) {
        for (k, v) in other.sources {
            self.sources.insert(k, v);
        }
    }
    fn get_line(&self, file_name: &str, line_n: usize) -> Option<&String> {
        let file = if let Some(f) = self.sources.get(file_name) {
            f
        } else {
            very_very_verbose_println!("file {file_name} doesnt exist");
            println!("{:?}", self.sources);
            return None;
        };
        file.get_line(line_n)
    }
}
#[derive(Debug, Clone, PartialEq)]

pub enum PrimitiveType {
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,

    Signed8,
    Signed16,
    Signed32,
    Signed64,

    Float32,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //preprocessor
    Include,

    StringLiteral,
    NumberLiteral,
    Identifer,

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
    KWconst,
    KWstatic,
    Primitive(String),

    Slash,
    Plus,
    Star,
    Dash,
    And,
    Carot,
    Percent,
    Dollar,
    At,
    Exclaim,
    Assign,

    Colon,
    SemiColon,
    OpenCurly,
    ClosedCurly,
    OpenBracket,
    ClosedBracket,
    OpenParenth,
    ClosedParenth,
    Comma,
    Dot,
    Apostro,
    Greater,
    Lesser,

    Compare,
    GreaterEq,
    LesserEq,
    DoublePipe,
    Arrow,

    Pipe,
    BackTick,
    Grave,

    EOF,
}
impl TokenKind {
    fn match_keyword(s: &str) -> Option<Self> {
        match s {
            "fn" => Some(Self::KWfn),
            "let" => Some(Self::KWlet),
            "return" => Some(Self::KWreturn),
            "if" => Some(Self::KWif),
            "else" => Some(Self::KWelse),
            "match" => Some(Self::KWmatch),
            "while" => Some(Self::KWwhile),
            "for" => Some(Self::KWfor),
            "in" => Some(Self::KWin),
            "break" => Some(Self::KWbreak),
            "continue" => Some(Self::KWcontinue),
            "struct" => Some(Self::KWstruct),
            "enum" => Some(Self::KWenum),
            "const" => Some(Self::KWconst),
            "static" => Some(Self::KWstatic),
            "u8" => Some(Self::Primitive(s.to_string())),
            "u16" => Some(Self::Primitive(s.to_string())),
            "u32" => Some(Self::Primitive(s.to_string())),
            "u64" => Some(Self::Primitive(s.to_string())),
            "i8" => Some(Self::Primitive(s.to_string())),
            "i16" => Some(Self::Primitive(s.to_string())),
            "i32" => Some(Self::Primitive(s.to_string())),
            "i64" => Some(Self::Primitive(s.to_string())),
            "f32" => Some(Self::Primitive(s.to_string())),

            "include" => Some(Self::Include),
            _ => None,
        }
    }
}
#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub metadata: MetadataReference,
}
impl Token {
    fn new(kind: TokenKind, lexeme: &str, metadata: MetadataReference) -> Self {
        Self {
            kind,
            lexeme: lexeme.to_string(),
            metadata,
        }
    }
    /// returns EOF sentinel token
    fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            lexeme: <String as std::default::Default>::default(),
            metadata: MetadataReference::default(),
        }
    }
    // fn dummy() -> Self {
    //     Self {
    //         kind: TokenKind::Dummy,
    //         lexeme: String::new(),
    //         metadata: MetadataReference::new(String::new().as_str(), 0, 0),
    //     }
    // }
    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<{:?} `{}` [{}:{}]>",
            self.kind, self.lexeme, self.metadata.line_number, self.metadata.column
        )
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}
#[derive(Debug)]
enum State {
    Inital, // initial state, enters after building word
    // PreProcessor,
    PreComment,
    Comment,

    DoubleCharToken(char),
    BuildingIdentifier,
    BuildingString,
    EndString,
    StrEsc,
}
enum Stream {
    Master,
    Preprocessor,
}

pub struct TokenStream {
    tokens: VecDeque<Token>,

    pub source: Source,
}
impl TokenStream {
    pub fn new() -> Self {
        Self {
            tokens: VecDeque::new(),

            source: Source::new(),
        }
    }
    pub fn tokenize_source_tree(&mut self, file_path: &str) -> Result<(), CompileError> {
        very_verbose_println!("entry file : <{file_path}>");
        // self.source.open_file(file_path)?;
        let mut tokenizer = Tokenizer::new();
        let main = tokenizer.sources.open_file(file_path)?;
        for (n, line) in main.source.clone().iter().enumerate() {
            let token_stream = tokenizer
                .parse_line(file_path, line, n + 1)
                .map_err(|e| e.fmt_metadata(&self.source))?;
            self.tokens.extend(token_stream);
        }
        self.source.merge(tokenizer.sources);
        self.tokens.push_back(Token::eof());
        Ok(())
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(0)
    }
    pub fn peek_is(&self, kind: TokenKind) -> bool {
        if let Some(t) = self.peek() {
            t.kind == kind
        } else {
            false
        }
    }

    pub fn eof(&self) -> bool {
        self.peek_is(TokenKind::EOF)
    }
}

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for token in &self.tokens {
            s.push_str((token.to_string() + "\n").as_str());
        }
        write!(f, "{s}")
    }
}

enum Location {
    System,
    Module,
    Absolute,
    None,
}

struct Tokenizer {
    state: State,
    sources: Source,
    active_lexeme: String,
    active_character: (usize, char),
    lexeme_column_start: usize,
    active_stream: Stream,
    token_stream: Vec<Token>,
    preprocessor_stream: Vec<Token>,
}

impl Tokenizer {
    fn return_token_stream(&mut self) -> Vec<Token> {
        std::mem::take(&mut self.token_stream)
    }
    fn push_token(&mut self, token: Token) {
        // very_very_verbose_println!("pushing token {token}");
        match self.active_stream {
            Stream::Master => self.token_stream.push(token),
            Stream::Preprocessor => self.preprocessor_stream.push(token),
        }
        self.active_lexeme.clear();
        self.state = State::Inital;
    }
    fn include_file(&mut self, file: &str) -> Result<(), CompileError> {
        verbose_println!("including file {}", file);
        let mut tokenizer = Tokenizer::new();
        let sub_file = tokenizer.sources.open_file(file)?;
        for (n, line) in sub_file.source.clone().iter().enumerate() {
            let token_stream = tokenizer
                .parse_line(file, line, n + 1)
                .map_err(|e| e.fmt_metadata(&tokenizer.sources))?;
            self.token_stream.extend(token_stream);
        }
        self.sources.merge(tokenizer.sources);
        Ok(())
    }
    fn execute_preprocessor(&mut self) -> Result<(), CompileError> {
        very_verbose_println!("executing {:?}", self.preprocessor_stream);
        enum Command {
            Include(Location),
            If,
            Else,
            None,
        }
        let mut command = Command::None;

        for token in &self.preprocessor_stream.clone() {
            match command {
                Command::None => match token.kind {
                    TokenKind::Include => command = Command::Include(Location::None),
                    TokenKind::KWif => command = Command::If,
                    TokenKind::KWelse => command = Command::Else,
                    _ => {
                        return Err(CompileError::new(format!(
                            "unexpected token in preprocessor command"
                        ))
                        .attach_token(token.clone()))
                    }
                },
                Command::Include(ref mut location) => match location {
                    Location::None => match token.lexeme.as_str() {
                        "system" => *location = Location::System,
                        "module" => *location = Location::Module,
                        "absolute" => *location = Location::Absolute,
                        _ => {
                            return Err(CompileError::new(
                                "include subword not recognized".to_string(),
                            )
                            .attach_token(token.clone()))
                        }
                    },
                    Location::System => {
                        let include_path = SYSTEM_LIB_ROOT.to_owned()
                            + token.lexeme.as_str()
                            + SOURCE_FILE_EXTENSION;
                        self.include_file(&include_path)
                            .map_err(|e| e.attach_token(token.clone()))?;
                    }
                    Location::Module => {
                        todo!("module not implemented yet")
                    }

                    Location::Absolute => {
                        self.include_file(&token.lexeme)?;
                    }
                },
                Command::If => {
                    todo!("if not implemented yet")
                }
                Command::Else => {
                    todo!("else not implemented yet")
                }
            }
        }
        self.preprocessor_stream.clear();
        Ok(())
    }

    fn new() -> Self {
        Self {
            state: State::Inital,
            active_lexeme: String::new(),
            active_character: (0, '\0'),
            lexeme_column_start: 0,
            active_stream: Stream::Master,
            token_stream: Vec::new(),
            preprocessor_stream: Vec::new(),
            sources: Source::new(),
        }
    }
    fn parse_line(
        &mut self,
        file_name: &str,
        line: &str,
        line_n: usize,
    ) -> Result<Vec<Token>, CompileError> {
        very_very_verbose_println!(">> tokenizing ::{line_n}:: {line}");
        let mut advance = true;
        let mut line_iter = line.chars().enumerate();
        match self.state {
            State::Comment => self.state = State::Inital,
            _ => (),
        }
        loop {
            // very_very_verbose_println!("STATE : {:?}", self.state);
            if advance {
                if let Some(c) = line_iter.next() {
                    self.active_character = c;
                } else {
                    break;
                }
            } else {
                advance = true;
            }
            let chr = self.active_character.1;
            match self.state {
                State::Inital => {
                    self.lexeme_column_start = self.active_character.0 + 1;
                    match chr {
                        '#' => self.active_stream = Stream::Preprocessor,
                        '/' => self.state = State::PreComment,
                        '"' => self.state = State::BuildingString,
                        '{' => {
                            let token = Token::new(
                                TokenKind::OpenCurly,
                                "{",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '}' => {
                            let token = Token::new(
                                TokenKind::ClosedCurly,
                                "}",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '(' => {
                            let token = Token::new(
                                TokenKind::OpenParenth,
                                "(",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        ')' => {
                            let token = Token::new(
                                TokenKind::ClosedParenth,
                                ")",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '[' => {
                            let token = Token::new(
                                TokenKind::OpenBracket,
                                "[",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        ']' => {
                            let token = Token::new(
                                TokenKind::ClosedBracket,
                                "]",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        ',' => {
                            let token = Token::new(
                                TokenKind::Comma,
                                ",",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '.' => {
                            let token = Token::new(
                                TokenKind::Dot,
                                ".",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '+' => {
                            let token = Token::new(
                                TokenKind::Plus,
                                "+",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '*' => {
                            let token = Token::new(
                                TokenKind::Star,
                                "*",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        ':' => {
                            let token = Token::new(
                                TokenKind::Colon,
                                ":",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        ';' => {
                            // self.active_stream = Stream::Master;
                            match self.active_stream {
                                Stream::Preprocessor => {
                                    self.active_stream = Stream::Master;
                                    self.execute_preprocessor()
                                        .map_err(|e| e.fmt_metadata(&self.sources))?;
                                }
                                Stream::Master => {
                                    let token = Token::new(
                                        TokenKind::SemiColon,
                                        ";",
                                        MetadataReference::new(
                                            file_name,
                                            line_n,
                                            self.lexeme_column_start,
                                        ),
                                    );
                                    self.push_token(token);
                                }
                            }
                        }
                        '&' => {
                            let token = Token::new(
                                TokenKind::And,
                                "&",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '^' => {
                            let token = Token::new(
                                TokenKind::Carot,
                                "^",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '%' => {
                            let token = Token::new(
                                TokenKind::Percent,
                                "%",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '$' => {
                            let token = Token::new(
                                TokenKind::Dollar,
                                "$",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '@' => {
                            let token = Token::new(
                                TokenKind::At,
                                "@",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '!' => {
                            let token = Token::new(
                                TokenKind::Exclaim,
                                "!",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '`' => {
                            let token = Token::new(
                                TokenKind::BackTick,
                                "`",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '~' => {
                            let token = Token::new(
                                TokenKind::Grave,
                                "~",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }
                        '\'' => {
                            let token = Token::new(
                                TokenKind::Apostro,
                                "&",
                                MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                            );
                            self.push_token(token);
                        }

                        '<' | '>' | '=' | '|' | '-' => self.state = State::DoubleCharToken(chr),
                        ' ' | '\t' => continue,
                        _ => {
                            self.state = State::BuildingIdentifier;
                            self.active_lexeme.push(chr);
                        }
                    }
                }

                State::DoubleCharToken(ch2) => {
                    let kind = match chr {
                        '=' => match ch2 {
                            '<' => TokenKind::LesserEq,
                            '>' => TokenKind::GreaterEq,
                            '=' => TokenKind::Compare,
                            '|' => TokenKind::DoublePipe,
                            '-' => {
                                advance = false;
                                TokenKind::Dash
                            }
                            // '-' => TokenKind::EqDash,
                            // '+' => TokenKind::EqPlus,
                            // '*' => TokenKind::
                            _ => unreachable!(),
                        },

                        '>' => match ch2 {
                            '-' => TokenKind::Arrow,
                            _ => unreachable!(),
                        },

                        _ => {
                            advance = false;
                            match ch2 {
                                '<' => TokenKind::Lesser,
                                '>' => TokenKind::Greater,
                                '=' => TokenKind::Assign,
                                '|' => TokenKind::Pipe,
                                '-' => TokenKind::Dash,
                                _ => unreachable!(),
                            }
                        }
                    };
                    let token = Token::new(
                        kind,
                        ch2.to_string().as_str(),
                        MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                    );
                    self.push_token(token);
                }

                State::PreComment => match chr {
                    '/' => self.state = State::Comment,
                    _ => {
                        advance = false;
                        let token = Token::new(
                            TokenKind::Slash,
                            &self.active_lexeme,
                            MetadataReference::new(file_name, line_n, self.lexeme_column_start),
                        );
                        self.push_token(token);
                    }
                },
                State::Comment => break,
                State::BuildingString => match chr {
                    '"' => self.state = State::EndString,
                    '\\' => self.state = State::StrEsc,
                    _ => self.active_lexeme.push(chr),
                },
                State::StrEsc => {
                    let esc_char = match chr {
                        '\\' => '\\',
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\'' => '\'',
                        '"' => '"',
                        '0' => '\0',
                        'b' => '\x08',
                        _ => {
                            return Err(CompileError::new(format!(
                                "invalid escape sequence >>\"\\{chr}\"<< "
                            )))
                        }
                    };
                    self.active_lexeme.push(esc_char);
                    self.state = State::BuildingString;
                }
                State::BuildingIdentifier => match chr {
                    '_' => self.active_lexeme.push(chr),
                    c if c.is_alphanumeric() && c != ' ' && c != '\t' => {
                        self.active_lexeme.push(chr)
                    }
                    _ => {
                        advance = false;
                        let kind = match self.active_lexeme.chars().nth(0).unwrap() {
                            c if c.is_digit(10) => TokenKind::NumberLiteral,
                            _ => match TokenKind::match_keyword(&self.active_lexeme.trim()) {
                                Some(k) => k,
                                None => TokenKind::Identifer,
                            },
                        };

                        let token_metadata =
                            MetadataReference::new(file_name, line_n, self.lexeme_column_start);
                        let token = Token::new(kind, &self.active_lexeme, token_metadata);
                        self.push_token(token);
                    }
                },
                State::EndString => {
                    let token_metadata =
                        MetadataReference::new(file_name, line_n, self.lexeme_column_start);
                    let token = Token::new(
                        TokenKind::StringLiteral,
                        &self.active_lexeme,
                        token_metadata,
                    );
                    self.push_token(token);
                    self.active_lexeme.clear();
                }
            }
        }
        Ok(self.return_token_stream())
    }
}
