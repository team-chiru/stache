use super::rule::Rule;
use super::error::CompilingError;
use super::CompilingStatus;

use regex::Regex;

struct Compiler {
    status: CompilingStatus,
    input: String,
    output: Vec<Rule>
}

static REGEX: &'static str = r"^\{\{(?P<symbol>[=^#/!?>&]?)(?P<key>[ \sa-zA-Z]+)\}\}";

impl Iterator for Compiler {
    type Item = CompilingStatus;

    fn next(&mut self) -> Option<CompilingStatus> {
        if self.input.is_empty() {
            return None;
        }

        let re = Regex::new(REGEX).unwrap();
        let new_input: Option<String>;
        let old_input = self.input.clone();

        if let Some(capture) = re.captures(&self.input) {
            let len = capture[0].len();
            let (s, remain) = old_input.split_at(len);

            let lines: Vec<&str> = s.lines().collect();
            let lines_len: i32 = lines.len() as i32;

            if lines_len == 1 {
                self.status.line_index += lines[0].len() as i32;
            } else if let Some(line) = lines.last() {
                self.status.column_index += lines_len;
                self.status.line_index = line.len() as i32;
            }

            self.output.push(
                Rule::Symbolic(
                    capture["symbol"].to_string(),
                    capture["key"].to_string()
                )
            );

            new_input = Some(remain.to_string());
        } else {
            let (s, remain) = old_input.split_at(1);
            let mut new_rule: Option<Rule> = None;

            if s == "\n" {
                self.status.line_index += 1;
                self.status.column_index = 0;
            } else {
                self.status.column_index += 1;
            }

            new_input = Some(remain.to_string());

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

        println!("{:?}", status);
    }

    Ok(compiler.output)
}
