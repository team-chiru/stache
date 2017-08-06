use super::rule::{ Symbol, Rule, Template };
use super::error::CompilingError;
use super::status::Status;

use regex::Regex;

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

struct Compiler {
    status: CompilingStatus,
    input: String,
    compiled: Vec<Rule>,
    trailed: Vec<Rule>
}

// test: https://regex101.com/r/XJ6sWg/1
static REGEX: &'static str = r"^\{\{(?P<symbol>[=^#/!?>&]?)(?P<key>[ \sa-zA-Z0-9!.\-='^#/!?>&]+)\}\}";

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            status: CompilingStatus::default(),
            input: String::default(),
            compiled: vec![],
            trailed: vec![]
        }
    }
}

impl Compiler {
    pub fn new(input: &String) -> Self{
        let mut compiler = Compiler::default();
        compiler.input = input.clone();

        compiler
    }

    fn compiles(&mut self) -> Option<CompilingStatus> {
        if self.input.is_empty() {
            return None;
        }

        // TODO add regex builder logic from rule configuration
        let re = Regex::new(REGEX).unwrap();

        let new_input: Option<String>;
        let old_input = self.input.clone();

        // matches some specific rules
        if let Some(capture) = re.captures(&self.input) {
            let len = capture[0].len();
            let (s, remain) = old_input.split_at(len);

            // updates compiler status
            self.status.updates(s);

            // updates template input
            new_input = Some(remain.to_string());

            // updates output rules
            let symbol = capture["symbol"].to_string();
            let key = capture["key"].trim().to_string();

            self.compiled.push(
                match (&capture["symbol"], &capture["key"]) {
                    (_, ".") => Rule::Noop(false, symbol),
                    _ => Rule::Symbolic(false, Symbol::from(symbol), key)
                }
            );
        } else { // fills the default rule
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<Rule> = None;

            // updates compiler status
            self.status.updates(s);

            // updates template input
            new_input = Some(remain.to_string());

            // updates output rules
            match self.compiled.last_mut() {
                Some(&mut Rule::Default(false, ref mut value)) => {
                    value.push_str(s);
                },
                _ => {
                    new_rule = Some(Rule::Default(false, s.to_string()));
                }
            }

            if let Some(rule) = new_rule {
                self.compiled.push(rule);
            }
        }

        // updates compiler input (effective)
        if let Some(input) = new_input {
            self.input = input;
        }

        Some(self.status.clone())
    }

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
            (prec, Default(false, mut out), next) => {
                if let Symbolic(false, symbol, ..) = prec {
                    // instruction rules connot be followed by newlines
                    if out.starts_with("\n") && (symbol.is_instruction() || symbol.is_comment()) {
                        out.remove(0);
                    }

                }

                if let Symbolic(false, symbol, ..) = next {
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
                    self.trailed.push(Default(false, out));
                }
            },
            _ => self.trailed.push(current)
        }

        Some(self.status.clone())
    }
}

pub fn compile(tmpl: String) -> Result<Template, CompilingError> {
    let mut compiler = Compiler::new(&tmpl);

    // FIRST STEP: compiles input to template
    while let Some(status) = compiler.compiles() {
        if let Some(error) = status.error {
            return Err(error);
        }
    }

    // resets status and prepares second step
    compiler.status = CompilingStatus::default();
    compiler.compiled.reverse();

    // SECOND STEP: trails template to sanitized template
    while let Some(status) = compiler.trails() {
        if let Some(error) = status.error {
            return Err(error);
        }
    }

    Ok(Template::new(compiler.trailed))
}
