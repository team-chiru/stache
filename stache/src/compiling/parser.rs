use super::error::CompilingError;
use status::Status;
use rule::DefaultRule;
use Template;
use super::matcher::Matcher;
use expr::{ DescriptionCollector, Delimiter, Expression, Description };

type ParsingStatus = Status<CompilingError>;

#[derive(Debug, Clone)]
pub struct Parser {
    matcher: Matcher,
    compiled: Vec<DefaultRule>
}

impl Parser {
    pub fn new(matcher: Matcher) -> Self {
        Parser {
            matcher: matcher,
            compiled: vec![]
        }
    }

    pub fn init(descr: &Description) -> Result<Self, CompilingError> {
        let expressions = Expression::collect(&descr);

        if let Some(expr) = expressions.last() {
            let matcher = Matcher::build(expr).unwrap();

            Ok(Parser::new(matcher))
        } else {
            Err(CompilingError::InvalidStatement(String::from("no expressions")))
        }
    }

    fn process(&mut self, input: String) -> Option<String> {
        let next_input: Option<String>;
        //let captured: Option<String>;

        let old_input = input.clone();
        //let mut next_status = debug.clone();

        if let Some(capture) = self.matcher.captures(&input) {
            let len = capture[0].len();
            let (_, remain) = old_input.split_at(len);

            // updates compiler status
            //captured = Some(String::from(s));
            //next_status.updates(s);

            // updates template input
            next_input = Some(remain.to_string());

            let open;
            let close;
            let symbol;

            if let Some(s) = capture.name("open") {
                open = String::from(s.as_str());
            } else {
                return None;
            }

            if let Some(s) = capture.name("close") {
                close = String::from(s.as_str());
            } else {
                return None;
            }

            if let Some(s) = capture.name("symbol") {
                symbol = String::from(s.as_str());
            } else {
                symbol = String::default();
            }

            let raw_key = capture.name("key").unwrap().as_str();
            let key = String::from(raw_key.trim());

            // updates output rules
            self.compiled.push(
                DefaultRule::Symbolic( Delimiter { open, close }, symbol, key)
            );
        } else { // fills the default rule
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<DefaultRule> = None;
            let mut last_rule: Option<DefaultRule> = None;

            // updates compiler status
            //captured = Some(String::from(s));
            //next_status.updates(s);

            // updates template input
            next_input = Some(remain.to_string());

            // updates output rules
            match self.compiled.pop().map(|r| r.into()) {
                Some(DefaultRule::Default(value)) => {
                    last_rule = Some(DefaultRule::Default(value + s));
                },
                Some(rule) => {
                    last_rule = Some(rule);
                    new_rule = Some(DefaultRule::Default(s.to_string()));
                },
                None => {
                    new_rule = Some(DefaultRule::Default(s.to_string()));
                }
            }

            if let Some(rule) = last_rule {
                self.compiled.push(rule);
            }

            if let Some(rule) = new_rule {
                self.compiled.push(rule);
            }
        }

        if let Some(next) = next_input.clone() {
            if next.is_empty() {
                None
            } else {
                next_input
            }
        } else {
            None
        }
    }

    pub fn parses(&mut self, input: String) -> Result<Template<DefaultRule>, CompilingError> {
        let mut input = input.clone();

        // compiles input to template
        while let Some(next) = self.process(input) {
            input = next;
        }

        Ok(Template::new(self.compiled.drain(..).collect()))
    }
}

#[derive(Debug, Clone)]
struct Position {
    processed: Option<String>,
    remained: Option<String>,
    status: ParsingStatus
}

impl Default for Position {
    fn default() -> Self {
        Position {
            processed: None,
            remained: None,
            status: ParsingStatus::default()
        }
    }
}


/*

    */

/*
pub fn compile_partials<R>(partials: Value) -> Result<HashMap<String, Template<R>>, CompilingError>
where R: Sized + RuleMatching {
    if let Value::Object(map) = partials {
        let mut hash = HashMap::new();

        for (key, value) in map {
            if let Value::String(s) = value {
                hash.insert(key, compile_template(s).unwrap());
            } else {
                return Err(CompilingError::InvalidStatement(
                    String::from("Cannot compile partials")
                ));
            }
        }

        Ok(hash)
    } else {
        Err(CompilingError::InvalidStatement(
            String::from("Cannot compile partials")
        ))
    }
}
*/
