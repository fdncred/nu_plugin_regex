mod regex_;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{
    Category, PluginExample, PluginSignature, Record, Span, Spanned, SyntaxShape, Type, Value,
};

struct Regex_;

impl Regex_ {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Regex_ {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("regex")
            .usage("Parse input with a regular expression")
            .required(
                "pattern",
                SyntaxShape::String,
                "the regular expression to use",
            )
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::String, Type::Table(vec![]))])
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "Parse a string with a regular expression".into(),
                example: r#""hello world" | regex '(?P<first>\w+) (?P<second>\w+)'"#.into(),
                result: Some(Value::list(
                    vec![
                        Value::test_record(Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::test_string("hello world"),
                                Value::test_string("capgrp0"),
                                Value::test_string("hello world"),
                                Value::test_int(0),
                                Value::test_int(11),
                            ],
                        }),
                        Value::test_record(Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::test_string("hello world"),
                                Value::test_string("first"),
                                Value::test_string("hello"),
                                Value::test_int(0),
                                Value::test_int(5),
                            ],
                        }),
                        Value::test_record(Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::test_string("hello world"),
                                Value::test_string("second"),
                                Value::test_string("world"),
                                Value::test_int(6),
                                Value::test_int(11),
                            ],
                        }),
                    ],
                    Span::test_data(),
                )),
            }])]
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
            Value::String { val, .. } => Ok(crate::regex_::regex_from_string(
                &pattern.item,
                pattern.span,
                val,
                input.span(),
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
