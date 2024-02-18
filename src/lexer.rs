#![allow(dead_code)]
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    src: &'a str,
    len_remaining: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounKind {
    Chair,
    Door,
    Stair,
    Weapon,
    Homework,
    Computer,
    Game,
    Building,
    Stats,
    Foot,
    Bus,
    Back,
    Nothing,
    Bathroom,

    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbKind {
    Get,
    Open,
    Close,
    Exit,
    Quit,
    Go,
    Hit,
    Leave,
    Check,
    Continue,
    Wait,
    Board,
    Enter,
    Head,
    Do,

    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdverbKind {
    Quick,
    Slow,
    Careful,
    Intense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjKind {
    Near,
    Far,
    Strong,
    Weak,
    Away,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Noun(NounKind),
    Verb(VerbKind),
    Adj(AdjKind),
    Adverb(AdverbKind),
    Article,
    Help,
    Hint,
    Eol,
    Eof,
    Illegal,
    Dummy,
}

impl Into<TokenKind> for VerbKind {
    fn into(self) -> TokenKind {
        TokenKind::Verb(self)
    }
}

impl Into<TokenKind> for NounKind {
    fn into(self) -> TokenKind {
        TokenKind::Noun(self)
    }
}

// Life of a Michigan Tech Student

impl Token {
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars(),
            len_remaining: src.len(),
            src,
        }
    }

    fn bump(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn peak(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn consume_word(&mut self, c: char) -> TokenKind {
        let mut word = String::new();
        word.push(c);
        while self.peak().is_ascii_alphabetic() {
            word.push(self.bump().to_ascii_lowercase());
        }

        match word.as_str() {
            "help" => TokenKind::Help,
            "hint" => TokenKind::Hint,

            // Verbs
            "get" => TokenKind::Verb(VerbKind::Get),
            "open" => TokenKind::Verb(VerbKind::Open),
            "close" => TokenKind::Verb(VerbKind::Close),
            "exit" => TokenKind::Verb(VerbKind::Exit),
            "go" => TokenKind::Verb(VerbKind::Go),
            "hit" => TokenKind::Verb(VerbKind::Hit),
            "quit" => TokenKind::Verb(VerbKind::Quit),
            "leave" => TokenKind::Verb(VerbKind::Leave),
            "check" => TokenKind::Verb(VerbKind::Check),
            "continue" => TokenKind::Verb(VerbKind::Continue),
            "wait" => TokenKind::Verb(VerbKind::Wait),
            "board" => TokenKind::Verb(VerbKind::Board),
            "enter" => TokenKind::Verb(VerbKind::Enter),
            "head" => TokenKind::Verb(VerbKind::Head),
            "do" => TokenKind::Verb(VerbKind::Do),

            // Nouns
            "chair" => TokenKind::Noun(NounKind::Chair),
            "door" => TokenKind::Noun(NounKind::Door),
            "stair" => TokenKind::Noun(NounKind::Stair),
            "weapon" => TokenKind::Noun(NounKind::Weapon),
            "homework" => TokenKind::Noun(NounKind::Homework),
            "computer" => TokenKind::Noun(NounKind::Computer),
            "game" => TokenKind::Noun(NounKind::Game),
            "building" => TokenKind::Noun(NounKind::Building),
            "stats" => TokenKind::Noun(NounKind::Stats),
            "foot" => TokenKind::Noun(NounKind::Foot),
            "bus" => TokenKind::Noun(NounKind::Bus),
            "back" => TokenKind::Noun(NounKind::Back),
            "nothing" => TokenKind::Noun(NounKind::Nothing),
            "bathroom" => TokenKind::Noun(NounKind::Bathroom),

            // Adverbs
            "quick" => TokenKind::Adverb(AdverbKind::Quick),
            "slow" => TokenKind::Adverb(AdverbKind::Slow),
            "careful" => TokenKind::Adverb(AdverbKind::Careful),
            "intense" => TokenKind::Adverb(AdverbKind::Intense),

            // Adjectives
            "near" => TokenKind::Adj(AdjKind::Near),
            "far" => TokenKind::Adj(AdjKind::Far),
            "strong" => TokenKind::Adj(AdjKind::Strong),
            "weak" => TokenKind::Adj(AdjKind::Weak),
            "away" => TokenKind::Adj(AdjKind::Away),

            // Articles
            "the" => TokenKind::Article,
            "a" => TokenKind::Article,
            "an" => TokenKind::Article,
            "for" => TokenKind::Article,
            "to" => TokenKind::Article,

            _ => TokenKind::Illegal,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let c = match self.bump() {
            '\0' => return Token {
                kind: TokenKind::Eof,
            },
            c => c,
        };

        let kind = match c {
            'a'..='z' | 'A'..='Z' => self.consume_word(c),
            ' ' => return self.next_token(),
            _ => TokenKind::Illegal,
        };

        Token {
            kind,
        }
    }
}

