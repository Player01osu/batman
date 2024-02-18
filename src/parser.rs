#![allow(dead_code)]
use crate::lexer::{AdjKind, AdverbKind, Lexer, NounKind, Token, TokenKind, VerbKind};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

#[derive(Debug, Clone)]
pub struct GameExpr {
    verb: VerbKind,
    noun: NounKind,
    adverb: Option<AdverbKind>,
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

    Raw(String),
}

#[derive(Debug, Clone)]
pub enum ParseErr {
    Unexpected((TokenKind, TokenKind)),
}

type Result<T> = std::result::Result<T, ParseErr>;

fn is_game_verb(verb: VerbKind) -> bool {
    true
}

fn is_program_verb(verb: VerbKind) -> bool {
    true
}

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
            _ => unimplemented!(),
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
            (VerbKind::Get, NounKind::Chair) => todo!(),
            (VerbKind::Get, NounKind::Door) => todo!(),
            (VerbKind::Get, NounKind::Stair) => todo!(),
            (VerbKind::Get, NounKind::Weapon) => todo!(),
            (VerbKind::Get, NounKind::Homework) => todo!(),
            (VerbKind::Get, NounKind::Computer) => todo!(),
            (VerbKind::Get, NounKind::Game) => todo!(),
            (VerbKind::Get, NounKind::Dummy) => todo!(),
            (VerbKind::Open, NounKind::Chair) => todo!(),
            (VerbKind::Open, NounKind::Door) => todo!(),
            (VerbKind::Open, NounKind::Stair) => todo!(),
            (VerbKind::Open, NounKind::Weapon) => todo!(),
            (VerbKind::Open, NounKind::Homework) => todo!(),
            (VerbKind::Open, NounKind::Computer) => todo!(),
            (VerbKind::Open, NounKind::Game) => todo!(),
            (VerbKind::Open, NounKind::Dummy) => todo!(),
            (VerbKind::Close, NounKind::Chair) => todo!(),
            (VerbKind::Close, NounKind::Door) => todo!(),
            (VerbKind::Close, NounKind::Stair) => todo!(),
            (VerbKind::Close, NounKind::Weapon) => todo!(),
            (VerbKind::Close, NounKind::Homework) => todo!(),
            (VerbKind::Close, NounKind::Computer) => todo!(),
            (VerbKind::Close, NounKind::Dummy) => todo!(),
            (VerbKind::Exit, NounKind::Chair) => todo!(),
            (VerbKind::Exit, NounKind::Door) => todo!(),
            (VerbKind::Exit, NounKind::Stair) => todo!(),
            (VerbKind::Exit, NounKind::Weapon) => todo!(),
            (VerbKind::Exit, NounKind::Homework) => todo!(),
            (VerbKind::Exit, NounKind::Computer) => todo!(),
            (VerbKind::Exit, NounKind::Dummy) => todo!(),
            (VerbKind::Go, NounKind::Chair) => todo!(),
            (VerbKind::Go, NounKind::Door) => todo!(),
            (VerbKind::Go, NounKind::Stair) => todo!(),
            (VerbKind::Go, NounKind::Weapon) => todo!(),
            (VerbKind::Go, NounKind::Homework) => todo!(),
            (VerbKind::Go, NounKind::Computer) => todo!(),
            (VerbKind::Go, NounKind::Game) => todo!(),
            (VerbKind::Go, NounKind::Dummy) => todo!(),
            (VerbKind::Dummy, NounKind::Chair) => todo!(),
            (VerbKind::Dummy, NounKind::Door) => todo!(),
            (VerbKind::Dummy, NounKind::Stair) => todo!(),
            (VerbKind::Dummy, NounKind::Weapon) => todo!(),
            (VerbKind::Dummy, NounKind::Homework) => todo!(),
            (VerbKind::Dummy, NounKind::Computer) => todo!(),
            (VerbKind::Dummy, NounKind::Game) => todo!(),
            (VerbKind::Dummy, NounKind::Dummy) => todo!(),
            (VerbKind::Hit, NounKind::Chair) => todo!(),
            (VerbKind::Hit, NounKind::Door) => todo!(),
            (VerbKind::Hit, NounKind::Stair) => todo!(),
            (VerbKind::Hit, NounKind::Weapon) => todo!(),
            (VerbKind::Hit, NounKind::Homework) => todo!(),
            (VerbKind::Hit, NounKind::Computer) => todo!(),
            (VerbKind::Hit, NounKind::Game) => todo!(),
            (VerbKind::Hit, NounKind::Dummy) => todo!(),
            (VerbKind::Quit, NounKind::Chair) => todo!(),
            (VerbKind::Quit, NounKind::Door) => todo!(),
            (VerbKind::Quit, NounKind::Stair) => todo!(),
            (VerbKind::Quit, NounKind::Weapon) => todo!(),
            (VerbKind::Quit, NounKind::Homework) => todo!(),
            (VerbKind::Quit, NounKind::Computer) => todo!(),
            (VerbKind::Quit, NounKind::Dummy) => todo!(),
            v => unimplemented!("{v:?}"),
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
