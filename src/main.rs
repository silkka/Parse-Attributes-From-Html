use std::str::from_utf8;

fn main() {
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;

    let xml = r#"<tag1 data-ui-id="test" data-ui-content="lol">
                <tag2><!--Test comment-->Test</tag2>
                <tag2 data-ui-id="test1" data-ui-content="lol1">Test 2</tag2>
             </tag1>"#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut result: Vec<String> = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                for attribute in e.attributes().into_iter() {
                    let key = from_utf8(attribute.as_ref().unwrap().key.into_inner()).unwrap();
                    if key == "data-ui-id" || key == "data-ui-content" {
                        let value = attribute.unwrap().value;
                        let value = from_utf8(&value).unwrap();
                        result.push(format!("[{}={}]", &key, &value));
                    }
                }
            }
            _ => (),
        }
        buf.clear();
        if result.len() > 0 {
            break;
        };
    }

    println!("{:?}", result)
}
