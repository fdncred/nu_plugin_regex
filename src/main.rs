mod regex_;

use nu_plugin::{
    serve_plugin, EngineInterface, EvaluatedCall, MsgPackSerializer, Plugin, PluginCommand,
    SimplePluginCommand,
};
use nu_protocol::{Category, Example, LabeledError, Signature, Spanned, SyntaxShape, Type, Value};

struct RegExPlugin;

impl Plugin for RegExPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Regex_)]
    }
}

struct Regex_;

impl SimplePluginCommand for Regex_ {
    type Plugin = RegExPlugin;

    fn name(&self) -> &str {
        "regex"
    }

    fn usage(&self) -> &str {
        "Parse input with a regular expression and display the results."
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .required(
                "pattern",
                SyntaxShape::String,
                "the regular expression to use",
            )
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::String, Type::Table(vec![]))])
            .category(Category::Experimental)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Parse a string with a regular expression".into(),
            example: r#""hello world" | regex '(?P<first>\w+) (?P<second>\w+)'"#.into(),
            result: None,
        }]
    }

    fn run(
        &self,
        _config: &RegExPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let pattern: Spanned<String> = call.req(0)?;
        let span = input.span();
        match input {
            Value::String { val, .. } => Ok(crate::regex_::regex_from_string(
                &pattern.item,
                pattern.span,
                val,
                span,
            )?),
            v => Err(
                LabeledError::new(format!("requires string input, got {}", v.get_type()))
                    .with_label("Expected string from pipeline", call.head),
            ),
        }
    }
}

fn main() {
    serve_plugin(&RegExPlugin, MsgPackSerializer);
}
