use super::error::CompilingError;
use status::Status;
use rule::DefaultRule;

use std::collections::HashMap;
use regex;
use serde_json::Value;

use std::marker::Sized;

type CompilingStatus = Status<CompilingError>;

impl CompilingStatus {
    fn updates(&mut self, s: &str) {
        let lines: Vec<&str> = s.lines().collect();
        let lines_len: i32 = lines.len() as i32;

        if lines_len == 1 {
            if s == "\n" {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += lines[0].len() as i32;
            }
        } else if let Some(line) = lines.last() {
            self.column += lines_len;
            self.line = line.len() as i32;
        }
    }
}

struct Compiler<'r, R> where R: Sized {
    pub status: CompilingStatus,
    compiled: Vec<DefaultRule<'r>>,
    trailed: Vec<R>
}

impl<'r, R> Default for Compiler<'r, R> where R: Sized {
    fn default() -> Self {
        Compiler {
            status: CompilingStatus::default(),
            compiled: vec![],
            trailed: vec![]
        }
    }
}


impl<'r, R> Compiler<'r, R> where R: Sized + RuleMatching {
    pub fn new(input: &'r str) -> Self {
        Compiler::default()
    }

    fn compiles(&mut self, input: &'r String, matcher: &'r Matcher) -> Result<Option<String>, CompilingError> {
        let new_input: Option<String>;
        let old_input = input.clone();

        // matches some specific rules
        if let Some(capture) = matcher.captures(input) {
            let len = capture[0].len();
            let (s, remain) = old_input.split_at(len);

            // updates compiler status
            self.status.updates(s);

            // updates template input
            new_input = Some(remain.to_string());

            let open = capture.name("open").unwrap().as_str();
            let close = capture.name("close").unwrap().as_str();
            let symbol = capture.name("symbol").unwrap().as_str();

            // updates output rules
            self.compiled.push(
                match (&capture["symbol"], &capture["key"]) {
                    (_, ".") =>
                        DefaultRule::Iterator((open, close), symbol),
                    _ =>
                        DefaultRule::Symbolic((open, close), symbol, capture["key"].trim().to_string())
                }
            );
        } else { // fills the default rule
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<DefaultRule> = None;

            // updates compiler status
            self.status.updates(s);

            // updates template input
            new_input = Some(remain.to_string());

            // updates output rules
            match self.compiled.last_mut() {
                Some(&mut DefaultRule::Default(ref mut value)) => {
                    value.push_str(s);
                },
                _ => {
                    new_rule = Some(DefaultRule::Default(s.to_string()));
                }
            }

            if let Some(rule) = new_rule {
                self.compiled.push(rule);
            }
        }

        Ok(new_input)
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
}

use compiling::{ Matcher, RuleMatching };
use rule::Template;

pub fn compile_template<'r, R>(tmpl: String) -> Result<Template<R>, CompilingError> where R: Sized + Clone + PartialEq + RuleMatching + From<DefaultRule<'r>> {
    let matcher: &'r Matcher = &Matcher::build(R::configure_matching()).unwrap(); //FIXME all things needs to be done at the end, try to put out the 'r lifetime
    let mut compiler = Compiler::<R>::default();

    let mut current_input = tmpl.clone();
    let mut old_input;

    // FIRST STEP: compiles input to template
    while !current_input.is_empty() {
        old_input = current_input.clone();

        if let Ok(new_input) = compiler.compiles(&old_input, &matcher) {
            if let Some(new_input) = new_input {
                current_input = new_input;
            }
        }

        if let Some(error) = compiler.status.error {
            return Err(error);
        }
    }

    // resets status and prepares second step
    compiler.status = CompilingStatus::default();
    compiler.compiled.reverse();

    // SECOND STEP: trails template to sanitized template
    /*
    while let Some(status) = compiler.trails() {
        if let Some(error) = status.error {
            return Err(error);
        }
    }

    */

    let mut converted_rules: Vec<R> = vec![];
    for rule in compiler.compiled {
        converted_rules.push(R::from(rule))
    }

    Ok(Template::new(converted_rules))
}

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

pub fn compile() {
    unimplemented!()
}
