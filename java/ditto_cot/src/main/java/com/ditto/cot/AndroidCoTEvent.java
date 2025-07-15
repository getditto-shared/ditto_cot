package com.ditto.cot;

import java.time.Instant;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Android-compatible version of CoTEvent that doesn't use JAXB annotations
 * Represents a Cursor-on-Target (CoT) event parsed from XML
 */
public class AndroidCoTEvent {
    
    private String version;
    private String uid;
    private String type;
    private String time;
    private String start;
    private String stale;
    private String how;
    private AndroidCoTPoint point;
    private AndroidCoTDetail detail;
    
    // Constructors
    public AndroidCoTEvent() {}
    
    public AndroidCoTEvent(String version, String uid, String type, String time, String start, String stale, String how) {
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
    
    public AndroidCoTPoint getPoint() { return point; }
    public void setPoint(AndroidCoTPoint point) { this.point = point; }
    
    public AndroidCoTDetail getDetail() { return detail; }
    public void setDetail(AndroidCoTDetail detail) { this.detail = detail; }
    
    // Helper methods to access point data without exposing AndroidCoTPoint
    public String getPointLatitude() { return point != null ? point.getLat() : null; }
    public String getPointLongitude() { return point != null ? point.getLon() : null; }
    public String getPointHae() { return point != null ? point.getHae() : null; }
    public String getPointCe() { return point != null ? point.getCe() : null; }
    public String getPointLe() { return point != null ? point.getLe() : null; }
    
    // Helper method to access detail data without exposing AndroidCoTDetail
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
    
    /**
     * Convert CoT start time to microseconds since epoch (preserves full precision)
     */
    public long getStartMicros() {
        if (start == null) return 0;
        Instant instant = Instant.parse(start);
        return instant.getEpochSecond() * 1_000_000L + instant.getNano() / 1_000L;
    }
    
    /**
     * Convert CoT stale time to microseconds since epoch (preserves full precision)
     */
    public long getStaleMicros() {
        if (stale == null) return 0;
        Instant instant = Instant.parse(stale);
        return instant.getEpochSecond() * 1_000_000L + instant.getNano() / 1_000L;
    }
    
    /**
     * Convert CoT time to microseconds since epoch (preserves full precision)
     */
    public long getTimeMicros() {
        if (time == null) return 0;
        Instant instant = Instant.parse(time);
        return instant.getEpochSecond() * 1_000_000L + instant.getNano() / 1_000L;
    }
    
    /**
     * Convert CoT start time to milliseconds since epoch (preserves sub-second precision)
     */
    public long getStartMillis() {
        return start != null ? Instant.parse(start).toEpochMilli() : 0;
    }
    
    /**
     * Convert CoT stale time to milliseconds since epoch (preserves sub-second precision)
     */
    public long getStaleMillis() {
        return stale != null ? Instant.parse(stale).toEpochMilli() : 0;
    }
}

/**
 * Android-compatible version of CoTPoint that doesn't use JAXB annotations
 * Represents the point element in a CoT event
 */
class AndroidCoTPoint {
    
    private String lat;
    private String lon;
    private String hae;
    private String ce;
    private String le;
    
    // Constructors
    public AndroidCoTPoint() {}
    
    public AndroidCoTPoint(String lat, String lon, String hae, String ce, String le) {
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
 * Android-compatible version of CoTDetail that doesn't use JAXB annotations
 * Represents the detail element in a CoT event
 * This can contain arbitrary XML content that gets converted to a Map
 */
class AndroidCoTDetail {
    
    private List<org.w3c.dom.Element> anyElements;
    
    public AndroidCoTDetail() {
        this.anyElements = new ArrayList<>();
    }
    
    public List<org.w3c.dom.Element> getAnyElements() { return anyElements; }
    public void setAnyElements(List<org.w3c.dom.Element> anyElements) { this.anyElements = anyElements; }
    
    /**
     * Convert the detail content to a Map for use in Ditto documents
     */
    public Map<String, Object> toMap() {
        Map<String, Object> result = new HashMap<>();
        if (anyElements != null) {
            for (org.w3c.dom.Element element : anyElements) {
                result.put(element.getTagName(), extractElementValue(element));
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
            this.anyElements = null;
            return;
        }
        
        DetailConverter converter = new DetailConverter();
        org.w3c.dom.Element detailElement = converter.convertMapToDetailElement(detailMap, document);
        
        if (detailElement != null) {
            // Convert detail element children to anyElements list
            this.anyElements = new ArrayList<>();
            org.w3c.dom.Node child = detailElement.getFirstChild();
            while (child != null) {
                if (child instanceof org.w3c.dom.Element) {
                    anyElements.add((org.w3c.dom.Element) child);
                }
                child = child.getNextSibling();
            }
        }
    }
}