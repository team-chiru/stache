use super::error::CompilingError;
use status::Status;
use rule::{ DefaultRule, Template };
use compiling::{ Matcher, RuleMatching };

use std::collections::HashMap;
use regex;
use serde_json::Value;
use std::marker::Sized;

type CompilingStatus = Status<CompilingError>;

#[derive(Debug, Clone)]
struct Compiler<Rule> {
    matcher: Matcher,
    input: String,
    compiled: Vec<Rule>,
    trailed: Vec<Rule>
}

pub fn compiles_template<Rule>(tmpl: &String) -> Result<Template<Rule>, CompilingError> where Rule: RuleMatching + PartialEq + Clone + From<DefaultRule> + Into<DefaultRule> {
    let mut current_input = tmpl.clone();
    let mut old_input;
    let mut compiler: Compiler<Rule> = Compiler::new(tmpl.clone());

    // FIRST STEP: compiles input to template
    while !&compiler.input.is_empty() {
        old_input = current_input.clone();
        let mut next_input: Option<String> = None;

        let position = compiler.process(Status::default());

        if let Some(new_input) = position.remained {
            compiler.input = new_input;
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

    Ok(Template::new(compiler.compiled.clone()))
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

impl<Rule> Compiler<Rule> where Rule: RuleMatching  {
    fn new(template: String) -> Self {
        Compiler {
            matcher: Matcher::build(Rule::configure_matching()).unwrap(),
            input: template,
            compiled: vec![],
            trailed: vec![]
        }
    }

    fn process(&mut self, debug_status: CompilingStatus) -> Position where Rule: From<DefaultRule> + Into<DefaultRule> {
        let next_input: Option<String>;
        let captured: Option<String>;

        let old_input = self.input.clone();
        let mut next_status = debug_status.clone();

        if let Some(capture) = self.matcher.captures(&self.input) {
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

            // updates output rules
            self.compiled.push(
                match (&capture["symbol"], &capture["key"]) {
                    (_, ".") =>
                        Rule::from(DefaultRule::Iterator((open, close), symbol)),
                    _ =>
                        Rule::from(DefaultRule::Symbolic((open, close), symbol, capture["key"].trim().to_string()))
                }
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
                self.compiled.push(Rule::from(rule));
            }

            if let Some(rule) = new_rule {
                self.compiled.push(Rule::from(rule));
            }
        }

        Position {
            processed: captured,
            remained: next_input,
            status: next_status
        }
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
