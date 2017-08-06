use rule::{ Template, Rule };
use std::fmt::Debug;
use error::ExecutionError;

#[derive(PartialEq, Debug, Clone)]
pub enum Command<Input, Output> {
    Skip(Rule),
    SliceOff(Rule, Vec<Input>, bool),
    Import(Rule),
    Write(Output),
    None
}

pub trait Engine<Input, Output> {
    fn decide(&Rule, &Input) -> Self;
    fn execute(self, &mut Template, &Vec<Input>) -> Result<String, ExecutionError>;

    fn process_all(tmpl: Template, contexts: Vec<Input>) -> Result<String, ExecutionError>
    where Self: Engine<Input, Output> + Sized + Debug, Input: Clone + Debug {
        let mut output = String::default();
        let mut tmpl = tmpl.clone();

        while let Some(rule) = tmpl.next() {
            let mut context_stack = contexts.iter().rev();

            while let Some(context) = context_stack.next() {
                let cmd = Self::decide(&rule, &context);
                let mut is_written = false;

                match cmd.execute(&mut tmpl, &contexts) {
                    Ok(value) => {
                        if value != String::default() {
                            output.push_str(&value);
                            is_written = true;
                        }
                    },
                    Err(error) => return Err(error)
                }

                if is_written || rule.is_dotted() {
                    break;
                }
            }
        }

        Ok(output)
    }
}
