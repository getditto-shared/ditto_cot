package com.ditto.cot;

import com.ditto.cot.schema.*;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;
import org.w3c.dom.NodeList;
import org.xml.sax.InputSource;
import org.xml.sax.SAXException;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import javax.xml.transform.OutputKeys;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerException;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;

import java.io.IOException;
import java.io.StringReader;
import java.io.StringWriter;
import java.time.Instant;
import java.time.format.DateTimeFormatter;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.core.type.TypeReference;

/**
 * Android-compatible converter class for transforming CoT XML to Ditto documents and vice versa.
 * Uses standard javax.xml APIs that are available on Android instead of JAXB.
 */
public class AndroidCoTConverter {
    
    private final DocumentBuilderFactory documentBuilderFactory;
    private final TransformerFactory transformerFactory;
    private final ObjectMapper objectMapper;
    
    public AndroidCoTConverter() {
        this.documentBuilderFactory = DocumentBuilderFactory.newInstance();
        this.transformerFactory = TransformerFactory.newInstance();
        this.objectMapper = new ObjectMapper();
    }
    
    /**
     * Flatten the detail map into r_* fields for DQL compatibility.
     * Converts nested structures like {takv: {os: "35"}} to r_takv_os: "35"
     */
    private Map<String, Object> flattenDetailToRFields(Map<String, Object> detailMap) {
        Map<String, Object> flattened = new HashMap<>();
        flattenRecursive("r", detailMap, flattened);
        return flattened;
    }
    
    private void flattenRecursive(String prefix, Map<String, Object> map, Map<String, Object> result) {
        for (Map.Entry<String, Object> entry : map.entrySet()) {
            String key = entry.getKey();
            Object value = entry.getValue();
            String newKey = prefix + "_" + key;
            
            if (value instanceof Map) {
                @SuppressWarnings("unchecked")
                Map<String, Object> nestedMap = (Map<String, Object>) value;
                flattenRecursive(newKey, nestedMap, result);
            } else {
                result.put(newKey, value);
            }
        }
    }
    
    /**
     * Parse CoT XML string into an AndroidCoTEvent object
     * @param xmlContent the XML content to parse
     * @return the parsed AndroidCoTEvent object
     * @throws ParserConfigurationException if the parser configuration is invalid
     * @throws IOException if an I/O error occurs
     * @throws SAXException if the XML parsing fails
     */
    public AndroidCoTEvent parseCoTXml(String xmlContent) throws ParserConfigurationException, IOException, SAXException {
        DocumentBuilder builder = documentBuilderFactory.newDocumentBuilder();
        Document doc = builder.parse(new InputSource(new StringReader(xmlContent)));
        
        Element root = doc.getDocumentElement();
        if (!"event".equals(root.getTagName())) {
            throw new IllegalArgumentException("Root element must be 'event'");
        }
        
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        
        // Parse attributes
        cotEvent.setUid(root.getAttribute("uid"));
        cotEvent.setType(root.getAttribute("type"));
        cotEvent.setVersion(root.getAttribute("version"));
        cotEvent.setTime(root.getAttribute("time"));
        cotEvent.setStart(root.getAttribute("start"));
        cotEvent.setStale(root.getAttribute("stale"));
        cotEvent.setHow(root.getAttribute("how"));
        
        // Parse child elements
        NodeList children = root.getChildNodes();
        for (int i = 0; i < children.getLength(); i++) {
            Node child = children.item(i);
            if (child.getNodeType() == Node.ELEMENT_NODE) {
                Element element = (Element) child;
                
                if ("point".equals(element.getTagName())) {
                    AndroidCoTPoint point = new AndroidCoTPoint();
                    point.setLat(element.getAttribute("lat"));
                    point.setLon(element.getAttribute("lon"));
                    point.setHae(element.getAttribute("hae"));
                    point.setCe(element.getAttribute("ce"));
                    point.setLe(element.getAttribute("le"));
                    cotEvent.setPoint(point);
                } else if ("detail".equals(element.getTagName())) {
                    AndroidCoTDetail detail = new AndroidCoTDetail();
                    detail.setAnyElements(new java.util.ArrayList<>());
                    
                    // Parse detail children
                    NodeList detailChildren = element.getChildNodes();
                    for (int j = 0; j < detailChildren.getLength(); j++) {
                        Node detailChild = detailChildren.item(j);
                        if (detailChild.getNodeType() == Node.ELEMENT_NODE) {
                            detail.getAnyElements().add((Element) detailChild);
                        }
                    }
                    
                    cotEvent.setDetail(detail);
                }
            }
        }
        
        return cotEvent;
    }
    
    /**
     * Convert CoT XML to appropriate Ditto document type based on CoT type
     * @param xmlContent the XML content to convert
     * @return the converted Ditto document
     * @throws Exception if conversion fails
     */
    public Object convertToDocument(String xmlContent) throws Exception {
        AndroidCoTEvent cotEvent = parseCoTXml(xmlContent);
        return convertCoTEventToDocument(cotEvent);
    }
    
    /**
     * Convert AndroidCoTEvent to appropriate Ditto document type
     * @param cotEvent the CoT event to convert
     * @return the converted Ditto document
     */
    public Object convertCoTEventToDocument(AndroidCoTEvent cotEvent) {
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
     * Convert AndroidCoTEvent to ApiDocument
     */
    private ApiDocument convertToApiDocument(AndroidCoTEvent cotEvent) {
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
     * Convert AndroidCoTEvent to ChatDocument
     */
    private ChatDocument convertToChatDocument(AndroidCoTEvent cotEvent) {
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
     * Convert AndroidCoTEvent to FileDocument
     */
    private FileDocument convertToFileDocument(AndroidCoTEvent cotEvent) {
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
     * Convert AndroidCoTEvent to MapItemDocument
     */
    private MapItemDocument convertToMapItemDocument(AndroidCoTEvent cotEvent) {
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
     * Convert AndroidCoTEvent to GenericDocument
     */
    private GenericDocument convertToGenericDocument(AndroidCoTEvent cotEvent) {
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
    private void setCommonFields(Object document, AndroidCoTEvent cotEvent) {
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
    
    private void setCommonFieldsForApiDocument(ApiDocument doc, AndroidCoTEvent cotEvent) {
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
    
    private void setCommonFieldsForChatDocument(ChatDocument doc, AndroidCoTEvent cotEvent) {
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
    
    private void setCommonFieldsForFileDocument(FileDocument doc, AndroidCoTEvent cotEvent) {
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
    
    private void setCommonFieldsForMapItemDocument(MapItemDocument doc, AndroidCoTEvent cotEvent) {
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
    
    private void setCommonFieldsForGenericDocument(GenericDocument doc, AndroidCoTEvent cotEvent) {
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
    private String extractCallsign(AndroidCoTEvent cotEvent) {
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
    private String formatLocation(AndroidCoTPoint point) {
        if (point != null) {
            return point.getLatDouble() + "," + point.getLonDouble();
        }
        return "";
    }
    
    /**
     * Convert Ditto document back to CoT XML
     * @param document the Ditto document to convert
     * @return the XML representation
     * @throws Exception if conversion fails
     */
    public String convertDocumentToXml(Object document) throws Exception {
        AndroidCoTEvent cotEvent = convertDocumentToCoTEvent(document);
        return convertCoTEventToXml(cotEvent);
    }
    
    /**
     * Convert AndroidCoTEvent to XML string
     * @param cotEvent the CoT event to convert
     * @return the XML representation
     * @throws ParserConfigurationException if the parser configuration is invalid
     * @throws TransformerException if XML transformation fails
     */
    public String convertCoTEventToXml(AndroidCoTEvent cotEvent) throws ParserConfigurationException, TransformerException {
        DocumentBuilder builder = documentBuilderFactory.newDocumentBuilder();
        Document doc = builder.newDocument();
        
        // Create root event element
        Element eventElement = doc.createElement("event");
        doc.appendChild(eventElement);
        
        // Set attributes
        eventElement.setAttribute("uid", cotEvent.getUid());
        eventElement.setAttribute("type", cotEvent.getType());
        eventElement.setAttribute("version", cotEvent.getVersion() != null ? cotEvent.getVersion() : "2.0");
        eventElement.setAttribute("time", cotEvent.getTime());
        eventElement.setAttribute("start", cotEvent.getStart());
        eventElement.setAttribute("stale", cotEvent.getStale());
        eventElement.setAttribute("how", cotEvent.getHow() != null ? cotEvent.getHow() : "");
        
        // Add point element
        if (cotEvent.getPoint() != null) {
            Element pointElement = doc.createElement("point");
            pointElement.setAttribute("lat", cotEvent.getPoint().getLat());
            pointElement.setAttribute("lon", cotEvent.getPoint().getLon());
            pointElement.setAttribute("hae", cotEvent.getPoint().getHae());
            pointElement.setAttribute("ce", cotEvent.getPoint().getCe());
            pointElement.setAttribute("le", cotEvent.getPoint().getLe());
            eventElement.appendChild(pointElement);
        }
        
        // Add detail element
        if (cotEvent.getDetail() != null && cotEvent.getDetail().getAnyElements() != null) {
            Element detailElement = doc.createElement("detail");
            
            // Add any elements from the detail
            for (Element element : cotEvent.getDetail().getAnyElements()) {
                Node imported = doc.importNode(element, true);
                detailElement.appendChild(imported);
            }
            
            eventElement.appendChild(detailElement);
        }
        
        // Transform to string
        Transformer transformer = transformerFactory.newTransformer();
        transformer.setOutputProperty(OutputKeys.INDENT, "yes");
        StringWriter writer = new StringWriter();
        transformer.transform(new DOMSource(doc), new StreamResult(writer));
        
        return writer.toString();
    }
    
    /**
     * Convert Ditto document back to AndroidCoTEvent
     */
    public AndroidCoTEvent convertDocumentToCoTEvent(Object document) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        
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
    
    private AndroidCoTEvent convertApiDocumentToCoTEvent(ApiDocument doc) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private AndroidCoTEvent convertChatDocumentToCoTEvent(ChatDocument doc) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private AndroidCoTEvent convertFileDocumentToCoTEvent(FileDocument doc) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private AndroidCoTEvent convertMapItemDocumentToCoTEvent(MapItemDocument doc) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    private AndroidCoTEvent convertGenericDocumentToCoTEvent(GenericDocument doc) {
        AndroidCoTEvent cotEvent = new AndroidCoTEvent();
        setCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }
    
    /**
     * Set AndroidCoTEvent fields from common document fields
     */
    private void setCoTEventFromCommonFields(AndroidCoTEvent cotEvent, Object document) {
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
    
    private void setCommonCoTEventFields(AndroidCoTEvent cotEvent, String id, String type, String version,
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
            AndroidCoTPoint point = new AndroidCoTPoint();
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
                DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
                DocumentBuilder builder = factory.newDocumentBuilder();
                Document tempDoc = builder.newDocument();
                
                AndroidCoTDetail cotDetail = new AndroidCoTDetail();
                cotDetail.setFromMap(detail, tempDoc);
                cotEvent.setDetail(cotDetail);
            } catch (Exception e) {
                // Fallback to empty detail if conversion fails
                cotEvent.setDetail(new AndroidCoTDetail());
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
        Map<String, Object> map = objectMapper.convertValue(document, new TypeReference<Map<String, Object>>() {});
        
        // Extract the nested r field and flatten it into r_* fields
        Object rField = map.get("r");
        if (rField instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> rMap = (Map<String, Object>) rField;
            
            // Remove the nested r field
            map.remove("r");
            
            // Add flattened r_* fields
            Map<String, Object> flattenedFields = flattenDetailToRFields(rMap);
            map.putAll(flattenedFields);
        }
        
        return map;
    }

    /**
     * Reconstruct the nested r field from flattened r_* fields.
     * Converts r_takv_os: "35" back to {takv: {os: "35"}}
     */
    private Map<String, Object> unflattenRFieldsToDetail(Map<String, Object> map) {
        Map<String, Object> rField = new HashMap<>();
        Map<String, Object> mapCopy = new HashMap<>(map);
        
        // Find all r_* fields and reconstruct the nested structure
        for (Map.Entry<String, Object> entry : map.entrySet()) {
            String key = entry.getKey();
            if (key.startsWith("r_")) {
                // Remove the r_ prefix
                String withoutPrefix = key.substring(2);
                
                // Handle special case for __group (detail elements starting with underscores)
                int lastUnderscore = withoutPrefix.lastIndexOf('_');
                if (lastUnderscore > 0) {
                    String detailType = withoutPrefix.substring(0, lastUnderscore);
                    String attribute = withoutPrefix.substring(lastUnderscore + 1);
                    
                    // Get or create the detail type map
                    @SuppressWarnings("unchecked")
                    Map<String, Object> detailMap = (Map<String, Object>) rField.computeIfAbsent(
                        detailType, k -> new HashMap<String, Object>()
                    );
                    
                    // Add the attribute
                    detailMap.put(attribute, entry.getValue());
                    
                    // Remove the flattened field from the copy
                    mapCopy.remove(key);
                }
            }
        }
        
        // Add the reconstructed r field back to the map
        if (!rField.isEmpty()) {
            mapCopy.put("r", rField);
        }
        
        return mapCopy;
    }
    
    /**
     * Convert a Map from Ditto back to a CoT document
     */
    public <T> T convertMapToDocument(Map<String, Object> map, Class<T> documentClass) {
        // First unflatten any r_* fields back to nested r field
        Map<String, Object> unflattenedMap = unflattenRFieldsToDetail(map);
        return objectMapper.convertValue(unflattenedMap, documentClass);
    }

    /**
     * Convert JSON string from Ditto back to a CoT document
     */
    public <T> T convertJsonToDocument(String json, Class<T> documentClass) throws JsonProcessingException {
        return objectMapper.readValue(json, documentClass);
    }

    /**
     * Public method to unflatten r_* fields back to nested r field structure.
     * This method should be called by ATAK when processing flattened Ditto documents
     * to restore the nested detail structure needed for callsign and other detail access.
     * 
     * Example:
     * Input:  {r_contact_callsign: "USV-4", r_contact_endpoint: "*:-1:stcp", e: "USV-4"}
     * Output: {r: {contact: {callsign: "USV-4", endpoint: "*:-1:stcp"}}, e: "USV-4"}
     * 
     * @param flattenedMap Map containing flattened r_* fields from Ditto
     * @return Map with r_* fields reconstructed into nested r field structure
     */
    public Map<String, Object> unflattenRField(Map<String, Object> flattenedMap) {
        Map<String, Object> result = unflattenRFieldsToDetail(flattenedMap);
        
        // Extract important fields from the reconstructed r structure and set them as top-level keys
        // This ensures insertTopLevelProperties can find them in the expected locations
        Object rField = result.get("r");
        if (rField instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> rMap = (Map<String, Object>) rField;
            
            // Extract contact callsign and set it in 'e' field (DITTO_KEY_AUTHOR_CALLSIGN)
            Object contact = rMap.get("contact");
            if (contact instanceof Map) {
                @SuppressWarnings("unchecked")
                Map<String, Object> contactMap = (Map<String, Object>) contact;
                Object callsign = contactMap.get("callsign");
                if (callsign instanceof String && !((String) callsign).isEmpty()) {
                    result.put("e", callsign); // DITTO_KEY_AUTHOR_CALLSIGN
                }
            }
            
            // Extract track speed and course if present
            Object track = rMap.get("track");
            if (track instanceof Map) {
                @SuppressWarnings("unchecked")
                Map<String, Object> trackMap = (Map<String, Object>) track;
                Object speed = trackMap.get("speed");
                if (speed != null) {
                    result.put("r1", speed); // DITTO_KEY_COT_EVENT_DETAIL_TRACK_SPEED
                }
                Object course = trackMap.get("course");
                if (course != null) {
                    result.put("r2", course); // DITTO_KEY_COT_EVENT_DETAIL_TRACK_COURSE
                }
            }
        }
        
        return result;
    }

}