use clipboard::{ClipboardContext, ClipboardProvider};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::str::from_utf8;

fn main() {
    let parsed_attributes = vec!["data-ui-id", "data-ui-content"];
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let xml = ctx.get_contents().unwrap();
    let result = match parse(parsed_attributes, &xml) {
        Ok(r) => r,
        Err(_) => "Bruh".to_string(),
    };
    println!("{}", result);
}

fn parse(parsed_attributes: Vec<&str>, xml: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut result: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                for attribute in e.attributes().into_iter() {
                    let key = from_utf8(attribute.as_ref().unwrap().key.into_inner()).unwrap();
                    if parsed_attributes.contains(&key) {
                        let value = attribute.unwrap().value;
                        let value = from_utf8(&value).unwrap();
                        result.push(format!("[{}=\"{}\"]", &key, &value));
                    }
                }
            }
            _ => (),
        }
        buf.clear();
        if result.len() > 0 {
            return Ok(result.concat());
        };
    }
    return Err("Bruh".to_string());
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn parse_top_level() {
        let xml = r#"<tag1 data-ui-id="test" data-ui-content="lol">
<tag2><!--Test comment-->Test</tag2>
<tag2>Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert_eq!(
            parse(parsed_attributes, xml).unwrap(),
            "[data-ui-id=\"test\"][data-ui-content=\"lol\"]"
        )
    }

    #[test]
    fn parse_lower_level() {
        let xml = r#"<tag1>
<tag2><!--Test comment-->Test</tag2>
<tag2 data-ui-id="test" data-ui-content="lol">Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert_eq!(
            parse(parsed_attributes, xml).unwrap(),
            "[data-ui-id=\"test\"][data-ui-content=\"lol\"]"
        )
    }

    #[test]
    fn parse_top_level_over_lower_level() {
        let xml = r#"<tag1 data-ui-id="test" data-ui-content="lol">
<tag2><!--Test comment-->Test</tag2>
<tag2 data-ui-id="test1" data-ui-content="lol2">Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert_eq!(
            parse(parsed_attributes, xml).unwrap(),
            "[data-ui-id=\"test\"][data-ui-content=\"lol\"]"
        )
    }

    #[test]
    fn parse_only_dui() {
        let xml = r#"<tag1 data-ui-id="test">
<tag2><!--Test comment-->Test</tag2>
<tag2 data-ui-id="test1" data-ui-content="lol2">Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert_eq!(
            parse(parsed_attributes, xml).unwrap(),
            "[data-ui-id=\"test\"]"
        )
    }

    #[test]
    fn parse_only_duc() {
        let xml = r#"<tag1 data-ui-content="lol">
<tag2><!--Test comment-->Test</tag2>
<tag2 data-ui-id="test1" data-ui-content="lol2">Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert_eq!(
            parse(parsed_attributes, xml).unwrap(),
            "[data-ui-content=\"lol\"]"
        )
    }

    #[test]
    fn nothing_found() {
        let xml = r#"<tag1>
<tag2><!--Test comment-->Test</tag2>
<tag2>Test 2</tag2>
</tag1>"#;
        let parsed_attributes = vec!["data-ui-id", "data-ui-content"];

        assert!(parse(parsed_attributes, xml).is_err())
    }
}
