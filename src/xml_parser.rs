use crate::detail_parser::parse_detail_section;
use crate::error::CotError;
use crate::model::FlatCotEvent;
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn parse_cot(xml: &str) -> Result<FlatCotEvent, CotError> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut flat = FlatCotEvent {
        uid: String::new(),
        type_: String::new(),
        time: String::new(),
        start: String::new(),
        stale: String::new(),
        how: String::new(),
        lat: 0.0,
        lon: 0.0,
        hae: 0.0,
        ce: 0.0,
        le: 0.0,
        callsign: None,
        group_name: None,
        detail_extra: Default::default(),
    };

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(ref e) if e.name().as_ref() == b"event" => {
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = attr.unescape_value().unwrap_or_default().to_string();
                    match key.as_str() {
                        "uid" => flat.uid = val,
                        "type" => flat.type_ = val,
                        "time" => flat.time = val,
                        "start" => flat.start = val,
                        "stale" => flat.stale = val,
                        "how" => flat.how = val,
                        "lat" => flat.lat = val.parse().unwrap_or(0.0),
                        "lon" => flat.lon = val.parse().unwrap_or(0.0),
                        "hae" => flat.hae = val.parse().unwrap_or(0.0),
                        "ce" => flat.ce = val.parse().unwrap_or(0.0),
                        "le" => flat.le = val.parse().unwrap_or(0.0),
                        _ => {}
                    }
                }
            }
            Event::Start(ref e) if e.name().as_ref() == b"detail" => {
                let mut detail_buf = Vec::new();
                let mut depth = 1;

                // Read until we find the matching end tag
                loop {
                    match reader.read_event_into(&mut detail_buf) {
                        Ok(Event::Start(_)) => depth += 1,
                        Ok(Event::End(_)) => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        Ok(Event::Eof) => break,
                        _ => {}
                    }
                    detail_buf.clear();
                }

                // Get the inner XML as a string
                let inner_xml = String::from_utf8_lossy(&detail_buf);
                let (callsign, group_name, extras) = parse_detail_section(&inner_xml);
                flat.callsign = callsign;
                flat.group_name = group_name;
                flat.detail_extra = extras;
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(flat)
}
