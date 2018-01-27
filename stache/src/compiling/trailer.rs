use super::error::CompilingError;
use rule::{ Rule, DefaultRule };
use { Template };

//use status::Status;
//type TrailingStatus = Status<CompilingError>;

#[derive(Debug, Clone)]
pub struct Trailer {
    trailed: Vec<DefaultRule>
}

impl Trailer {
    pub fn new() -> Self {
        Trailer { trailed: vec![] }
    }

    pub fn process(&mut self, input: &mut Vec<DefaultRule>) -> Option<Vec<DefaultRule>> {
        // FIXME differentiate newline between a single-lined string and a multi-line string

        // processes the current rule
        let current = input.pop();

        // analyses the previous and the next rule
        let next = input.last();

        let trailed = self.trailed.clone();
        let previous = trailed.last();

        use DefaultRule::Default;

        match (previous, current.clone(), next) {
            (previous, Some(Default(mut out)), next) => {
                // instruction rules connot be followed by newlines
                if let Some(rule) = previous {
                    if rule.is_instruction() || rule.is_comment() {
                        if out.starts_with("\r") {
                            out.remove(0);
                        }

                        if out.starts_with("\n") {
                            out.remove(0);
                        }
                    }
                }

                // instruction rules connot be preceded by whitespaces
                if let Some(rule) = next {
                    if rule.is_instruction() || rule.is_comment() {
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
            (_, Some(rule), _) => {
                self.trailed.push(rule);
            },
            _ => {}
        }

        if input.is_empty() {
            None
        } else {
            Some(input.clone())
        }
    }

    pub fn trails(input: Template<DefaultRule>) -> Result<Template<DefaultRule>, CompilingError> {
        let mut trailer = Trailer::new();
        let mut input = input.clone().rules();

        input.reverse();

        while !&input.is_empty() {
            while let Some(next) = trailer.process(&mut input) {
                input = next;
            }
        }

        Ok(Template::new(trailer.trailed.clone()))
    }
}
