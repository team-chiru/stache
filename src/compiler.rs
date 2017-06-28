use super::rule::Rule;
use super::error::CompilingError;
use super::status::Status;

use regex::Regex;

type CompilingStatus = Status<CompilingError>;

impl CompilingStatus {
    fn update(&mut self, s: &str) {
        let lines: Vec<&str> = s.lines().collect();
        let lines_len: i32 = lines.len() as i32;

        if lines_len == 1 {
            if s == "\n" {
                self.line_index += 1;
                self.column_index = 0;
            } else {
                self.column_index += lines[0].len() as i32;
            }
        } else if let Some(line) = lines.last() {
            self.column_index += lines_len;
            self.line_index = line.len() as i32;
        }
    }
}

struct Compiler {
    status: CompilingStatus,
    input: String,
    output: Vec<Rule>
}

// test: https://regex101.com/r/XJ6sWg/1
static REGEX: &'static str = r"^\{\{(?P<symbol>[=^#/!?>&]?)(?P<key>[ \sa-zA-Z.]+)\}\}";

impl Iterator for Compiler {
    type Item = CompilingStatus;

    fn next(&mut self) -> Option<CompilingStatus> {
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
            self.status.update(s);

            // updates template input
            new_input = Some(remain.to_string());

            // updates output rules
            self.output.push(
                Rule::Symbolic(
                    capture["symbol"].to_string(),
                    capture["key"].to_string()
                )
            );
        } else { // fills the default rule
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<Rule> = None;

            // updates compiler status
            self.status.update(s);

            // updates template input
            new_input = Some(remain.to_string());

            // updates output rules
            match self.output.last_mut() {
                Some(&mut Rule::Default(ref mut value)) => {
                    value.push_str(s);
                },
                _ => {
                    new_rule = Some(Rule::Default(s.to_string()));
                }
            }

            if let Some(rule) = new_rule {
                self.output.push(rule);
            }
        }

        // updates compiler input (effective)
        if let Some(input) = new_input {
            self.input = input;
        }

        Some(self.status.clone())
    }
}

pub fn compile(tmpl: String) -> Result<Vec<Rule>, CompilingError> {
    let mut compiler = Compiler {
        status: CompilingStatus {
            error: None,
            line_index: 0,
            column_index: 0
        },
        input: tmpl,
        output: vec!()
    };

    while let Some(status) = compiler.next() {
        if let Some(error) = status.error {
            return Err(error);
        }
    }

    Ok(compiler.output)
}
