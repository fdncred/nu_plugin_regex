use fancy_regex::Regex;
use nu_plugin::LabeledError;
use nu_protocol::{record, Span, Value};

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

    let has_capture_groups = !capture_groups(&re, false).is_empty();

    if has_capture_groups {
        let value_result1 = capture_with_groups(&re, val, pattern_span, value_span)?;
        Ok(value_result1)
    } else {
        let value_result2 = capture_without_groups(&re, pattern_span, val, value_span)?;
        Ok(value_result2)
    }
}

fn capture_groups(regex: &Regex, with_capture_group_zero: bool) -> Vec<String> {
    if with_capture_group_zero {
        regex
            .capture_names()
            .enumerate()
            .map(|(i, name)| {
                name.map(String::from)
                    .unwrap_or_else(|| format!("capgrp{}", i))
            })
            .collect()
    } else {
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
}

fn capture_without_groups(
    re: &Regex,
    pattern_span: Span,
    val: &str,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let matches = re.find_iter(val);
    let mut recs = Vec::new();

    for match_result in matches {
        match match_result {
            Ok(a_match) => {
                let rec = record!(
                    "input" => Value::string(val.to_string(), value_span),
                    "match" => Value::string(a_match.as_str().to_string(),value_span),
                    "begin" => Value::int(a_match.start() as i64, value_span),
                    "end" => Value::int(a_match.end() as i64, value_span),
                );
                recs.push(Value::record(rec, value_span));
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

    Ok(Value::list(recs, value_span))
}

fn capture_with_groups(
    re: &Regex,
    input: &str,
    pattern_span: Span,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let mut recs: Vec<Value> = Vec::new();
    let capture_group_names = capture_groups(re, true);
    let capture_matches = re.captures_iter(input);

    for capture_result in capture_matches {
        let captures = match capture_result {
            Ok(c) => c,
            Err(e) => {
                return Err(LabeledError {
                    label: "Error with regular expression captures".into(),
                    msg: format!("error: {}", e),
                    span: Some(pattern_span),
                });
            }
        };

        for (column_name, capture_match) in capture_group_names.iter().zip(captures.iter()) {
            let cap_string = capture_match
                .map(|v| (input.to_string(), v.as_str(), v.start(), v.end()))
                .unwrap_or(("".to_string(), "", 0, 0));

            let rec = record!(
                "input" => Value::string(cap_string.0, value_span),
                "capture_name" => Value::string(column_name.clone(), value_span),
                "match" => Value::string(cap_string.1.to_string(), value_span),
                "begin" => Value::int(cap_string.2 as i64, value_span),
                "end" => Value::int(cap_string.3 as i64, value_span),
            );
            recs.push(Value::record(rec, value_span));
        }
    }

    Ok(Value::list(recs, pattern_span))
}
