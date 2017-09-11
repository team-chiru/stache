use error::CompilingError;
use super::matcher::Matcher;
use super::compiler::Compiler;
use Template;
use expr::{ DescriptionCollector, Expression, Description };

use toml;
use serde;

pub trait RuleCompiler<R> {
    fn compiles(&String) -> Result<Template<R>, CompilingError>;
}

pub fn compiles_raw<R>(descr_raw: &str, raw: &String) -> Result<Template<R>, CompilingError>
where for<'de> R: serde::Deserialize<'de> + Clone + PartialEq {
    let descr: Description = toml::from_str(&descr_raw).unwrap();
    let expressions = Expression::collect(&descr);

    if let Some(expr) = expressions.last() {
        let tmpl = raw.clone();
        let matcher = Matcher::build(expr).unwrap();
        let mut compiler = Compiler::new(matcher);

        let defaults = compiler.compiles_template(&tmpl).unwrap();
        Ok(defaults.from_default::<R>(&descr))

    } else {
        Err(CompilingError::InvalidStatement(String::from("no expressions")))
    }
}
