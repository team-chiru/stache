use { Template, Partials };
use error::RenderingError;

pub trait TemplateEngine<R, Input, Output> {
    fn render(Template<R>, Partials<R>, Vec<Input>) -> Result<Output, RenderingError>
    where Self: TemplateEngine<R, Input, Output> + Sized, Input: Clone;
}
