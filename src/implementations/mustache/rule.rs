use rule::{ Rule, DefaultRule };
use compiling::RuleMatching;

use expr;

#[derive(PartialEq, Debug, Clone)]
pub enum MustacheRule {
    Interpolation(String),
    EscapedInterpolation(String),
    Wrap(String),
    InvertedWrap(String),
    Close(String),
    Import(String),
    Comment(String),
    IteratedWrap,
    Draft(String),
    None
}

impl<'r> From<DefaultRule<'r>> for MustacheRule {
    fn from<'s>(rule: DefaultRule) -> Self {
        use self::DefaultRule::*;
        use self::MustacheRule::*;

        match rule {
            Symbolic(("{{", "}}"), ref directive, ref key) => {
                let key = key.clone();

                match (*directive).into() {
                    "" => EscapedInterpolation(key),
                    "&" => Interpolation(key),
                    "#" => Wrap(key),
                    "^" => InvertedWrap(key),
                    "/" => Close(key),
                    ">" => Import(key),
                    "!" => Comment(key),
                    _ => MustacheRule::None
                }
            },
            Symbolic(("{{{", "}}}"), ref directive, ref key) => {
                match (*directive).into() {
                    "" => Interpolation(key.clone()),
                    _ => MustacheRule::None
                }
            },
            Iterator(("{{", "}}"), ref iterator) => {
                match (*iterator).into() {
                    "" => Interpolation(String::default()),
                    "#" => IteratedWrap,
                    "/" => Close(String::default()),
                    _ => MustacheRule::None
                }
            },
            Default(value) => Draft(value),
            _ => MustacheRule::None
        }
    }
}

impl<'r> Into<DefaultRule<'r>> for MustacheRule {
    fn into(self) -> DefaultRule<'r> {
        use self::DefaultRule::*;
        use self::MustacheRule::*;

        match self {
            Interpolation(key) => Symbolic(("{{{", "}}}"), "", key),
            EscapedInterpolation(key) => Symbolic(("{{", "}}"), "", key),
            Wrap(key) => Symbolic(("{{", "}}"), "#", key),
            InvertedWrap(key) => Symbolic(("{{", "}}"), "^", key),
            Close(key) => Symbolic(("{{", "}}"), "/", key),
            Import(key) => Symbolic(("{{", "}}"), ">", key),
            Comment(comment) => Symbolic(("{{", "}}"), "!", comment),
            IteratedWrap => Iterator(("{{", "}}"), "#"),
            Draft(text) => Default(text),
            MustacheRule::None => DefaultRule::None
        }
    }
}

impl<'r> Rule<'r> for MustacheRule {}


impl RuleMatching for MustacheRule {
    #[inline]
    fn get_delimiters<'d>() -> Vec<expr::Delimiter<'d>> {
        vec![("{{", "}}"), ("{{{", "}}}")]
    }

    #[inline]
    fn get_directives<'s>() -> Vec<expr::Symbol<'s>> {
        vec!["", "#", "^", "/", ">", "!", "&", "#"]
    }

    #[inline]
    fn get_default_keys<'k>() -> &'k str {
        r" \sa-zA-Z0-9!.\-='^#/!?>&"
    }
}
