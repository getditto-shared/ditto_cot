using NUnit.Framework;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;
using System.IO;

namespace Ditto.Cot.Tests;

[TestFixture]
public class ComplexDetailTest
{
    [Test]
    public void TestComplexDetailWithDuplicateElements()
    {
        // Use embedded complex detail XML for testing
        var xmlContent = @"<?xml version=""1.0"" standalone=""yes""?>
<event version=""2.0"" type=""a-u-S"" uid=""complex-detail-test"" time=""2025-07-05T21:00:00Z"" start=""2025-07-05T21:00:00Z"" stale=""2025-07-05T21:30:00Z"" how=""m-d-a"">
  <point lat=""35.123456"" lon=""-118.987654"" hae=""150.0"" ce=""100.0"" le=""50.0""/>
  <detail>
    <!-- Multiple sensor elements with same name but different attributes -->
    <sensor type=""optical"" resolution=""4K"" zoom=""10x"" id=""sensor-1""/>
    <sensor type=""thermal"" resolution=""1080p"" zoom=""5x"" id=""sensor-2""/>
    <sensor type=""radar"" frequency=""9.4GHz"" range=""50km"" id=""sensor-3""/>
    
    <!-- Multiple contact elements -->
    <contact callsign=""ALPHA-01"" endpoint=""192.168.1.100:8080"" role=""primary""/>
    <contact callsign=""BRAVO-02"" endpoint=""192.168.1.101:8080"" role=""backup""/>
    
    <!-- Multiple track elements representing historical positions -->
    <track course=""45.0"" speed=""2.5"" timestamp=""2025-07-05T20:55:00Z""/>
    <track course=""50.0"" speed=""3.0"" timestamp=""2025-07-05T20:58:00Z""/>
    <track course=""55.0"" speed=""2.8"" timestamp=""2025-07-05T21:00:00Z""/>
    
    <!-- Multiple remarks elements -->
    <remarks type=""operational"">Primary surveillance platform</remarks>
    <remarks type=""maintenance"">Last service: 2025-07-01</remarks>
    <remarks type=""alert"">Low battery warning</remarks>
    
    <!-- Single elements that should remain unique -->
    <status operational=""true"" last_maintenance=""2025-07-01T10:00:00Z""/>
    <acquisition method=""manual"" operator=""SENSOR_OP_001""/>
  </detail>
</event>";
        
        // Parse the CoT event first to inspect raw XML
        var cotEvent = DocumentConverter.ParseCoTXml(xmlContent);
        Console.WriteLine($"Raw detail XML: {cotEvent.Detail.RawXml}");
        Console.WriteLine($"Detail elements count: {cotEvent.Detail.Elements?.Length ?? 0}");
        
        if (cotEvent.Detail.Elements != null)
        {
            foreach (var elem in cotEvent.Detail.Elements)
            {
                Console.WriteLine($"Element: {elem.Name}, Attributes: {elem.Attributes?.Count ?? 0}");
            }
        }
        
        // Convert XML to Ditto Document
        var result = DocumentConverter.ConvertXmlToDocument(xmlContent, "complex-detail-test");
        
        // Should be a MapItemDocument (a-u-S type)
        Assert.That(result, Is.InstanceOf<MapItemDocument>());
        var mapItem = (MapItemDocument)result;
        
        // Verify the detail map contains all elements without loss
        Assert.That(mapItem.Detail, Is.Not.Empty);
        
        Console.WriteLine("=== C# CRDT-OPTIMIZED DETAIL PARSING TEST ===");
        Console.WriteLine($"Total keys generated: {mapItem.Detail.Count}");
        
        // Print all keys for debugging
        foreach (var kvp in mapItem.Detail)
        {
            Console.WriteLine($"Key: {kvp.Key}");
            if (kvp.Value is Dictionary<string, object> dict)
            {
                if (dict.ContainsKey("_tag"))
                {
                    Console.WriteLine($"  Tag: {dict["_tag"]}");
                }
                Console.WriteLine($"  Content: {JsonConvert.SerializeObject(dict)}");
            }
            else
            {
                Console.WriteLine($"  Value type: {kvp.Value?.GetType()}");
                Console.WriteLine($"  Value: {kvp.Value}");
            }
        }
        
        // Verify single occurrence elements use direct keys
        Assert.That(mapItem.DetailContainsKey("status"), Is.True, "Single 'status' element should use direct key");
        Assert.That(mapItem.DetailContainsKey("acquisition"), Is.True, "Single 'acquisition' element should use direct key");
        
        // Count duplicate elements by metadata
        var sensorCount = CountElementsByTag(mapItem.Detail, "sensor");
        var contactCount = CountElementsByTag(mapItem.Detail, "contact");
        var trackCount = CountElementsByTag(mapItem.Detail, "track");
        var remarksCount = CountElementsByTag(mapItem.Detail, "remarks");
        
        Console.WriteLine($"Sensor elements: {sensorCount}");
        Console.WriteLine($"Contact elements: {contactCount}");
        Console.WriteLine($"Track elements: {trackCount}");
        Console.WriteLine($"Remarks elements: {remarksCount}");
        
        // Verify all duplicate elements are preserved
        Assert.That(sensorCount, Is.EqualTo(3), "Should have 3 sensor elements");
        Assert.That(contactCount, Is.EqualTo(2), "Should have 2 contact elements");
        Assert.That(trackCount, Is.EqualTo(3), "Should have 3 track elements");
        Assert.That(remarksCount, Is.EqualTo(3), "Should have 3 remarks elements");
        
        // Total: 2 single + 11 with stable keys = 13 elements preserved
        var actualDetailMap = mapItem.GetDetailFromJson();
        Assert.That(actualDetailMap.Count, Is.EqualTo(13), "All 13 detail elements should be preserved");
        
        Console.WriteLine("✅ All elements preserved with stable keys!");
    }
    
    private int CountElementsByTag(Dictionary<string, object> detail, string tagName)
    {
        // First get the actual detail data from JSON storage
        var detailMap = new Dictionary<string, object>();
        if (detail.TryGetValue("_json", out var jsonValue) && jsonValue is string json)
        {
            try
            {
                detailMap = JsonConvert.DeserializeObject<Dictionary<string, object>>(json) ?? new Dictionary<string, object>();
            }
            catch
            {
                return 0;
            }
        }
        else
        {
            detailMap = detail;
        }
        
        return detailMap.Values
            .OfType<Dictionary<string, object>>()
            .Concat(detailMap.Values.OfType<Newtonsoft.Json.Linq.JObject>().Select(jo => jo.ToObject<Dictionary<string, object>>() ?? new Dictionary<string, object>()))
            .Count(dict => dict.ContainsKey("_tag") && dict["_tag"]?.ToString() == tagName);
    }
}