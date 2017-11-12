use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::wav::Recorder;

type WriteResult<'a> = Result<&'a str, String>;

pub fn write<'a>(file_name: &'a str, duration: f32, expression: &'a str) -> WriteResult<'a> {
    parse(expression)
        .and_then(|expr| {
            Recorder::new(8_000).record(file_name, duration, &mut ExprSignal::from(expr))
        })
        .and(Ok(file_name))
}
