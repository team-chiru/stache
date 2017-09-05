mod error;
pub type CompilingError = error::CompilingError;

mod matcher;
pub type Matcher = matcher::Matcher;
pub use self::matcher::RuleMatching;

mod compiler;
pub use self::compiler::compile;
pub use self::compiler::compile_template;
