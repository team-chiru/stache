use stache::rule::{ Rule, DefaultRule };
use stache::compiling::RuleMatching;
use stache::expr::{ Symbol, Delimiter };

#[derive(RuleMatcher, PartialEq, Debug, Clone)]
#[registry = "mustache.toml"]
pub enum MustacheRule {
    #[command = ""] #[iterator = "."]
    Interpolation(String),

    #[command = "&"] #[iterator = "."]
    EscapedInterpolation(String),

    #[command = "#"]
    Section(String),

    #[command = "^"]
    InvertedSection(String),

    #[command = "/"]
    Close(String),

    #[command = ">"]
    Partial(String),

    #[command = "!"]
    Comment(String),

    #[command = "#"] #[iterator = "."]
    IteratedSection,

    Draft(String),
    None
}

impl From<DefaultRule> for MustacheRule {
    fn from(rule: DefaultRule) -> Self {
        use self::DefaultRule::*;
        use self::MustacheRule::*;

        match rule {
            Symbolic((ref open, ref close), ref directive, ref key) => {
                let key = key.clone();

                match (open.as_ref(), close.as_ref()) {
                    ("{{", "}}") =>
                        match directive.as_ref() {
                            "" => EscapedInterpolation(key),
                            "&" => Interpolation(key),
                            "#" => Section(key),
                            "^" => InvertedSection(key),
                            "/" => Close(key),
                            ">" => Partial(key),
                            "!" => Comment(key),
                            _ => MustacheRule::None
                        },
                    ("{{{", "}}}") =>
                        match directive.as_ref() {
                            "" => Interpolation(key.clone()),
                            _ => MustacheRule::None
                        },
                    _ => MustacheRule::None
                }
            },
            Iterator((ref open, ref close), ref iterator) =>
                match (open.as_ref(), close.as_ref()) {
                    ("{{", "}}") =>
                        match iterator.as_ref() {
                            "" => Interpolation(String::default()),
                            "#" => IteratedSection,
                            "/" => Close(String::default()),
                            _ => MustacheRule::None
                        },
                    _ => MustacheRule::None
                },
            Default(value) => Draft(value),
            _ => MustacheRule::None
        }
    }
}

impl Into<DefaultRule> for MustacheRule {
    fn into(self) -> DefaultRule {
        use self::DefaultRule::*;
        use self::MustacheRule::*;

        match self {
            Interpolation(key) => Symbolic((String::from("{{{"), String::from("}}}")), String::from(""), key),
            EscapedInterpolation(key) => Symbolic((String::from("{{"), String::from("}}")), String::from(""), key),
            Section(key) => Symbolic((String::from("{{"), String::from("}}")), String::from("#"), key),
            InvertedSection(key) => Symbolic((String::from("{{"), String::from("}}")), String::from("^"), key),
            Close(key) => Symbolic((String::from("{{"), String::from("}}")), String::from("/"), key),
            Partial(key) => Symbolic((String::from("{{"), String::from("}}")), String::from(">"), key),
            Comment(comment) => Symbolic((String::from("{{"), String::from("}}")), String::from("!"), comment),
            IteratedSection => Iterator((String::from("{{"), String::from("}}")), String::from("#")),
            Draft(text) => Default(text),
            MustacheRule::None => DefaultRule::None
        }
    }
}

impl<'r> Rule<'r> for MustacheRule {}


impl RuleMatching for MustacheRule {
    #[inline]
    fn get_delimiters() -> Vec<Delimiter> {
        vec![
            (String::from("{{"), String::from("}}")),
            (String::from("{{{"), String::from("}}}"))
        ]
    }

    #[inline]
    fn get_directives() -> Vec<Symbol> {
        vec![
            String::from(""),
            String::from("#"),
            String::from("^"),
            String::from("/"),
            String::from(">"),
            String::from("!"),
            String::from("&"),
            String::from("#")
        ]
    }

    #[inline]
    fn get_default_keys() -> String {
        String::from(r" \sa-zA-Z0-9!.\-='^#/!?>&")
    }
}
