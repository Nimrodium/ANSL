use crate::token::{CompileError, MetadataReference, Source, Token, TokenKind, TokenStream};

#[derive(Debug)]
pub struct BinaryOP {
    left: Box<ASTnode>,
    right: Box<ASTnode>,
    metadata: MetadataReference,
}
#[derive(Debug)]
pub enum ASTnode {
    Root {
        globals: Vec<ASTnode>,
        functions: Vec<ASTnode>,
    },
    Type {
        name: String,
        metadata: MetadataReference,
    },
    Variable {
        name: String,
        kind: Box<ASTnode>,
        metadata: MetadataReference,
    },
    NumberLiteral(usize),
    StringLiteral(String),
    ArrayLiteral(Vec<ASTnode>),
    TupleLiteral(Vec<ASTnode>),
    FunctionCall {
        name: String,
        params: Vec<ASTnode>,
        metadata: MetadataReference,
    },
    Block {
        nodes: Vec<ASTnode>,
    },
    Assign {
        dest: Box<ASTnode>,
        expr: Box<ASTnode>,
        metadata: MetadataReference,
    },
    VariableDeclaration {
        name: String,
        kind: Box<ASTnode>,
        metadata: MetadataReference,
    },
    FunctionDefinition {
        name: String,
        params: Vec<ASTnode>,
        body: Box<ASTnode>,
        metadata: MetadataReference,
    },

    Add(BinaryOP),
    Sub(BinaryOP),
    Mult(BinaryOP),
    Div(BinaryOP),
    Mod(BinaryOP),

    And(BinaryOP),
    Or(BinaryOP),
    Xor(BinaryOP),
    Not {
        value: Box<ASTnode>,
        metadata: MetadataReference,
    },
}

pub fn root_parse(mut token_stream: TokenStream) -> Result<ASTnode, CompileError> {
    // let x : u8 = 1 + 2;
    let mut functions: Vec<ASTnode> = Vec::new();
    let mut globals: Vec<ASTnode> = Vec::new();
    while !token_stream.eof() {
        let token = token_stream.next().unwrap();
        let node: ASTnode = match token.kind {
            TokenKind::KWfn => parse_function(&mut token_stream)?,
            TokenKind::KWconst => parse_const(&mut token_stream)?,
            TokenKind::KWstatic => parse_static(&mut token_stream)?,
            _ => {
                return Err(CompileError::new(format!(
                    "statement `{}` not allowed in root namespace",
                    token.lexeme
                ))
                .attach_token(token)
                .fmt_metadata(&token_stream.source))
            }
        };
        match node {
            ASTnode::FunctionDefinition { .. } => functions.push(node),
            _ => globals.push(node),
        }
    }
    Ok(ASTnode::Root { globals, functions })
}
fn parse_function(token_stream: &mut TokenStream) -> Result<ASTnode, CompileError> {
    todo!("function")
}
fn parse_const(token_stream: &mut TokenStream) -> Result<ASTnode, CompileError> {
    enum ConstState {
        Initial,
        VariableName(Token),
        Colon(Token),
        Type(ASTnode),
    }
    let mut state = ConstState::Initial;
    while !token_stream.peek_is(TokenKind::SemiColon) {
        let token = token_stream.next().unwrap();
        println!("{}", token);
        state = match state {
            ConstState::Initial => {
                if token.is(TokenKind::Identifer) {
                    ConstState::VariableName(token)
                } else {
                    return Err(CompileError::new("expected variable name".to_string())
                        .attach_token(token)
                        .fmt_metadata(&token_stream.source));
                }
            }
            ConstState::VariableName(variable) => {
                if token.is(TokenKind::Colon) {
                    ConstState::Colon(variable)
                } else {
                    return Err(CompileError::new("expected colon".to_string())
                        .attach_token(token)
                        .fmt_metadata(&token_stream.source));
                }
            }
            ConstState::Colon(variable) => match token.kind {
                TokenKind::Primitive(kind) => ConstState::Type(ASTnode::VariableDeclaration {
                    name: variable.lexeme,
                    kind: Box::new(ASTnode::Type {
                        name: kind,
                        metadata: token.metadata,
                    }),
                    metadata: variable.metadata,
                }),
                _ => {
                    return Err(CompileError::new("expected primitive type".to_string())
                        .attach_token(token)
                        .fmt_metadata(&token_stream.source))
                }
            },
            ConstState::Type(left) => {
                if token.is(TokenKind::Assign) {
                    let expr = parse_expr(token_stream)?;
                    let return_node = ASTnode::Assign {
                        dest: Box::new(left),
                        expr: Box::new(expr),
                        metadata: token.metadata,
                    };
                    return Ok(return_node);
                } else {
                    return Err(CompileError::new("expected equal sign".to_string())
                        .attach_token(token)
                        .fmt_metadata(&token_stream.source));
                }
            }
        };

        // if token.is(TokenKind::Identifer) {
        // } else {
        //     return Err(CompileError::new("expected variable name".to_string())
        //         .attach_token(token)
        //         .fmt_metadata(&token_stream.source));
        // }
    }
    todo!("const")
}
fn parse_static(token_stream: &mut TokenStream) -> Result<ASTnode, CompileError> {
    todo!("static")
}
fn parse_variable_declaration(token_stream: &mut TokenStream) -> Result<ASTnode, CompileError> {
    todo!("variable_declaration")
}
fn parse_expr(token_stream: &mut TokenStream) -> Result<ASTnode, CompileError> {
    todo!("expr")
}
