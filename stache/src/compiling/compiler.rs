use super::error::CompilingError;
use status::Status;
use rule::{ DefaultRule };
use Template;
use super::matcher::Matcher;
use expr::Delimiter;

type CompilingStatus = Status<CompilingError>;

#[derive(Debug, Clone)]
pub struct Compiler {
    matcher: Matcher,
    compiled: Vec<DefaultRule>,
    trailed: Vec<DefaultRule>
}

#[derive(Debug, Clone)]
struct Position {
    processed: Option<String>,
    remained: Option<String>,
    status: CompilingStatus
}

impl Default for Position {
    fn default() -> Self {
        Position {
            processed: None,
            remained: None,
            status: CompilingStatus::default()
        }
    }
}

impl Compiler {
    pub fn new(matcher: Matcher) -> Self {
        Compiler {
            matcher: matcher,
            compiled: vec![],
            trailed: vec![]
        }
    }

    fn process(&mut self, input: String, debug: CompilingStatus) -> Position {
        let next_input: Option<String>;
        let captured: Option<String>;

        let old_input = input.clone();
        let mut next_status = debug.clone();

        if let Some(capture) = self.matcher.captures(&input) {
            let len = capture[0].len();
            let (s, remain) = old_input.split_at(len);

            // updates compiler status
            captured = Some(String::from(s));
            next_status.updates(s);

            // updates template input
            next_input = Some(remain.to_string());

            let open = String::from(capture.name("open").unwrap().as_str());
            let close = String::from(capture.name("close").unwrap().as_str());
            let symbol = String::from(capture.name("symbol").unwrap().as_str());
            let key = String::from(capture.name("key").unwrap().as_str());

            // updates output rules
            self.compiled.push(
                DefaultRule::Symbolic( Delimiter { open, close }, symbol, key)
            );
        } else { // fills the default rule
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<DefaultRule> = None;
            let mut last_rule: Option<DefaultRule> = None;

            // updates compiler status
            captured = Some(String::from(s));
            next_status.updates(s);

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

        Position {
            processed: captured,
            remained: next_input,
            status: next_status
        }
    }

    pub fn compiles_template(&mut self, tmpl: &String) -> Result<Template<DefaultRule>, CompilingError> {
        let mut input = tmpl.clone();

        // FIRST STEP: compiles input to template
        while !&input.is_empty() {
            let position = self.process(input, Status::default());

            if let Some(new_input) = position.remained {
                input = new_input;
            } else {
                panic!()
            }

            if let Some(error) = position.status.error {
                return Err(error);
            }
        }

        // resets status and prepares second step
        //self.status = CompilingStatus::default();
        //self.compiled.reverse();

        // SECOND STEP: trails template to sanitized template
        /*
        while let Some(status) = compiler.trails() {
        if let Some(error) = status.error {
        return Err(error);
    }
    }

    */

        Ok(Template::new(self.compiled.clone()))
    }
}

/*
    pub fn trails(&mut self) -> Option<CompilingStatus> {
        if self.compiled.is_empty() {
            return None;
        }

        // FIXME differentiate newline between a single-lined string and a multi-line string

        // processes the current rule
        let mut current = Rule::default();
        if let Some(rule) = self.compiled.pop() {
            current = rule;
        }

        // analyses the previous and the next rule
        let mut next = Rule::default();
        let mut prev = Rule::default();

        if let Some(rule) = self.compiled.last() {
            next = rule.clone();
        }

        if let Some(rule) = self.trailed.last() {
            prev = rule.clone();
        }

        use self::Rule::*;

        match (prev, current.clone(), next) {
            (prec, Default(mut out), next) => {
                if let Symbolic(symbol, ..) = prec {
                    // instruction rules connot be followed by newlines
                    if symbol.is_instruction() || symbol.is_comment() {
                        if out.starts_with("\r") {
                            out.remove(0);
                        }

                        if out.starts_with("\n") {
                            out.remove(0);
                        }
                    }
                }

                if let Symbolic(symbol, ..) = next {
                    // instruction rules connot be preceded by whitespaces
                    if symbol.is_instruction() || symbol.is_comment() {
                        let backup = out.clone();
                        let clone = out.clone();
                        let mut reversed = clone.chars().rev();

                        while reversed.next() == Some(' ') {
                            out.pop();
                        }

                        if out.pop() != Some('\n') {
                            out = backup;
                        } else {
                            out.push('\n');
                        }
                    }
                }

                if out != String::default() {
                    self.trailed.push(Default(out));
                }
            },
            _ => self.trailed.push(current)
        }

        Some(self.status.clone())
    }
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
