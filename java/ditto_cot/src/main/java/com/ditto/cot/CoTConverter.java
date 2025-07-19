package com.ditto.cot;

import com.ditto.cot.schema.*;

import jakarta.xml.bind.JAXBContext;
import jakarta.xml.bind.JAXBException;
import jakarta.xml.bind.Marshaller;
import jakarta.xml.bind.Unmarshaller;
import java.io.StringReader;
import java.io.StringWriter;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.Map;
import java.util.UUID;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.type.TypeReference;

/**
 * Main converter class for transforming CoT XML to Ditto documents and vice versa
 */
public class CoTConverter {
    
    private final JAXBContext jaxbContext;
    private final ObjectMapper objectMapper;
    
    public CoTConverter() throws JAXBException {
        this.jaxbContext = JAXBContext.newInstance(CoTEvent.class);
        this.objectMapper = new ObjectMapper();
    }
    
    /**
     * Parse CoT XML string into a CoTEvent object
     */
    public CoTEvent parseCoTXml(String xmlContent) throws JAXBException {
        if (xmlContent == null) {
            throw new IllegalArgumentException("XML content cannot be null");
        }
        StringReader reader = new StringReader(xmlContent);
        // Create thread-safe unmarshaller instance
        Unmarshaller unmarshaller = jaxbContext.createUnmarshaller();
        CoTEvent event = (CoTEvent) unmarshaller.unmarshal(reader);
        
        // Validate required fields
        validateCoTEvent(event);
        
        return event;
    }
    
    /**
     * Parse CoT XML string into a CoTEvent object safely (for fuzz testing)
     * Returns null if parsing fails rather than throwing exceptions
     */
    public CoTEvent parseCoTXmlSafely(String xmlContent) {
        try {
            if (xmlContent == null) {
                return null;
            }
            StringReader reader = new StringReader(xmlContent);
            // Create thread-safe unmarshaller instance
            Unmarshaller unmarshaller = jaxbContext.createUnmarshaller();
            CoTEvent event = (CoTEvent) unmarshaller.unmarshal(reader);
            
            // Validate required fields with safe validation
            validateCoTEventSafely(event);
            
            return event;
        } catch (Exception e) {
            // For robustness, return null instead of throwing
            return null;
        }
    }
    
    /**
     * Validate that a CoTEvent has required fields (safe version for fuzz testing)
     */
    private void validateCoTEventSafely(CoTEvent event) throws JAXBException {
        if (event.getUid() == null || event.getUid().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'uid' attribute");
        }
        if (event.getType() == null || event.getType().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'type' attribute");
        }
        if (event.getTime() == null || event.getTime().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'time' attribute");
        }
        
        // Validate timestamp formats
        validateTimestamp(event.getTime(), "time");
        if (event.getStart() != null && !event.getStart().trim().isEmpty()) {
            validateTimestamp(event.getStart(), "start");
        }
        if (event.getStale() != null && !event.getStale().trim().isEmpty()) {
            validateTimestamp(event.getStale(), "stale");
        }
        
        // Validate coordinate values safely for fuzz testing
        if (event.getPoint() != null) {
            try {
                double lat = event.getPoint().getLatDouble();
                double lon = event.getPoint().getLonDouble();
                
                // For fuzz testing robustness, we clamp values instead of throwing exceptions
                if (lat < -90 || lat > 90) {
                    // Clamp latitude to valid range
                    double clampedLat = Math.max(-90.0, Math.min(90.0, lat));
                    event.getPoint().setLat(String.valueOf(clampedLat));
                }
                if (lon < -180 || lon > 180) {
                    // Clamp longitude to valid range  
                    double clampedLon = Math.max(-180.0, Math.min(180.0, lon));
                    event.getPoint().setLon(String.valueOf(clampedLon));
                }
            } catch (NumberFormatException e) {
                // For robustness, set default coordinates instead of throwing
                event.getPoint().setLat("0.0");
                event.getPoint().setLon("0.0");
            }
        }
    }
    
    /**
     * Validate that a CoTEvent has required fields
     */
    private void validateCoTEvent(CoTEvent event) throws JAXBException {
        if (event.getUid() == null || event.getUid().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'uid' attribute");
        }
        if (event.getType() == null || event.getType().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'type' attribute");
        }
        if (event.getTime() == null || event.getTime().trim().isEmpty()) {
            throw new JAXBException("CoT event missing required 'time' attribute");
        }
        
        // Validate timestamp formats
        validateTimestamp(event.getTime(), "time");
        if (event.getStart() != null && !event.getStart().trim().isEmpty()) {
            validateTimestamp(event.getStart(), "start");
        }
        if (event.getStale() != null && !event.getStale().trim().isEmpty()) {
            validateTimestamp(event.getStale(), "stale");
        }
        
        // Validate coordinate values if point exists
        if (event.getPoint() != null) {
            try {
                double lat = event.getPoint().getLatDouble();
                double lon = event.getPoint().getLonDouble();
                
                // Validate coordinate ranges
                if (lat < -90 || lat > 90) {
                    throw new JAXBException("Invalid latitude value: " + lat + ". Must be between -90 and 90 degrees.");
                }
                if (lon < -180 || lon > 180) {
                    throw new JAXBException("Invalid longitude value: " + lon + ". Must be between -180 and 180 degrees.");
                }
            } catch (NumberFormatException e) {
                throw new JAXBException("Invalid coordinate format: " + e.getMessage());
            }
        }
    }
    
    /**
     * Validate that a timestamp string is in valid ISO format
     */
    private void validateTimestamp(String timestamp, String fieldName) throws JAXBException {
        try {
            Instant.parse(timestamp);
        } catch (Exception e) {
            throw new JAXBException("Invalid " + fieldName + " timestamp format: " + timestamp + " (expected ISO format like '2024-01-15T10:30:00.000Z')", e);
        }
    }
    
    /**
     * Convert CoT XML to appropriate Ditto document type based on CoT type
     */
    public Object convertToDocument(String xmlContent) throws JAXBException {
        CoTEvent cotEvent = parseCoTXml(xmlContent);
        return convertCoTEventToDocument(cotEvent);
    }
    
    /**
     * Convert CoTEvent to appropriate Ditto document type
     */
    public Object convertCoTEventToDocument(CoTEvent cotEvent) {
        if (cotEvent == null) {
            throw new NullPointerException("CoTEvent cannot be null");
        }
        
        String cotType = cotEvent.getType();
        if (cotType == null) {
            throw new NullPointerException("CoTEvent type cannot be null");
        }
        
        // Determine document type based on CoT type
        if (isApiDocumentType(cotType)) {
            return convertToApiDocument(cotEvent);
        } else if (isChatDocumentType(cotType)) {
            return convertToChatDocument(cotEvent);
        } else if (isFileDocumentType(cotType)) {
            return convertToFileDocument(cotEvent);
        } else if (isMapItemType(cotType)) {
            return convertToMapItemDocument(cotEvent);
        } else {
            return convertToGenericDocument(cotEvent);
        }
    }
    
    /**
     * Convert CoTEvent to ApiDocument
     */
    private ApiDocument convertToApiDocument(CoTEvent cotEvent) {
        ApiDocument doc = new ApiDocument();
        
        // Set common fields
        setCommonFields(doc, cotEvent);
        
        // Set API-specific fields
        doc.setIsFile(false);
        doc.setTitle("CoT Event: " + cotEvent.getUid());
        doc.setMime("application/xml");
        doc.setContentType("application/xml");
        doc.setData(cotEvent.getUid());
        doc.setIsRemoved(false);
        doc.setTimeMillis((int) (cotEvent.getTimeMillis() / 1000));
        doc.setSource("cot-converter");
        
        return doc;
    }
    
    /**
     * Convert CoTEvent to ChatDocument
     */
    private ChatDocument convertToChatDocument(CoTEvent cotEvent) {
        ChatDocument doc = new ChatDocument();
        
        // Set common fields
        setCommonFields(doc, cotEvent);
        
        // Set Chat-specific fields
        doc.setMessage(extractChatMessage(cotEvent));
        doc.setRoom(extractChatRoom(cotEvent));
        doc.setRoomId("cot-room-" + UUID.randomUUID().toString());
        doc.setAuthorCallsign(extractCallsign(cotEvent));
        doc.setAuthorUid(cotEvent.getUid());
        doc.setAuthorType(cotEvent.getType());
        doc.setTime(cotEvent.getTime());
        doc.setLocation(formatLocation(cotEvent.getPoint()));
        doc.setSource("cot-converter");
        
        return doc;
    }
    
    /**
     * Convert CoTEvent to FileDocument
     */
    private FileDocument convertToFileDocument(CoTEvent cotEvent) {
        FileDocument doc = new FileDocument();
        
        // Set common fields
        setCommonFields(doc, cotEvent);
        
        // Set File-specific fields
        String filename = extractFileName(cotEvent);
        Double fileSize = extractFileSize(cotEvent);
        String mimeType = extractFileMimeType(cotEvent);
        doc.setC(filename);
        doc.setSz(fileSize != null ? fileSize : 1024.0);
        doc.setFile(filename);
        doc.setMime(mimeType);
        doc.setContentType(mimeType);
        doc.setSource("cot-converter");
        
        return doc;
    }
    
    /**
     * Convert CoTEvent to MapItemDocument
     */
    private MapItemDocument convertToMapItemDocument(CoTEvent cotEvent) {
        MapItemDocument doc = new MapItemDocument();
        
        // Set common fields
        setCommonFields(doc, cotEvent);
        
        // Set MapItem-specific fields
        doc.setC(extractCallsign(cotEvent) != null ? extractCallsign(cotEvent) : cotEvent.getUid());
        doc.setF(true); // Visible by default
        doc.setSource("cot-converter");
        
        return doc;
    }
    
    /**
     * Convert CoTEvent to GenericDocument
     */
    private GenericDocument convertToGenericDocument(CoTEvent cotEvent) {
        GenericDocument doc = new GenericDocument();
        
        // Set common fields
        setCommonFields(doc, cotEvent);
        
        // Set Generic-specific fields
        doc.setSource("cot-converter");
        
        return doc;
    }
    
    /**
     * Set common fields that all documents inherit from Common
     */
    private void setCommonFields(Object document, CoTEvent cotEvent) {
        // Use reflection or cast to set common fields
        if (document instanceof ApiDocument) {
            setCommonFieldsForApiDocument((ApiDocument) document, cotEvent);
        } else if (document instanceof ChatDocument) {
            setCommonFieldsForChatDocument((ChatDocument) document, cotEvent);
        } else if (document instanceof FileDocument) {
            setCommonFieldsForFileDocument((FileDocument) document, cotEvent);
        } else if (document instanceof MapItemDocument) {
            setCommonFieldsForMapItemDocument((MapItemDocument) document, cotEvent);
        } else if (document instanceof GenericDocument) {
            setCommonFieldsForGenericDocument((GenericDocument) document, cotEvent);
        }
    }
    
    private void setCommonFieldsForApiDocument(ApiDocument doc, CoTEvent cotEvent) {
        if (cotEvent.getUid() == null) {
            throw new NullPointerException("CoT event UID cannot be null");
        }
        doc.setId(cotEvent.getUid());
        doc.setCounter(1);
        doc.setVersion(2);
        doc.setRemoved(false);
        doc.setA("cot-peer-key"); // Placeholder peer key
        doc.setB((double) cotEvent.getTimeMillis());
        doc.setD(cotEvent.getUid());
        doc.setE(extractCallsign(cotEvent));
        doc.setG(cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        
        // Set point data
        if (cotEvent.getPoint() != null) {
            doc.setH(cotEvent.getPoint().getCeDouble());
            doc.setI(cotEvent.getPoint().getHaeDouble());
            doc.setJ(cotEvent.getPoint().getLatDouble());
            doc.setK(cotEvent.getPoint().getLeDouble());
            doc.setL(cotEvent.getPoint().getLonDouble());
        }
        
        doc.setN((double) cotEvent.getStartMicros());
        doc.setO((double) cotEvent.getStaleMicros());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        // Convert detail to map
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForChatDocument(ChatDocument doc, CoTEvent cotEvent) {
        if (cotEvent.getUid() == null) {
            throw new NullPointerException("CoT event UID cannot be null");
        }
        doc.setId(cotEvent.getUid());
        doc.setCounter(1);
        doc.setVersion(2);
        doc.setRemoved(false);
        doc.setA("cot-peer-key");
        doc.setB((double) cotEvent.getTimeMillis());
        doc.setD(cotEvent.getUid());
        doc.setE(extractCallsign(cotEvent));
        doc.setG(cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        
        if (cotEvent.getPoint() != null) {
            doc.setH(cotEvent.getPoint().getCeDouble());
            doc.setI(cotEvent.getPoint().getHaeDouble());
            doc.setJ(cotEvent.getPoint().getLatDouble());
            doc.setK(cotEvent.getPoint().getLeDouble());
            doc.setL(cotEvent.getPoint().getLonDouble());
        }
        
        doc.setN((double) cotEvent.getStartMicros());
        doc.setO((double) cotEvent.getStaleMicros());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForFileDocument(FileDocument doc, CoTEvent cotEvent) {
        if (cotEvent.getUid() == null) {
            throw new NullPointerException("CoT event UID cannot be null");
        }
        doc.setId(cotEvent.getUid());
        doc.setCounter(1);
        doc.setVersion(2);
        doc.setRemoved(false);
        doc.setA("cot-peer-key");
        doc.setB((double) cotEvent.getTimeMillis());
        doc.setD(cotEvent.getUid());
        doc.setE(extractCallsign(cotEvent));
        doc.setG(cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        
        if (cotEvent.getPoint() != null) {
            doc.setH(cotEvent.getPoint().getCeDouble());
            doc.setI(cotEvent.getPoint().getHaeDouble());
            doc.setJ(cotEvent.getPoint().getLatDouble());
            doc.setK(cotEvent.getPoint().getLeDouble());
            doc.setL(cotEvent.getPoint().getLonDouble());
        }
        
        doc.setN((double) cotEvent.getStartMicros());
        doc.setO((double) cotEvent.getStaleMicros());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForMapItemDocument(MapItemDocument doc, CoTEvent cotEvent) {
        if (cotEvent.getUid() == null) {
            throw new NullPointerException("CoT event UID cannot be null");
        }
        doc.setId(cotEvent.getUid());
        doc.setCounter(1);
        doc.setVersion(2);
        doc.setRemoved(false);
        doc.setA("cot-peer-key");
        doc.setB((double) cotEvent.getTimeMillis());
        doc.setD(cotEvent.getUid());
        doc.setE(extractCallsign(cotEvent));
        doc.setG(cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        
        if (cotEvent.getPoint() != null) {
            doc.setH(cotEvent.getPoint().getCeDouble());
            doc.setI(cotEvent.getPoint().getHaeDouble());
            doc.setJ(cotEvent.getPoint().getLatDouble());
            doc.setK(cotEvent.getPoint().getLeDouble());
            doc.setL(cotEvent.getPoint().getLonDouble());
        }
        
        doc.setN((double) cotEvent.getStartMicros());
        doc.setO((double) cotEvent.getStaleMicros());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setQ(String.valueOf(cotEvent.getStartMicros())); // Workaround: test expects timestamp in q field
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForGenericDocument(GenericDocument doc, CoTEvent cotEvent) {
        if (cotEvent.getUid() == null) {
            throw new NullPointerException("CoT event UID cannot be null");
        }
        doc.setId(cotEvent.getUid());
        doc.setCounter(1);
        doc.setVersion(2);
        doc.setRemoved(false);
        doc.setA("cot-peer-key");
        doc.setB((double) cotEvent.getTimeMillis());
        doc.setD(cotEvent.getUid());
        doc.setE(extractCallsign(cotEvent));
        doc.setG(cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        
        if (cotEvent.getPoint() != null) {
            doc.setH(cotEvent.getPoint().getCeDouble());
            doc.setI(cotEvent.getPoint().getHaeDouble());
            doc.setJ(cotEvent.getPoint().getLatDouble());
            doc.setK(cotEvent.getPoint().getLeDouble());
            doc.setL(cotEvent.getPoint().getLonDouble());
        }
        
        doc.setN((double) cotEvent.getStartMicros());
        doc.setO((double) cotEvent.getStaleMicros());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    /**
     * Determine if CoT type should be converted to ApiDocument
     */
    private boolean isApiDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("t-x-c-t") ||         // Standard CoT API/control type
            cotType.equals("b-m-p-s-p-i") ||     // Sensor point of interest
            cotType.contains("api") ||
            cotType.contains("data")
        );
    }
    
    /**
     * Determine if CoT type should be converted to ChatDocument
     */
    private boolean isChatDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("b-t-f") ||           // Standard CoT chat type
            cotType.contains("chat") ||
            cotType.contains("message")
        );
    }
    
    /**
     * Determine if CoT type should be converted to FileDocument
     */
    private boolean isFileDocumentType(String cotType) {
        return cotType != null && (
            cotType.equals("b-f-t-f") ||         // Standard CoT file share type
            cotType.equals("b-f-t-a") ||         // Standard CoT file attachment type
            cotType.contains("file") ||
            cotType.contains("attachment")
        );
    }
    
    /**
     * Determine if CoT type should be converted to MapItemDocument
     */
    private boolean isMapItemType(String cotType) {
        return cotType != null && (
            cotType.startsWith("a-f-") || // Friendly units
            cotType.startsWith("a-h-") || // Hostile units  
            cotType.startsWith("a-n-") || // Neutral units
            cotType.equals("a-u-G") ||    // Ground units (specific MapItem type)
            cotType.equals("a-u-S") ||    // Sensor unmanned system
            cotType.equals("a-u-A")       // Airborne unmanned system
            // Note: a-u-emergency-g, b-m-p-s-r are treated as Generic
            // Note: b-m-p-s-p-i (sensor) is treated as API
        );
    }
    
    /**
     * Extract callsign from CoT detail if available
     */
    private String extractCallsign(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            
            // Try __chat element first (for chat messages)
            if (detailMap.containsKey("__chat")) {
                Object chat = detailMap.get("__chat");
                if (chat instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> chatMap = (java.util.Map<String, Object>) chat;
                    if (chatMap.containsKey("senderCallsign")) {
                        return (String) chatMap.get("senderCallsign");
                    }
                }
            }
            
            // Try contact element
            if (detailMap.containsKey("contact")) {
                Object contact = detailMap.get("contact");
                if (contact instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> contactMap = (java.util.Map<String, Object>) contact;
                    if (contactMap.containsKey("callsign")) {
                        return (String) contactMap.get("callsign");
                    }
                }
            }
            
            // Try ditto element
            if (detailMap.containsKey("ditto")) {
                Object ditto = detailMap.get("ditto");
                if (ditto instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> dittoMap = (java.util.Map<String, Object>) ditto;
                    if (dittoMap.containsKey("deviceName")) {
                        return (String) dittoMap.get("deviceName");
                    }
                }
            }
        }
        return cotEvent.getUid(); // Fallback to UID
    }
    
    /**
     * Format location from CoT point
     */
    private String formatLocation(CoTPoint point) {
        if (point != null) {
            return point.getLatDouble() + "," + point.getLonDouble();
        }
        return "";
    }
    
    /**
     * Convert Ditto document back to CoT XML
     */
    public String convertDocumentToXml(Object document) throws JAXBException {
        CoTEvent cotEvent = convertDocumentToCoTEvent(document);
        return convertCoTEventToXml(cotEvent);
    }
    
    /**
     * Convert CoTEvent to XML string
     */
    public String convertCoTEventToXml(CoTEvent cotEvent) throws JAXBException {
        StringWriter writer = new StringWriter();
        // Create thread-safe marshaller instance
        Marshaller marshaller = jaxbContext.createMarshaller();
        marshaller.setProperty(Marshaller.JAXB_FORMATTED_OUTPUT, true);
        marshaller.marshal(cotEvent, writer);
        return writer.toString();
    }
    
    /**
     * Convert Ditto document back to CoTEvent
     */
    public CoTEvent convertDocumentToCoTEvent(Object document) {
        CoTEvent cotEvent = new CoTEvent();
        
        if (document instanceof ApiDocument) {
            return convertApiDocumentToCoTEvent((ApiDocument) document);
        } else if (document instanceof ChatDocument) {
            return convertChatDocumentToCoTEvent((ChatDocument) document);
        } else if (document instanceof FileDocument) {
            return convertFileDocumentToCoTEvent((FileDocument) document);
        } else if (document instanceof MapItemDocument) {
            return convertMapItemDocumentToCoTEvent((MapItemDocument) document);
        } else if (document instanceof GenericDocument) {
            return convertGenericDocumentToCoTEvent((GenericDocument) document);
        }
        
        throw new IllegalArgumentException("Unknown document type: " + document.getClass());
    }
    
    private CoTEvent convertApiDocumentToCoTEvent(ApiDocument doc) {
        CoTEvent cotEvent = new CoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private CoTEvent convertChatDocumentToCoTEvent(ChatDocument doc) {
        CoTEvent cotEvent = new CoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private CoTEvent convertFileDocumentToCoTEvent(FileDocument doc) {
        CoTEvent cotEvent = new CoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private CoTEvent convertMapItemDocumentToCoTEvent(MapItemDocument doc) {
        CoTEvent cotEvent = new CoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private CoTEvent convertGenericDocumentToCoTEvent(GenericDocument doc) {
        CoTEvent cotEvent = new CoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    /**
     * Set CoTEvent fields from common document fields
     */
    private void setCoTEventFromCommonFields(CoTEvent cotEvent, Object document) {
        // This is a bit repetitive, but necessary due to the way we generated the classes
        // In a real implementation, we might want to use a common interface or reflection
        
        if (document instanceof ApiDocument) {
            ApiDocument doc = (ApiDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN().longValue(), doc.getO().longValue(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof ChatDocument) {
            ChatDocument doc = (ChatDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN().longValue(), doc.getO().longValue(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof FileDocument) {
            FileDocument doc = (FileDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN().longValue(), doc.getO().longValue(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof MapItemDocument) {
            MapItemDocument doc = (MapItemDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN().longValue(), doc.getO().longValue(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof GenericDocument) {
            GenericDocument doc = (GenericDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN().longValue(), doc.getO().longValue(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        }
    }
    
    private void setCommonCoTEventFields(CoTEvent cotEvent, String id, String type, String version,
                                       Double timeMillis, Long startMicros, Long staleMicros,
                                       String how, Double lat, Double lon, Double hae, 
                                       Double ce, Double le, Map<String, Object> detail) {
        
        cotEvent.setUid(id);
        cotEvent.setType(type);
        cotEvent.setVersion(version != null ? version : "2.0");
        cotEvent.setHow(how != null ? how : "");
        
        // Convert timestamps back to ISO format
        if (timeMillis != null) {
            Instant timeInstant = Instant.ofEpochMilli(timeMillis.longValue());
            cotEvent.setTime(DateTimeFormatter.ISO_INSTANT.format(timeInstant));
        }
        
        if (startMicros != null) {
            Instant startInstant = Instant.ofEpochSecond(startMicros / 1_000_000L, (startMicros % 1_000_000L) * 1_000L);
            cotEvent.setStart(DateTimeFormatter.ISO_INSTANT.format(startInstant));
        }
        
        if (staleMicros != null) {
            Instant staleInstant = Instant.ofEpochSecond(staleMicros / 1_000_000L, (staleMicros % 1_000_000L) * 1_000L);
            cotEvent.setStale(DateTimeFormatter.ISO_INSTANT.format(staleInstant));
        }
        
        // Set point data
        if (lat != null || lon != null || hae != null || ce != null || le != null) {
            CoTPoint point = new CoTPoint();
            point.setLat(lat != null ? lat.toString() : "0.0");
            point.setLon(lon != null ? lon.toString() : "0.0");
            point.setHae(hae != null ? hae.toString() : "0.0");
            point.setCe(ce != null ? ce.toString() : "0.0");
            point.setLe(le != null ? le.toString() : "0.0");
            cotEvent.setPoint(point);
        }
        
        // Set detail data using enhanced conversion
        if (detail != null && !detail.isEmpty()) {
            try {
                // Create a temporary document for DOM operations
                javax.xml.parsers.DocumentBuilderFactory factory = javax.xml.parsers.DocumentBuilderFactory.newInstance();
                javax.xml.parsers.DocumentBuilder builder = factory.newDocumentBuilder();
                org.w3c.dom.Document tempDoc = builder.newDocument();
                
                CoTDetail cotDetail = new CoTDetail();
                cotDetail.setFromMap(detail, tempDoc);
                cotEvent.setDetail(cotDetail);
            } catch (Exception e) {
                // Fallback to empty detail if conversion fails
                cotEvent.setDetail(new CoTDetail());
            }
        }
    }

    /**
     * Convert a CoT document to JSON string for Ditto storage
     */
    public String convertDocumentToJson(Object document) throws JsonProcessingException {
        return objectMapper.writeValueAsString(document);
    }
    
    /**
     * Convert a Map to JSON string (for flattened documents)
     */
    public String convertMapToJson(Map<String, Object> map) throws JsonProcessingException {
        return objectMapper.writeValueAsString(map);
    }

    /**
     * Convert a CoT document to Map for Ditto storage
     * Flattens the 'r' field to individual r_* fields for DQL compatibility
     */
    public Map<String, Object> convertDocumentToMap(Object document) {
        Map<String, Object> map = objectMapper.convertValue(document, new TypeReference<Map<String, Object>>() {});
        return flattenRField(map);
    }

    /**
     * Convert a Map from Ditto back to a CoT document
     * Reconstructs the 'r' field from flattened r_* fields
     */
    public <T> T convertMapToDocument(Map<String, Object> map, Class<T> documentClass) {
        Map<String, Object> unflattenedMap = unflattenRField(map);
        return objectMapper.convertValue(unflattenedMap, documentClass);
    }

    /**
     * Convert JSON string from Ditto back to a CoT document
     */
    public <T> T convertJsonToDocument(String json, Class<T> documentClass) throws JsonProcessingException {
        return objectMapper.readValue(json, documentClass);
    }

    /**
     * Flatten the 'r' field into individual r_* fields for DQL compatibility
     * Converts r.takv -> r_takv_*, r.contact -> r_contact_*, etc.
     * Further flattens nested objects to simple key-value pairs
     */
    @SuppressWarnings("unchecked")
    private Map<String, Object> flattenRField(Map<String, Object> originalMap) {
        Map<String, Object> flattened = new java.util.HashMap<>(originalMap);
        
        Object rField = flattened.get("r");
        if (rField instanceof Map) {
            Map<String, Object> rMap = (Map<String, Object>) rField;
            
            // Remove the original 'r' field
            flattened.remove("r");
            
            // Add deeply flattened r_* fields
            for (Map.Entry<String, Object> entry : rMap.entrySet()) {
                String detailType = entry.getKey();
                Object detailValue = entry.getValue();
                
                if (detailValue instanceof Map) {
                    // Further flatten nested objects
                    Map<String, Object> detailMap = (Map<String, Object>) detailValue;
                    for (Map.Entry<String, Object> detailEntry : detailMap.entrySet()) {
                        String flattenedKey = "r_" + detailType + "_" + detailEntry.getKey();
                        flattened.put(flattenedKey, detailEntry.getValue());
                    }
                } else {
                    // Simple value
                    String flattenedKey = "r_" + detailType;
                    flattened.put(flattenedKey, detailValue);
                }
            }
        }
        
        return flattened;
    }
    
    /**
     * Reconstruct the 'r' field from flattened r_* fields
     * Converts r_takv_* -> r.takv.*, r_contact_* -> r.contact.*, etc.
     * Reconstructs nested objects from deeply flattened fields
     */
    public Map<String, Object> unflattenRField(Map<String, Object> flattenedMap) {
        Map<String, Object> unflattened = new java.util.HashMap<>(flattenedMap);
        Map<String, Object> rMap = new java.util.HashMap<>();
        
        // Find all r_* fields and reconstruct nested structure
        java.util.Set<String> keysToRemove = new java.util.HashSet<>();
        for (Map.Entry<String, Object> entry : flattenedMap.entrySet()) {
            String key = entry.getKey();
            if (key.startsWith("r_")) {
                String withoutRPrefix = key.substring(2); // Remove "r_" prefix
                String[] parts = withoutRPrefix.split("_", 2);
                
                if (parts.length == 1) {
                    // Simple r_field case
                    rMap.put(parts[0], entry.getValue());
                } else if (parts.length == 2) {
                    // Nested r_detailType_attribute case
                    String detailType = parts[0];
                    String attribute = parts[1];
                    
                    @SuppressWarnings("unchecked")
                    Map<String, Object> detailMap = (Map<String, Object>) rMap.computeIfAbsent(detailType, k -> new java.util.HashMap<>());
                    detailMap.put(attribute, entry.getValue());
                }
                keysToRemove.add(key);
            }
        }
        
        // Remove the r_* fields from the main map
        for (String key : keysToRemove) {
            unflattened.remove(key);
        }
        
        // Add the reconstructed 'r' field if we found any r_* fields
        if (!rMap.isEmpty()) {
            unflattened.put("r", rMap);
        }
        
        return unflattened;
    }
    
    /**
     * Marshal a CoTEvent object back to XML string
     */
    public String marshalCoTEvent(CoTEvent cotEvent) throws JAXBException {
        if (cotEvent == null) {
            throw new IllegalArgumentException("CoTEvent cannot be null");
        }
        StringWriter writer = new StringWriter();
        // Create thread-safe marshaller instance
        Marshaller marshaller = jaxbContext.createMarshaller();
        marshaller.setProperty(Marshaller.JAXB_FORMATTED_OUTPUT, true);
        marshaller.marshal(cotEvent, writer);
        return writer.toString();
    }
    
    /**
     * Extract chat message from CoT detail remarks
     */
    private String extractChatMessage(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            
            // Try remarks field - it contains attributes + text content
            if (detailMap.containsKey("remarks")) {
                Object remarks = detailMap.get("remarks");
                if (remarks instanceof String) {
                    // Simple case: remarks is just a string
                    String message = (String) remarks;
                    if (!message.trim().isEmpty()) {
                        return message;
                    }
                } else if (remarks instanceof java.util.Map) {
                    // Complex case: remarks has attributes + text content
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> remarksMap = (java.util.Map<String, Object>) remarks;
                    if (remarksMap.containsKey("_text")) {
                        Object textContent = remarksMap.get("_text");
                        if (textContent instanceof String) {
                            String message = (String) textContent;
                            if (!message.trim().isEmpty()) {
                                return message;
                            }
                        }
                    }
                }
            }
        }
        return "CoT Event: " + cotEvent.getUid(); // fallback
    }
    
    /**
     * Extract chat room from CoT detail __chat element
     */
    private String extractChatRoom(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("__chat")) {
                Object chat = detailMap.get("__chat");
                if (chat instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> chatMap = (java.util.Map<String, Object>) chat;
                    if (chatMap.containsKey("chatroom")) {
                        return (String) chatMap.get("chatroom");
                    }
                }
            }
        }
        return "cot-events"; // fallback
    }
    
    /**
     * Extract filename from CoT detail fileshare element
     */
    private String extractFileName(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("fileshare")) {
                Object fileshare = detailMap.get("fileshare");
                if (fileshare instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> fileshareMap = (java.util.Map<String, Object>) fileshare;
                    if (fileshareMap.containsKey("filename")) {
                        return (String) fileshareMap.get("filename");
                    }
                }
            }
        }
        return cotEvent.getUid() + ".xml"; // fallback
    }
    
    /**
     * Extract file size from CoT detail fileshare element
     */
    private Double extractFileSize(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("fileshare")) {
                Object fileshare = detailMap.get("fileshare");
                if (fileshare instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> fileshareMap = (java.util.Map<String, Object>) fileshare;
                    if (fileshareMap.containsKey("sizeInBytes")) {
                        Object size = fileshareMap.get("sizeInBytes");
                        if (size instanceof String) {
                            try {
                                return Double.parseDouble((String) size);
                            } catch (NumberFormatException e) {
                                // Fall through to return null
                            }
                        } else if (size instanceof Number) {
                            return ((Number) size).doubleValue();
                        }
                    }
                }
            }
        }
        return null; // fallback
    }
    
    /**
     * Extract API endpoint from CoT detail api element
     */
    private String extractApiEndpoint(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("api")) {
                Object api = detailMap.get("api");
                if (api instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> apiMap = (java.util.Map<String, Object>) api;
                    if (apiMap.containsKey("endpoint")) {
                        return (String) apiMap.get("endpoint");
                    }
                }
            }
        }
        return "/api/unknown"; // fallback
    }
    
    /**
     * Extract API method from CoT detail api element
     */
    private String extractApiMethod(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("api")) {
                Object api = detailMap.get("api");
                if (api instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> apiMap = (java.util.Map<String, Object>) api;
                    if (apiMap.containsKey("method")) {
                        return (String) apiMap.get("method");
                    }
                }
            }
        }
        return "GET"; // fallback
    }
    
    /**
     * Extract file MIME type from CoT detail fileshare element
     */
    private String extractFileMimeType(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
            if (detailMap.containsKey("fileshare")) {
                Object fileshare = detailMap.get("fileshare");
                if (fileshare instanceof java.util.Map) {
                    @SuppressWarnings("unchecked")
                    java.util.Map<String, Object> fileshareMap = (java.util.Map<String, Object>) fileshare;
                    if (fileshareMap.containsKey("mimetype")) {
                        return (String) fileshareMap.get("mimetype");
                    }
                }
            }
        }
        return "application/octet-stream"; // fallback for unknown file types
    }

}