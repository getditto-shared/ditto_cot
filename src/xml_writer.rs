use crate::model::FlatCotEvent;

pub fn to_cot_xml(event: &FlatCotEvent) -> String {
    let mut xml = String::new();
    xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str(&format!(
        r#"<event version="2.0" uid="{}" type="{}" time="{}" start="{}" stale="{}" how="{}" lat="{}" lon="{}" hae="{}" ce="{}" le="{}">"#,
        event.uid, event.type_, event.time, event.start, event.stale, event.how,
        event.lat, event.lon, event.hae, event.ce, event.le
    ));
    xml.push_str("<detail>");
    if let Some(ref cs) = event.callsign {
        xml.push_str(&format!(r#"<contact callsign="{}"/>"#, cs));
    }
    if let Some(ref group) = event.group_name {
        xml.push_str(&format!(r#"<__group name="{}"/>"#, group));
    }
    for (k, v) in &event.detail_extra {
        if let Some(obj) = v.as_object() {
            xml.push_str(&format!(r#"<{}"#, k));
            for (key, val) in obj {
                if let Some(s) = val.as_str() {
                    xml.push_str(&format!(r#" {}="{}""#, key, s));
                }
            }
            xml.push_str("/>");
        }
    }
    xml.push_str("</detail>");
    xml.push_str("</event>");
    xml
}