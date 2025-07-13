using System.Xml;
using System.Xml.Serialization;
using Ditto.Cot.Models;
using Newtonsoft.Json;

namespace Ditto.Cot;

public static class DocumentConverter
{
    /// <summary>
    /// Convert a CoT XML string to the appropriate Ditto document type
    /// </summary>
    public static object ConvertXmlToDocument(string xmlContent, string peerKey = "cot-peer-key")
    {
        var cotEvent = ParseCoTXml(xmlContent);
        return ConvertCoTEventToDocument(cotEvent, peerKey);
    }

    /// <summary>
    /// Convert a CoTEvent to the appropriate Ditto document type based on CoT type
    /// </summary>
    public static object ConvertCoTEventToDocument(CoTEvent cotEvent, string peerKey = "cot-peer-key")
    {
        var cotType = cotEvent.Type;

        if (cotType == "a-u-emergency-g")
        {
            return ConvertToApiDocument(cotEvent, peerKey);
        }
        else if (cotType.Contains("b-t-f") || cotType.Contains("chat"))
        {
            return ConvertToChatDocument(cotEvent, peerKey);
        }
        else if (cotType.Contains("file") || cotType.Contains("attachment") || cotType.Contains("b-f-t-a"))
        {
            return ConvertToFileDocument(cotEvent, peerKey);
        }
        else if (cotType.Contains("a-u-r-loc-g") || cotType.Contains("a-f-G-U-C") ||
                 cotType.Contains("a-f-G-U") || cotType.Contains("a-f-G-U-I") ||
                 cotType.Contains("a-f-G-U-T") || cotType.Contains("a-u-S") ||
                 cotType.Contains("a-u-A") || cotType.Contains("a-u-G"))
        {
            return ConvertToMapItemDocument(cotEvent, peerKey);
        }
        else
        {
            return ConvertToGenericDocument(cotEvent, peerKey);
        }
    }

    /// <summary>
    /// Parse CoT XML string into a CoTEvent object
    /// </summary>
    public static CoTEvent ParseCoTXml(string xmlContent)
    {
        var serializer = new XmlSerializer(typeof(CoTEvent));
        using var reader = new StringReader(xmlContent);
        var cotEvent = (CoTEvent?)serializer.Deserialize(reader);
        
        if (cotEvent == null)
        {
            throw new InvalidOperationException("Failed to deserialize CoT XML");
        }

        // Store raw detail XML for round-trip conversion
        if (cotEvent.Detail != null)
        {
            try
            {
                var doc = new XmlDocument();
                doc.LoadXml(xmlContent);
                var detailNode = doc.SelectSingleNode("//detail");
                cotEvent.Detail.RawXml = detailNode?.OuterXml ?? "<detail></detail>";
            }
            catch
            {
                cotEvent.Detail.RawXml = "<detail></detail>";
            }
        }

        return cotEvent;
    }

    /// <summary>
    /// Convert CoTEvent to ApiDocument
    /// </summary>
    public static ApiDocument ConvertToApiDocument(CoTEvent cotEvent, string peerKey)
    {
        var doc = new ApiDocument();
        SetCommonFields(doc, cotEvent, peerKey);

        doc.IsFile = false;
        doc.Title = $"CoT Event: {cotEvent.Uid}";
        doc.Mime = "application/vnd.cot.emergency+json";
        doc.ContentType = "emergency";
        doc.Data = cotEvent.Uid;
        doc.IsRemoved = false;
        doc.TimeMillisField = DateTimeOffset.TryParse(cotEvent.Time, out var timeResult) 
            ? timeResult.ToUnixTimeMilliseconds() 
            : DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();

        return doc;
    }

    /// <summary>
    /// Convert CoTEvent to ChatDocument
    /// </summary>
    public static ChatDocument ConvertToChatDocument(CoTEvent cotEvent, string peerKey)
    {
        var doc = new ChatDocument();
        SetCommonFields(doc, cotEvent, peerKey);

        doc.Message = $"CoT Event: {cotEvent.Uid}";
        doc.Room = "cot-events";
        doc.RoomId = $"cot-room-{Guid.NewGuid()}";
        doc.AuthorCallsign = ExtractCallsign(cotEvent);
        doc.AuthorUidField = cotEvent.Uid;
        doc.AuthorType = cotEvent.Type;
        doc.Time = cotEvent.Time;
        doc.Location = $"{cotEvent.Point.LatDouble},{cotEvent.Point.LonDouble},{cotEvent.Point.HaeDouble}";

        return doc;
    }

    /// <summary>
    /// Convert CoTEvent to FileDocument
    /// </summary>
    public static FileDocument ConvertToFileDocument(CoTEvent cotEvent, string peerKey)
    {
        var doc = new FileDocument();
        SetCommonFields(doc, cotEvent, peerKey);

        doc.FileName = $"{cotEvent.Uid}.xml";
        doc.Size = 1024.0; // Placeholder size
        doc.File = cotEvent.Uid;
        doc.Mime = "application/xml";
        doc.ContentType = "file";
        doc.ItemId = cotEvent.Uid;

        return doc;
    }

    /// <summary>
    /// Convert CoTEvent to MapItemDocument
    /// </summary>
    public static MapItemDocument ConvertToMapItemDocument(CoTEvent cotEvent, string peerKey)
    {
        var doc = new MapItemDocument();
        SetCommonFields(doc, cotEvent, peerKey);

        var callsign = ExtractCallsign(cotEvent);
        doc.Name = !string.IsNullOrEmpty(callsign) ? callsign : cotEvent.Uid;
        doc.Visible = true;

        return doc;
    }

    /// <summary>
    /// Convert CoTEvent to GenericDocument
    /// </summary>
    public static GenericDocument ConvertToGenericDocument(CoTEvent cotEvent, string peerKey)
    {
        var doc = new GenericDocument();
        SetCommonFields(doc, cotEvent, peerKey);
        return doc;
    }

    /// <summary>
    /// Set common fields that all documents inherit from CommonDocument
    /// </summary>
    private static void SetCommonFields(CommonDocument doc, CoTEvent cotEvent, string peerKey)
    {
        doc.Id = cotEvent.Uid;
        doc.Counter = 0;
        doc.Version = 2;
        doc.Removed = false;
        doc.PeerKey = peerKey;
        doc.AuthorUid = cotEvent.Uid;
        doc.Callsign = ExtractCallsign(cotEvent) ?? string.Empty;
        doc.VersionString = cotEvent.Version;

        // Set point data - following schema mapping  
        doc.CircularError = cotEvent.Point.CeDouble;
        doc.HeightAboveEllipsoid = cotEvent.Point.HaeDouble;
        doc.Latitude = cotEvent.Point.LatDouble;
        doc.LinearError = cotEvent.Point.LeDouble;
        doc.Longitude = cotEvent.Point.LonDouble;
        
        // Set time field (b)
        doc.TimeMillisFromEpoch = DateTimeOffset.TryParse(cotEvent.Time, out var timeParseResult) 
            ? timeParseResult.ToUnixTimeMilliseconds() 
            : DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();

        // Convert timestamps to microseconds for precision (following Rust implementation)
        doc.StartTime = DateTimeOffset.TryParse(cotEvent.Start, out var startResult) 
            ? startResult.ToUnixTimeMilliseconds() * 1000 
            : DateTimeOffset.UtcNow.ToUnixTimeMilliseconds() * 1000;

        doc.StaleTime = DateTimeOffset.TryParse(cotEvent.Stale, out var staleResult) 
            ? staleResult.ToUnixTimeMilliseconds() * 1000 
            : DateTimeOffset.UtcNow.AddMinutes(30).ToUnixTimeMilliseconds() * 1000;

        // Store time for conversion but don't serialize to JSON
        doc.TimeMillis = DateTimeOffset.TryParse(cotEvent.Time, out var timeResult) 
            ? timeResult.ToUnixTimeMilliseconds() 
            : DateTimeOffset.UtcNow.ToUnixTimeMilliseconds();

        // Store original timestamp strings for precision preservation (base64 encoded)
        doc.SetTimeOriginal(cotEvent.Time);
        doc.SetStartOriginal(cotEvent.Start);
        doc.SetStaleOriginal(cotEvent.Stale);
        
        // Store original point values for precision preservation (base64 encoded)
        doc.SetCeOriginal(cotEvent.Point.Ce);
        doc.SetLeOriginal(cotEvent.Point.Le);
        doc.SetLatOriginal(cotEvent.Point.Lat);
        doc.SetLonOriginal(cotEvent.Point.Lon);
        doc.SetHaeOriginal(cotEvent.Point.Hae);


        doc.How = cotEvent.How;
        doc.EventType = cotEvent.Type;

        // Serialize detail section as JSON string for CRDT compatibility
        var detailMap = cotEvent.Detail?.ToMapWithStableKeys(cotEvent.Uid) ?? new Dictionary<string, object>();
        doc.SetDetailAsJson(detailMap);

        doc.Source = "cot-converter-csharp";
    }

    /// <summary>
    /// Extract callsign from CoT detail if available
    /// </summary>
    private static string? ExtractCallsign(CoTEvent cotEvent)
    {
        var detailMap = cotEvent.Detail?.ToMap();
        if (detailMap == null) return null;

        // Try contact.callsign
        if (detailMap.TryGetValue("contact", out var contactValue) && contactValue is Dictionary<string, object> contact)
        {
            if (contact.TryGetValue("callsign", out var callsignValue) && callsignValue is string callsign)
            {
                return callsign;
            }
        }

        // Try ditto.deviceName  
        if (detailMap.TryGetValue("ditto", out var dittoValue) && dittoValue is Dictionary<string, object> ditto)
        {
            if (ditto.TryGetValue("deviceName", out var deviceNameValue) && deviceNameValue is string deviceName)
            {
                return deviceName;
            }
        }

        return null;
    }

    // Reverse conversion methods (Ditto Document to CoT Event)

    /// <summary>
    /// Convert ApiDocument back to CoTEvent
    /// </summary>
    public static CoTEvent ConvertApiDocumentToCoTEvent(ApiDocument doc)
    {
        var cotEvent = new CoTEvent();
        SetCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }

    /// <summary>
    /// Convert ChatDocument back to CoTEvent
    /// </summary>
    public static CoTEvent ConvertChatDocumentToCoTEvent(ChatDocument doc)
    {
        var cotEvent = new CoTEvent();
        SetCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }

    /// <summary>
    /// Convert FileDocument back to CoTEvent
    /// </summary>
    public static CoTEvent ConvertFileDocumentToCoTEvent(FileDocument doc)
    {
        var cotEvent = new CoTEvent();
        SetCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }

    /// <summary>
    /// Convert GenericDocument back to CoTEvent
    /// </summary>
    public static CoTEvent ConvertGenericDocumentToCoTEvent(GenericDocument doc)
    {
        var cotEvent = new CoTEvent();
        SetCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }

    /// <summary>
    /// Convert MapItemDocument back to CoTEvent
    /// </summary>
    public static CoTEvent ConvertMapItemDocumentToCoTEvent(MapItemDocument doc)
    {
        var cotEvent = new CoTEvent();
        SetCoTEventFromCommonFields(cotEvent, doc);
        return cotEvent;
    }

    /// <summary>
    /// Set CoTEvent fields from common document fields
    /// </summary>
    private static void SetCoTEventFromCommonFields(CoTEvent cotEvent, CommonDocument doc)
    {
        cotEvent.Version = "2.0";
        cotEvent.Uid = doc.Id; // Use document Id as the CoT UID
        cotEvent.Type = doc.EventType;
        cotEvent.How = doc.How;

        // Convert timestamps back from original strings if available, otherwise use computed values
        cotEvent.Time = doc.GetTimeOriginal() ?? DateTimeOffset.FromUnixTimeMilliseconds((long)doc.TimeMillisFromEpoch).ToString("yyyy-MM-ddTHH:mm:ss.fffZ");
        cotEvent.Start = doc.GetStartOriginal() ?? DateTimeOffset.FromUnixTimeMilliseconds(doc.StartTime / 1000).ToString("yyyy-MM-ddTHH:mm:ss.fffZ");
        cotEvent.Stale = doc.GetStaleOriginal() ?? DateTimeOffset.FromUnixTimeMilliseconds(doc.StaleTime / 1000).ToString("yyyy-MM-ddTHH:mm:ss.fffZ");

        // Set point data
        cotEvent.Point = new CoTPoint
        {
            Lat = doc.GetLatOriginal() ?? doc.Latitude?.ToString("G17") ?? "0.0",
            Lon = doc.GetLonOriginal() ?? doc.Longitude?.ToString("G17") ?? "0.0",
            Hae = doc.GetHaeOriginal() ?? doc.HeightAboveEllipsoid?.ToString("G17") ?? "0.0",
            Ce = doc.GetCeOriginal() ?? doc.CircularError?.ToString("G17") ?? "0.0",
            Le = doc.GetLeOriginal() ?? doc.LinearError?.ToString("G17") ?? "0.0"
        };

        // Convert detail JSON back to XML structure
        cotEvent.Detail = new CoTDetail();
        var detailMap = doc.GetDetailFromJson();
        if (detailMap.Count > 0)
        {
            try
            {
                // Convert the detail map back to XmlElement array for proper serialization
                cotEvent.Detail.Elements = ConvertDetailMapToXmlElements(detailMap);
                
                // Also generate RawXml for compatibility with tests
                cotEvent.Detail.RawXml = ConvertDetailMapToXml(detailMap);
            }
            catch
            {
                cotEvent.Detail.Elements = new System.Xml.XmlElement[0];
                cotEvent.Detail.RawXml = "<detail></detail>";
            }
        }
        else
        {
            cotEvent.Detail.RawXml = "<detail></detail>";
        }
    }

    /// <summary>
    /// Convert detail map back to XmlElement array for proper serialization
    /// </summary>
    private static System.Xml.XmlElement[] ConvertDetailMapToXmlElements(Dictionary<string, object> detail)
    {
        if (detail.Count == 0) return new System.Xml.XmlElement[0];

        var elements = new List<System.Xml.XmlElement>();
        var doc = new System.Xml.XmlDocument();

        foreach (var kvp in detail)
        {
            var element = CreateXmlElementFromObject(doc, kvp.Key, kvp.Value);
            if (element != null)
            {
                elements.Add(element);
            }
        }

        return elements.ToArray();
    }

    /// <summary>
    /// Create XmlElement from object (for detail reconstruction)
    /// </summary>
    private static System.Xml.XmlElement? CreateXmlElementFromObject(System.Xml.XmlDocument doc, string name, object value)
    {
        var element = doc.CreateElement(name);

        if (value is Dictionary<string, object> dict)
        {
            // Add attributes from the dictionary
            foreach (var kvp in dict)
            {
                // Skip internal metadata fields but include all other string values as attributes
                if (kvp.Value is string stringValue && !kvp.Key.StartsWith("_"))
                {
                    element.SetAttribute(kvp.Key, stringValue);
                }
                // Handle JValue types (from JSON deserialization)
                else if (kvp.Value is Newtonsoft.Json.Linq.JValue jValue && !kvp.Key.StartsWith("_"))
                {
                    var valueStr = jValue.ToString();
                    if (!string.IsNullOrEmpty(valueStr))
                    {
                        element.SetAttribute(kvp.Key, valueStr);
                    }
                }
                // Also handle other object types
                else if (kvp.Value != null && !kvp.Key.StartsWith("_"))
                {
                    var valueStr = kvp.Value.ToString();
                    if (!string.IsNullOrEmpty(valueStr))
                    {
                        element.SetAttribute(kvp.Key, valueStr);
                    }
                }
            }
        }
        // Handle JObject (from JSON deserialization)
        else if (value is Newtonsoft.Json.Linq.JObject jObj)
        {
            foreach (var prop in jObj.Properties())
            {
                if (!prop.Name.StartsWith("_") && prop.Value != null)
                {
                    element.SetAttribute(prop.Name, prop.Value.ToString());
                }
            }
        }
        else if (value is string stringValue)
        {
            element.InnerText = stringValue;
        }

        return element;
    }

    /// <summary>
    /// Convert detail map back to XML string (simplified implementation)
    /// </summary>
    private static string ConvertDetailMapToXml(Dictionary<string, object> detail)
    {
        if (detail.Count == 0) return "<detail></detail>";

        var xml = "<detail>";
        foreach (var kvp in detail)
        {
            xml += ConvertObjectToXml(kvp.Key, kvp.Value);
        }
        xml += "</detail>";

        return xml;
    }

    /// <summary>
    /// Convert an object to XML representation
    /// </summary>
    private static string ConvertObjectToXml(string name, object value)
    {
        if (value is Dictionary<string, object> dict)
        {
            var xml = $"<{name}";
            var children = "";

            foreach (var kvp in dict)
            {
                if (kvp.Value is string strValue)
                {
                    xml += $" {kvp.Key}=\"{System.Security.SecurityElement.Escape(strValue)}\"";
                }
                else
                {
                    children += ConvertObjectToXml(kvp.Key, kvp.Value);
                }
            }

            xml += ">";
            xml += children;
            xml += $"</{name}>";
            return xml;
        }
        else if (value is string stringValue)
        {
            return $"<{name}>{System.Security.SecurityElement.Escape(stringValue)}</{name}>";
        }
        else
        {
            return $"<{name}>{value}</{name}>";
        }
    }

    /// <summary>
    /// Convert CoTEvent to XML string
    /// </summary>
    public static string ConvertCoTEventToXml(CoTEvent cotEvent)
    {
        var serializer = new XmlSerializer(typeof(CoTEvent));
        var namespaces = new XmlSerializerNamespaces();
        namespaces.Add("", ""); // Remove default namespaces
        
        using var stringWriter = new StringWriter();
        using var xmlWriter = XmlWriter.Create(stringWriter, new XmlWriterSettings 
        { 
            OmitXmlDeclaration = true,
            Indent = true
        });
        
        serializer.Serialize(xmlWriter, cotEvent, namespaces);
        return stringWriter.ToString();
    }

    /// <summary>
    /// Convert Ditto document back to CoT XML
    /// </summary>
    public static string ConvertDocumentToXml(object document)
    {
        CoTEvent cotEvent = document switch
        {
            ApiDocument api => ConvertApiDocumentToCoTEvent(api),
            ChatDocument chat => ConvertChatDocumentToCoTEvent(chat),
            FileDocument file => ConvertFileDocumentToCoTEvent(file),
            GenericDocument generic => ConvertGenericDocumentToCoTEvent(generic),
            MapItemDocument mapItem => ConvertMapItemDocumentToCoTEvent(mapItem),
            _ => throw new ArgumentException($"Unknown document type: {document.GetType()}")
        };

        return ConvertCoTEventToXml(cotEvent);
    }

    /// <summary>
    /// Convert Ditto document to JSON string
    /// </summary>
    public static string ConvertDocumentToJson(object document)
    {
        return JsonConvert.SerializeObject(document, Newtonsoft.Json.Formatting.Indented);
    }

    /// <summary>
    /// Convert JSON string to Ditto document with proper type conversion
    /// </summary>
    public static T ConvertJsonToDocument<T>(string json) where T : CommonDocument
    {
        // Use custom settings to handle integer conversion
        var settings = new JsonSerializerSettings
        {
            Converters = { new IntegerTypeConverter() }
        };
        
        var doc = JsonConvert.DeserializeObject<T>(json, settings);
        return doc ?? throw new ArgumentException($"Failed to deserialize JSON to {typeof(T).Name}");
    }
}

/// <summary>
/// Custom JSON converter that ensures double values that should be integers are properly converted
/// </summary>
public class IntegerTypeConverter : JsonConverter
{
    public override bool CanConvert(Type objectType)
    {
        return objectType == typeof(int) || objectType == typeof(int?) || 
               objectType == typeof(long) || objectType == typeof(long?);
    }

    public override object? ReadJson(JsonReader reader, Type objectType, object? existingValue, JsonSerializer serializer)
    {
        if (reader.TokenType == JsonToken.Null)
        {
            if (objectType == typeof(int?) || objectType == typeof(long?))
                return null;
            return 0;
        }

        if (reader.TokenType == JsonToken.Integer)
        {
            var longValue = Convert.ToInt64(reader.Value);
            
            if (objectType == typeof(int) || objectType == typeof(int?))
            {
                if (longValue >= int.MinValue && longValue <= int.MaxValue)
                    return (int)longValue;
                throw new JsonSerializationException($"Value {longValue} is out of range for Int32");
            }
            
            return longValue;
        }

        if (reader.TokenType == JsonToken.Float)
        {
            var doubleValue = Convert.ToDouble(reader.Value);
            var longValue = Convert.ToInt64(doubleValue);
            
            // Check if it's a whole number
            if (Math.Abs(doubleValue - longValue) < double.Epsilon)
            {
                if (objectType == typeof(int) || objectType == typeof(int?))
                {
                    if (longValue >= int.MinValue && longValue <= int.MaxValue)
                        return (int)longValue;
                    throw new JsonSerializationException($"Value {longValue} is out of range for Int32");
                }
                
                return longValue;
            }
            
            throw new JsonSerializationException($"Cannot convert decimal value {doubleValue} to integer type {objectType.Name}");
        }

        throw new JsonSerializationException($"Unexpected token {reader.TokenType} when parsing integer");
    }

    public override void WriteJson(JsonWriter writer, object? value, JsonSerializer serializer)
    {
        if (value == null)
        {
            writer.WriteNull();
        }
        else
        {
            writer.WriteValue(value);
        }
    }
}