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
    private final Unmarshaller unmarshaller;
    private final Marshaller marshaller;
    private final ObjectMapper objectMapper;
    
    public CoTConverter() throws JAXBException {
        this.jaxbContext = JAXBContext.newInstance(CoTEvent.class);
        this.unmarshaller = jaxbContext.createUnmarshaller();
        this.marshaller = jaxbContext.createMarshaller();
        this.marshaller.setProperty(Marshaller.JAXB_FORMATTED_OUTPUT, true);
        this.objectMapper = new ObjectMapper();
    }
    
    /**
     * Parse CoT XML string into a CoTEvent object
     */
    public CoTEvent parseCoTXml(String xmlContent) throws JAXBException {
        StringReader reader = new StringReader(xmlContent);
        return (CoTEvent) unmarshaller.unmarshal(reader);
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
        String cotType = cotEvent.getType();
        
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
        doc.setMessage("CoT Event: " + cotEvent.getUid());
        doc.setRoom("cot-events");
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
        doc.setC(cotEvent.getUid() + ".xml");
        doc.setSz(1024.0); // Placeholder size
        doc.setFile(cotEvent.getUid());
        doc.setMime("application/xml");
        doc.setContentType("application/xml");
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
        
        doc.setN(cotEvent.getStartSeconds());
        doc.setO(cotEvent.getStaleSeconds());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        // Convert detail to map
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForChatDocument(ChatDocument doc, CoTEvent cotEvent) {
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
        
        doc.setN(cotEvent.getStartSeconds());
        doc.setO(cotEvent.getStaleSeconds());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForFileDocument(FileDocument doc, CoTEvent cotEvent) {
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
        
        doc.setN(cotEvent.getStartSeconds());
        doc.setO(cotEvent.getStaleSeconds());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForMapItemDocument(MapItemDocument doc, CoTEvent cotEvent) {
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
        
        doc.setN(cotEvent.getStartSeconds());
        doc.setO(cotEvent.getStaleSeconds());
        doc.setP(cotEvent.getHow() != null ? cotEvent.getHow() : "");
        doc.setW(cotEvent.getType() != null ? cotEvent.getType() : "");
        
        if (cotEvent.getDetail() != null) {
            doc.setR(cotEvent.getDetail().toMap());
        }
    }
    
    private void setCommonFieldsForGenericDocument(GenericDocument doc, CoTEvent cotEvent) {
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
        
        doc.setN(cotEvent.getStartSeconds());
        doc.setO(cotEvent.getStaleSeconds());
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
            cotType.startsWith("b-m-p-s-p-i") || // Sensor point of interest
            cotType.contains("api") ||
            cotType.contains("data")
        );
    }
    
    /**
     * Determine if CoT type should be converted to ChatDocument
     */
    private boolean isChatDocumentType(String cotType) {
        return cotType != null && (
            cotType.contains("chat") ||
            cotType.contains("message")
        );
    }
    
    /**
     * Determine if CoT type should be converted to FileDocument
     */
    private boolean isFileDocumentType(String cotType) {
        return cotType != null && (
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
            cotType.startsWith("a-u-")    // Unknown units
        );
    }
    
    /**
     * Extract callsign from CoT detail if available
     */
    private String extractCallsign(CoTEvent cotEvent) {
        if (cotEvent.getDetail() != null) {
            var detailMap = cotEvent.getDetail().toMap();
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
                                  doc.getB(), doc.getN(), doc.getO(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof ChatDocument) {
            ChatDocument doc = (ChatDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN(), doc.getO(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof FileDocument) {
            FileDocument doc = (FileDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN(), doc.getO(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof MapItemDocument) {
            MapItemDocument doc = (MapItemDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN(), doc.getO(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        } else if (document instanceof GenericDocument) {
            GenericDocument doc = (GenericDocument) document;
            setCommonCoTEventFields(cotEvent, doc.getId(), doc.getW(), doc.getG(), 
                                  doc.getB(), doc.getN(), doc.getO(), doc.getP(),
                                  doc.getJ(), doc.getL(), doc.getI(), doc.getH(), doc.getK(),
                                  doc.getR());
        }
    }
    
    private void setCommonCoTEventFields(CoTEvent cotEvent, String id, String type, String version,
                                       Double timeMillis, Integer startSeconds, Integer staleSeconds,
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
        
        if (startSeconds != null) {
            Instant startInstant = Instant.ofEpochSecond(startSeconds);
            cotEvent.setStart(DateTimeFormatter.ISO_INSTANT.format(startInstant));
        }
        
        if (staleSeconds != null) {
            Instant staleInstant = Instant.ofEpochSecond(staleSeconds);
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
     * Convert a CoT document to Map for Ditto storage
     */
    public Map<String, Object> convertDocumentToMap(Object document) {
        return objectMapper.convertValue(document, new TypeReference<Map<String, Object>>() {});
    }

    /**
     * Convert a Map from Ditto back to a CoT document
     */
    public <T> T convertMapToDocument(Map<String, Object> map, Class<T> documentClass) {
        return objectMapper.convertValue(map, documentClass);
    }

    /**
     * Convert JSON string from Ditto back to a CoT document
     */
    public <T> T convertJsonToDocument(String json, Class<T> documentClass) throws JsonProcessingException {
        return objectMapper.readValue(json, documentClass);
    }
}