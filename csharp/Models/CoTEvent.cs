using System.Xml.Serialization;
using Newtonsoft.Json;

namespace Ditto.Cot.Models;

[XmlRoot("event")]
public class CoTEvent
{
    [XmlAttribute("version")]
    public string Version { get; set; } = "2.0";

    [XmlAttribute("uid")]
    public string Uid { get; set; } = string.Empty;

    [XmlAttribute("type")]
    public string Type { get; set; } = string.Empty;

    [XmlAttribute("time")]
    public string Time { get; set; } = string.Empty;

    [XmlAttribute("start")]
    public string Start { get; set; } = string.Empty;

    [XmlAttribute("stale")]
    public string Stale { get; set; } = string.Empty;

    [XmlAttribute("how")]
    public string How { get; set; } = string.Empty;

    [XmlElement("point")]
    public CoTPoint Point { get; set; } = new();

    [XmlElement("detail")]
    public CoTDetail Detail { get; set; } = new();
}

public class CoTPoint
{
    [XmlAttribute("lat")]
    public string Lat { get; set; } = "0.0";

    [XmlAttribute("lon")]
    public string Lon { get; set; } = "0.0";

    [XmlAttribute("hae")]
    public string Hae { get; set; } = "0.0";

    [XmlAttribute("ce")]
    public string Ce { get; set; } = "0.0";

    [XmlAttribute("le")]
    public string Le { get; set; } = "0.0";

    [JsonIgnore]
    public double LatDouble => double.TryParse(Lat, out var result) ? result : 0.0;

    [JsonIgnore]
    public double LonDouble => double.TryParse(Lon, out var result) ? result : 0.0;

    [JsonIgnore]
    public double HaeDouble => double.TryParse(Hae, out var result) ? result : 0.0;

    [JsonIgnore]
    public double CeDouble => double.TryParse(Ce, out var result) ? result : 0.0;

    [JsonIgnore]
    public double LeDouble => double.TryParse(Le, out var result) ? result : 0.0;
}

public class CoTDetail
{
    [XmlAnyElement]
    public System.Xml.XmlElement[]? Elements { get; set; }

    [JsonIgnore]
    [XmlIgnore]
    public string RawXml { get; set; } = string.Empty;

    public Dictionary<string, object> ToMap()
    {
        return ToMapWithStableKeys("default-doc-id");
    }

    public Dictionary<string, object> ToMapWithStableKeys(string docId)
    {
        var result = new Dictionary<string, object>();
        var elementCounts = new Dictionary<string, int>();
        
        if (Elements != null)
        {
            // First pass: count occurrences of each element name
            foreach (var element in Elements)
            {
                elementCounts[element.Name] = elementCounts.GetValueOrDefault(element.Name, 0) + 1;
            }

            // Second pass: assign stable keys
            var elementIndices = new Dictionary<string, int>();
            foreach (var element in Elements)
            {
                var elementName = element.Name;
                var currentIndex = elementIndices.GetValueOrDefault(elementName, 0);
                elementIndices[elementName] = currentIndex + 1;

                string key;
                if (elementCounts[elementName] == 1)
                {
                    // Single occurrence - use direct key
                    key = elementName;
                }
                else
                {
                    // Multiple occurrences - use stable hash-based key that includes element name
                    var stableInput = $"{docId}_{elementName}_{currentIndex}";
                    var hashBytes = System.Security.Cryptography.SHA256.HashData(System.Text.Encoding.UTF8.GetBytes(stableInput));
                    var hash = Convert.ToBase64String(hashBytes)
                        .Replace("=", "")
                        .Replace("+", "")
                        .Replace("/", "")
                        .Substring(0, 8);
                    key = $"{elementName}_{hash}_{currentIndex}";
                }

                var elementData = ParseElementWithMetadata(element, elementName, docId, currentIndex);
                result[key] = elementData;
            }
        }

        return result;
    }

    private object ParseElement(System.Xml.XmlElement element)
    {
        return ParseElementWithMetadata(element, element.Name, "default-doc-id", 0);
    }

    private object ParseElementWithMetadata(System.Xml.XmlElement element, string originalTag, string docId, int elementIndex)
    {
        var result = new Dictionary<string, object>();

        // Add CRDT metadata
        result["_tag"] = originalTag;
        result["_docId"] = docId;
        result["_elementIndex"] = elementIndex;

        // Add attributes
        if (element.Attributes != null)
        {
            foreach (System.Xml.XmlAttribute attr in element.Attributes)
            {
                result[attr.Name] = attr.Value;
            }
        }

        // Check for text content (but exclude elements that only contain other elements)
        var textContent = element.InnerText?.Trim();
        var hasElementChildren = element.ChildNodes.OfType<System.Xml.XmlElement>().Any();
        
        if (!string.IsNullOrEmpty(textContent) && !hasElementChildren)
        {
            result["_text"] = textContent;
        }

        // Add child elements (recursive)
        var childCounts = new Dictionary<string, int>();
        foreach (System.Xml.XmlNode child in element.ChildNodes)
        {
            if (child is System.Xml.XmlElement childElement)
            {
                var childName = childElement.Name;
                var childIndex = childCounts.GetValueOrDefault(childName, 0);
                childCounts[childName] = childIndex + 1;

                var childData = ParseElementWithMetadata(childElement, childName, docId, childIndex);
                result[childName] = childData;
            }
        }

        return result;
    }
}