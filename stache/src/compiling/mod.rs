mod error;
pub type CompilingError = error::CompilingError;

mod matcher;
pub type Matcher = matcher::Matcher;
pub use self::matcher::RuleMatching;

mod compiler;
//pub type Compiler = matcher::Matcher;
pub use self::compiler::compiles_template;
