#![allow(dead_code)]
use crate::lexer::{AdjKind, AdverbKind, Lexer, NounKind, Token, TokenKind, VerbKind};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

#[derive(Debug, Clone)]
pub enum GameExpr {
    Svn {
        verb: VerbKind,
        noun: NounKind,
        adverb: Option<AdverbKind>,
        adj: Option<AdjKind>,
    },
    Raw(String),
    Confirm(bool),
}

#[derive(Debug, Clone)]
pub struct ProgramExpr {
    verb: VerbKind,
    noun: NounKind,
    adverb: Option<AdverbKind>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Game(GameExpr),
    Program(ProgramExpr),
    Confirm(bool),
    Help,
    Hint,
}

#[derive(Error, Debug, Clone)]
pub enum ParseErr {
    #[error("Unexpected token")]
    Unexpected((TokenKind, TokenKind)),

    #[error("Missing verb")]
    MissingVerb,

    #[error("Missing noun")]
    MissingNoun,

    #[error("Bad Grammar; make sure it's in Subject Object Verb form")]
    BadGrammar,

    #[error("Internal unimplemented")]
    Unimplemented,
}

type Result<T> = std::result::Result<T, ParseErr>;

fn is_exit_game(verb: VerbKind, noun: NounKind) -> bool {
    matches!(verb, VerbKind::Close | VerbKind::Quit | VerbKind::Exit)
        && matches!(noun, NounKind::Game)
}

impl ProgramExpr {
    pub fn verb(&self) -> VerbKind {
        self.verb
    }

    pub fn noun(&self) -> NounKind {
        self.noun
    }
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Lexer::new(src),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<TokenKind> {
        let token = self.lexer.next_token();
        if token.kind() == kind {
            Ok(token.kind())
        } else {
            Err(ParseErr::Unexpected((kind, token.kind())))
        }
    }

    fn expect_noun(&mut self) -> Result<NounKind> {
        let token = self.eat_articles();
        match token.kind() {
            TokenKind::Noun(noun) => Ok(noun),
            _ => Err(ParseErr::Unexpected((NounKind::Dummy.into(), token.kind()))),
        }
    }

    fn expect_verb(&mut self) -> Result<VerbKind> {
        let token = self.eat_articles();
        match token.kind() {
            TokenKind::Verb(verb) => Ok(verb),
            _ => Err(ParseErr::Unexpected((VerbKind::Dummy.into(), token.kind()))),
        }
    }

    fn expect_adj(&mut self) -> Result<AdjKind> {
        let token = self.eat_articles();
        match token.kind() {
            TokenKind::Adj(adj) => Ok(adj),
            _ => Err(ParseErr::Unexpected((VerbKind::Dummy.into(), token.kind()))),
        }
    }

    fn expect_adverb(&mut self) -> Result<Option<AdverbKind>> {
        let token = self.eat_articles();
        match token.kind() {
            TokenKind::Adverb(adverb) => Ok(Some(adverb)),
            TokenKind::Eof | TokenKind::Eol => Ok(None),
            _ => Err(ParseErr::Unexpected((VerbKind::Dummy.into(), token.kind()))),
        }
    }

    fn eat_articles(&mut self) -> Token {
        loop {
            let token = self.lexer.next_token();
            match token.kind() {
                TokenKind::Article => continue,
                _ => return token,
            }
        }
    }

    fn expect_pos(&mut self) -> Result<(Option<AdjKind>, NounKind, Option<AdverbKind>)> {
        let token = self.eat_articles();
        match token.kind() {
            TokenKind::Noun(noun) => {
                let adverb_token = self.expect_adverb()?;
                Ok((None, noun, adverb_token))
            }
            _ => {
                Err(ParseErr::MissingNoun)
            }
        }
    }

    fn parse_svn(&mut self, verb: VerbKind) -> Result<Option<Expr>> {
        let (adj, noun, adverb) = self.expect_pos()?;
        match (verb, noun) {
            _ if is_exit_game(verb, noun) => Ok(Some(Expr::Program(ProgramExpr {
                verb: VerbKind::Exit,
                noun,
                adverb,
            }))),
            _ => Ok(Some(Expr::Game(GameExpr::Svn {
                verb,
                noun,
                adj,
                adverb,
            }))),
        }
    }

    pub fn next(&mut self) -> Result<Option<Expr>> {
        let token = self.lexer.next_token();

        match token.kind() {
            TokenKind::Hint => Ok(Some(Expr::Hint)),
            TokenKind::Help => Ok(Some(Expr::Help)),
            TokenKind::Verb(verb) => self.parse_svn(verb),
            TokenKind::Eof => Ok(None),
            k => Err(ParseErr::Unexpected((VerbKind::Dummy.into(), k))),
        }
    }
}
