mod regex_;

use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{
    Category, PluginExample, PluginSignature, Span, Spanned, SyntaxShape, Type, Value,
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
                result: Some(Value::List {
                    vals: vec![
                        Value::Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::String {
                                    val: "hello world".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "capgrp0".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "hello world".into(),
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 0,
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 11,
                                    span: Span::unknown(),
                                },
                            ],
                            span: Span::unknown(),
                        },
                        Value::Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::String {
                                    val: "hello world".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "first".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "hello".into(),
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 0,
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 5,
                                    span: Span::unknown(),
                                },
                            ],
                            span: Span::unknown(),
                        },
                        Value::Record {
                            cols: vec![
                                "input".into(),
                                "capture_name".into(),
                                "match".into(),
                                "begin".into(),
                                "end".into(),
                            ],
                            vals: vec![
                                Value::String {
                                    val: "hello world".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "second".into(),
                                    span: Span::unknown(),
                                },
                                Value::String {
                                    val: "world".into(),
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 6,
                                    span: Span::unknown(),
                                },
                                Value::Int {
                                    val: 11,
                                    span: Span::unknown(),
                                },
                            ],
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                }),
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
