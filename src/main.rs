mod regex_;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, Signature, Spanned, SyntaxShape, Type, Value};

struct Regex_;

impl Regex_ {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Regex_ {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("regex")
            .usage("Parse input with a regular expression")
            .required(
                "pattern",
                SyntaxShape::String,
                "the regular expression to use",
            )
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::String, Type::Table(vec![]))])
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "regex");
        let pattern: Spanned<String> = call.req(0)?;
        match input {
            Value::String { val, span } => Ok(crate::regex_::regex_from_string(
                &pattern.item,
                pattern.span,
                val,
                *span,
            )?),
            v => Err(LabeledError {
                label: "Expected binary from pipeline".into(),
                msg: format!("requires binary input, got {}", v.get_type()),
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Regex_::new(), MsgPackSerializer);
}
