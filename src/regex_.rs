use fancy_regex::Regex;
use nu_plugin::LabeledError;
use nu_protocol::{Span, Value};

fn validate_regex(pattern: &str, pattern_span: Span) -> Result<Regex, LabeledError> {
    match Regex::new(pattern) {
        Ok(regex) => Ok(regex),
        Err(e) => Err(LabeledError {
            label: "Invalid Regex".into(),
            msg: format!("error: {}", e),
            span: Some(pattern_span),
        }),
    }
}

pub fn regex_from_string(
    pattern: &str,
    pattern_span: Span,
    val: &str,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let re = validate_regex(pattern, pattern_span)?;
    let matches = re.find_iter(val);
    let mut recs = Vec::new();

    for m in matches {
        match m {
            Ok(mat) => {
                let mut cols = Vec::new();
                let mut vals = Vec::new();

                cols.push("input".to_string());
                cols.push("match".to_string());
                cols.push("begin".to_string());
                cols.push("end".to_string());
                vals.push(Value::String {
                    val: val.to_string(),
                    span: value_span,
                });
                vals.push(Value::String {
                    val: mat.as_str().to_string(),
                    span: value_span,
                });
                vals.push(Value::Int {
                    val: mat.start() as i64,
                    span: value_span,
                });
                vals.push(Value::Int {
                    val: mat.end() as i64,
                    span: value_span,
                });
                recs.push(Value::Record {
                    cols,
                    vals,
                    span: value_span,
                });
            }
            Err(e) => {
                return Err(LabeledError {
                    label: "Invalid Regex".into(),
                    msg: format!("error: {}", e),
                    span: Some(pattern_span),
                });
            }
        }
    }

    Ok(Value::List {
        vals: recs,
        span: value_span,
    })
}
