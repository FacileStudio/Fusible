//! Setting one prop on a component's JSX tag, the text around it untouched.

use std::ops::Range;

use serde_json::Value;

use crate::kicad::tsx_tag::find_tag;

/// The component name a `Fix::SetProp` selector targets: `U1` or `.U1`.
fn designator(selector: &str) -> Result<&str, String> {
    let name = selector.trim().trim_start_matches('.');
    let plain = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    match plain {
        true => Ok(name),
        false => Err(format!(
            "selector `{selector}` must name one component, such as U1"
        )),
    }
}

/// `value` as a JSX attribute value: a string literal, or an expression in braces.
fn attribute(value: &Value) -> String {
    match value {
        Value::String(text) if !text.contains(['"', '{', '}']) => format!("\"{text}\""),
        other => format!("{{{other}}}"),
    }
}

/// The end (exclusive) of the attribute value that starts at `start` in `tag`.
fn value_end(tag: &str, start: usize) -> Option<usize> {
    let rest = &tag[start..];
    if let Some(body) = rest.strip_prefix('"') {
        return body.find('"').map(|i| start + i + 2);
    }
    let mut depth = 0usize;
    for (i, c) in rest.char_indices() {
        match c {
            '{' => depth += 1,
            '}' if depth == 1 => return Some(start + i + 1),
            '}' => depth = depth.checked_sub(1)?,
            _ if depth == 0 => return None,
            _ => {}
        }
    }
    None
}

/// The range of `prop={...}` or `prop="..."` in `tag`, when the prop is set.
fn prop_range(tag: &str, prop: &str) -> Option<Range<usize>> {
    let key = format!("{prop}=");
    let at = tag
        .match_indices(&key)
        .map(|(i, _)| i)
        .find(|&i| tag[..i].ends_with(char::is_whitespace))?;
    Some(at..value_end(tag, at + key.len())?)
}

fn with_prop(tag: &str, prop: &str, value: &Value) -> String {
    let written = format!("{prop}={}", attribute(value));
    if let Some(range) = prop_range(tag, prop) {
        return format!("{}{written}{}", &tag[..range.start], &tag[range.end..]);
    }
    let close = tag.len() - if tag.ends_with("/>") { 2 } else { 1 };
    let head = tag[..close].trim_end();
    let between = &tag[head.len()..close];
    let last_line = head.rsplit('\n').next().unwrap_or(head);
    let sep = match head.contains('\n') {
        true => format!(
            "\n{}",
            &last_line[..last_line.len() - last_line.trim_start().len()]
        ),
        false => " ".to_string(),
    };
    format!("{head}{sep}{written}{between}{}", &tag[close..])
}

/// Returns `source` with `prop` set to `value` on the one tag named by `selector`.
pub fn set_prop(source: &str, selector: &str, prop: &str, value: &Value) -> Result<String, String> {
    let name = designator(selector)?;
    if !prop.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || prop.is_empty() {
        return Err(format!("`{prop}` is not a prop name"));
    }
    let range = find_tag(source, name)
        .ok_or_else(|| format!("no single tag with name=\"{name}\" in the board source"))?;
    let tag = with_prop(&source[range.clone()], prop, value);
    Ok(format!(
        "{}{tag}{}",
        &source[..range.start],
        &source[range.end..]
    ))
}

#[cfg(test)]
mod tests {
    use super::set_prop;
    use serde_json::json;

    const BOARD: &str = concat!(
        "<board>\n",
        "  <resistor name=\"R1\" resistance=\"1k\" />\n",
        "  <chip\n",
        "    name=\"U1\"\n",
        "    pinAttributes={{ VDD: { requiresPower: true } }}\n",
        "  />\n",
        "</board>\n",
    );

    #[test]
    fn an_absent_prop_is_added_and_a_present_one_replaced() {
        let added = set_prop(BOARD, "R1", "pcbX", &json!(2.5)).unwrap();
        let replaced = set_prop(BOARD, ".R1", "resistance", &json!("10k")).unwrap();
        let multi = set_prop(BOARD, "U1", "footprint", &json!("soic8")).unwrap();
        let nested = set_prop(BOARD, "U1", "pinAttributes", &json!({"VDD": 1})).unwrap();

        assert!(added.contains("<resistor name=\"R1\" resistance=\"1k\" pcbX={2.5} />"));
        assert!(replaced.contains("resistance=\"10k\" />"));
        assert!(multi.contains("requiresPower: true } }}\n    footprint=\"soic8\"\n  />"));
        assert!(
            nested.contains("pinAttributes={{\"VDD\":1}}\n  />"),
            "{nested}"
        );
    }

    #[test]
    fn an_unknown_or_ambiguous_target_is_refused() {
        assert!(set_prop(BOARD, "U9", "pcbX", &json!(1)).is_err());
        assert!(set_prop(BOARD, ".U1 > .VDD", "pcbX", &json!(1)).is_err());
        assert!(set_prop(BOARD, "U1", "bad prop", &json!(1)).is_err());
    }
}
