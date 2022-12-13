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

    let capture_group_names = capture_groups(&re);
    let has_capture_groups = capture_group_names.len() > 0;

    if has_capture_groups {
        let value_result1 =
            capture_with_groups(&re, capture_group_names, val, pattern_span, value_span)?;
        Ok(value_result1)
    } else {
        let value_result2 = capture_without_groups(&re, pattern_span, val, value_span)?;
        Ok(value_result2)
    }
}

fn capture_groups(regex: &Regex) -> Vec<String> {
    regex
        .capture_names()
        .enumerate()
        .skip(1)
        .map(|(i, name)| {
            name.map(String::from)
                .unwrap_or_else(|| format!("capgrp{}", i))
        })
        .collect()
}

fn capture_without_groups(
    re: &Regex,
    pattern_span: Span,
    val: &str,
    value_span: Span,
) -> Result<Value, LabeledError> {
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

fn capture_with_groups(
    re: &Regex,
    capture_group_names: Vec<String>,
    input: &str,
    pattern_span: Span,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let mut recs: Vec<Value> = Vec::new();
    let capture_matches = re.captures_iter(input);

    for c in capture_matches {
        let captures = match c {
            Ok(c) => c,
            Err(e) => {
                return Err(LabeledError {
                    label: "Error with regular expression captures".into(),
                    msg: format!("error: {}", e),
                    span: Some(pattern_span),
                });
            }
        };

        for (column_name, cap) in capture_group_names.iter().zip(captures.iter().skip(1)) {
            let mut cols = Vec::with_capacity(capture_group_names.len());
            let mut vals = Vec::with_capacity(captures.len());

            let cap_string = cap
                .map(|v| (input.to_string(), v.as_str(), v.start(), v.end()))
                .unwrap_or(("".to_string(), "", 0, 0));

            cols.push("input".to_string());
            vals.push(Value::String {
                val: cap_string.0,
                span: value_span,
            });
            cols.push("capture_name".to_string());
            vals.push(Value::String {
                val: column_name.clone(),
                span: value_span,
            });
            cols.push("match".to_string());
            vals.push(Value::String {
                val: cap_string.1.to_string(),
                span: value_span,
            });
            cols.push("begin".to_string());
            vals.push(Value::Int {
                val: cap_string.2 as i64,
                span: value_span,
            });
            cols.push("end".to_string());
            vals.push(Value::Int {
                val: cap_string.3 as i64,
                span: value_span,
            });

            recs.push(Value::Record {
                cols,
                vals,
                span: pattern_span,
            });
        }
    }

    Ok(Value::List {
        vals: recs,
        span: pattern_span,
    })
}
