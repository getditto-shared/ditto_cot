package com.ditto.cot;

import jakarta.xml.bind.annotation.*;
import java.time.Instant;
import java.util.HashMap;
import java.util.Map;

/**
 * Represents a Cursor-on-Target (CoT) event parsed from XML
 */
@XmlRootElement(name = "event")
@XmlAccessorType(XmlAccessType.FIELD)
public class CoTEvent {
    
    @XmlAttribute
    private String version;
    
    @XmlAttribute
    private String uid;
    
    @XmlAttribute
    private String type;
    
    @XmlAttribute
    private String time;
    
    @XmlAttribute
    private String start;
    
    @XmlAttribute
    private String stale;
    
    @XmlAttribute
    private String how;
    
    @XmlElement
    private CoTPoint point;
    
    @XmlElement
    private CoTDetail detail;
    
    // Constructors
    public CoTEvent() {}
    
    public CoTEvent(String version, String uid, String type, String time, String start, String stale, String how) {
        this.version = version;
        this.uid = uid;
        this.type = type;
        this.time = time;
        this.start = start;
        this.stale = stale;
        this.how = how;
    }
    
    // Getters and setters
    public String getVersion() { return version; }
    public void setVersion(String version) { this.version = version; }
    
    public String getUid() { return uid; }
    public void setUid(String uid) { this.uid = uid; }
    
    public String getType() { return type; }
    public void setType(String type) { this.type = type; }
    
    public String getTime() { return time; }
    public void setTime(String time) { this.time = time; }
    
    public String getStart() { return start; }
    public void setStart(String start) { this.start = start; }
    
    public String getStale() { return stale; }
    public void setStale(String stale) { this.stale = stale; }
    
    public String getHow() { return how; }
    public void setHow(String how) { this.how = how; }
    
    public CoTPoint getPoint() { return point; }
    public void setPoint(CoTPoint point) { this.point = point; }
    
    public CoTDetail getDetail() { return detail; }
    public void setDetail(CoTDetail detail) { this.detail = detail; }
    
    // Helper methods to access point data without exposing CoTPoint
    public String getPointLatitude() { return point != null ? point.getLat() : null; }
    public String getPointLongitude() { return point != null ? point.getLon() : null; }
    public String getPointHae() { return point != null ? point.getHae() : null; }
    public String getPointCe() { return point != null ? point.getCe() : null; }
    public String getPointLe() { return point != null ? point.getLe() : null; }
    
    // Helper method to access detail data without exposing CoTDetail
    public Map<String, Object> getDetailMap() { return detail != null ? detail.toMap() : new HashMap<>(); }
    
    /**
     * Convert CoT time string to milliseconds since epoch
     */
    public long getTimeMillis() {
        return time != null ? Instant.parse(time).toEpochMilli() : 0;
    }
    
    /**
     * Convert CoT start time to seconds since epoch
     */
    public int getStartSeconds() {
        return start != null ? (int) Instant.parse(start).getEpochSecond() : 0;
    }
    
    /**
     * Convert CoT stale time to seconds since epoch
     */
    public int getStaleSeconds() {
        return stale != null ? (int) Instant.parse(stale).getEpochSecond() : 0;
    }
}

/**
 * Represents the point element in a CoT event
 */
@XmlAccessorType(XmlAccessType.FIELD)
class CoTPoint {
    
    @XmlAttribute
    private String lat;
    
    @XmlAttribute
    private String lon;
    
    @XmlAttribute
    private String hae;
    
    @XmlAttribute
    private String ce;
    
    @XmlAttribute
    private String le;
    
    // Constructors
    public CoTPoint() {}
    
    public CoTPoint(String lat, String lon, String hae, String ce, String le) {
        this.lat = lat;
        this.lon = lon;
        this.hae = hae;
        this.ce = ce;
        this.le = le;
    }
    
    // Getters and setters
    public String getLat() { return lat; }
    public void setLat(String lat) { this.lat = lat; }
    
    public String getLon() { return lon; }
    public void setLon(String lon) { this.lon = lon; }
    
    public String getHae() { return hae; }
    public void setHae(String hae) { this.hae = hae; }
    
    public String getCe() { return ce; }
    public void setCe(String ce) { this.ce = ce; }
    
    public String getLe() { return le; }
    public void setLe(String le) { this.le = le; }
    
    // Convert to double values
    public double getLatDouble() { return lat != null ? Double.parseDouble(lat) : 0.0; }
    public double getLonDouble() { return lon != null ? Double.parseDouble(lon) : 0.0; }
    public double getHaeDouble() { return hae != null ? Double.parseDouble(hae) : 0.0; }
    public double getCeDouble() { return ce != null ? Double.parseDouble(ce) : 0.0; }
    public double getLeDouble() { return le != null ? Double.parseDouble(le) : 0.0; }
}

/**
 * Represents the detail element in a CoT event
 * This can contain arbitrary XML content that gets converted to a Map
 */
@XmlAccessorType(XmlAccessType.FIELD)
class CoTDetail {
    
    @XmlAnyElement(lax = true)
    private Object[] content;
    
    public CoTDetail() {}
    
    public Object[] getContent() { return content; }
    public void setContent(Object[] content) { this.content = content; }
    
    /**
     * Convert the detail content to a Map for use in Ditto documents
     */
    public Map<String, Object> toMap() {
        Map<String, Object> result = new HashMap<>();
        if (content != null) {
            for (Object item : content) {
                if (item instanceof org.w3c.dom.Element) {
                    org.w3c.dom.Element element = (org.w3c.dom.Element) item;
                    result.put(element.getTagName(), extractElementValue(element));
                }
            }
        }
        return result;
    }
    
    private Object extractElementValue(org.w3c.dom.Element element) {
        // Use the enhanced DetailConverter for better structure preservation
        DetailConverter converter = new DetailConverter();
        return converter.extractElementValue(element);
    }
    
    /**
     * Set detail content from a Map (for reverse conversion)
     */
    public void setFromMap(Map<String, Object> detailMap, org.w3c.dom.Document document) {
        if (detailMap == null || detailMap.isEmpty()) {
            this.content = null;
            return;
        }
        
        DetailConverter converter = new DetailConverter();
        org.w3c.dom.Element detailElement = converter.convertMapToDetailElement(detailMap, document);
        
        if (detailElement != null) {
            // Convert detail element children to content array
            java.util.List<Object> contentList = new java.util.ArrayList<>();
            org.w3c.dom.Node child = detailElement.getFirstChild();
            while (child != null) {
                if (child instanceof org.w3c.dom.Element) {
                    contentList.add(child);
                }
                child = child.getNextSibling();
            }
            this.content = contentList.toArray();
        }
    }
}