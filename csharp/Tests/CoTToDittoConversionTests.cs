using NUnit.Framework;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;

namespace Ditto.Cot.Tests;

[TestFixture]
public class CoTToDittoConversionTests
{
    private const string SampleFriendlyUnitXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""ANDROID-359975090805611"" type=""a-f-G-U-C"" time=""2023-01-15T10:30:00.000Z"" start=""2023-01-15T10:30:00.000Z"" stale=""2023-01-15T10:35:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""34.12345"" lon=""-118.12345"" hae=""150.0"" ce=""10.0"" le=""20.0""/>
    <detail>
        <contact callsign=""ALPHA-1"" endpoint=""*:-1:stcp""/>
        <track speed=""0.0"" course=""0.0""/>
        <__group name=""Cyan"" role=""Team Member""/>
    </detail>
</event>";

    private const string SampleChatXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""GeoChat.ANDROID-359975090805611.TEAM-1.123456789"" type=""b-t-f"" time=""2023-01-15T10:30:00.000Z"" start=""2023-01-15T10:30:00.000Z"" stale=""2023-01-15T10:35:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""34.12345"" lon=""-118.12345"" hae=""150.0"" ce=""10.0"" le=""20.0""/>
    <detail>
        <__chat messageId=""123456789"" chatroom=""All Chat Rooms"" id=""All Chat Rooms"" parent=""RootContactGroup"">
            <chatgrp uid0=""ANDROID-359975090805611"" uid1=""All Chat Rooms"" id=""All Chat Rooms""/>
        </__chat>
        <link uid=""ANDROID-359975090805611"" type=""a-f-G-U-C"" relation=""p-p""/>
        <remarks source=""BAO.F.ATAK.ANDROID-359975090805611"" to=""All Chat Rooms"" time=""2023-01-15T10:30:00.000Z"">Hello Team!</remarks>
    </detail>
</event>";

    private const string SampleEmergencyXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""EMERGENCY-359975090805611"" type=""a-u-emergency-g"" time=""2023-01-15T10:30:00.000Z"" start=""2023-01-15T10:30:00.000Z"" stale=""2023-01-15T10:35:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""34.12345"" lon=""-118.12345"" hae=""150.0"" ce=""10.0"" le=""20.0""/>
    <detail>
        <emergency type=""911 Alert""/>
        <contact callsign=""EMERGENCY-USER""/>
    </detail>
</event>";

    [Test]
    public void ConvertFriendlyUnitToMapItemDocument()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer-123");

        // Assert
        Assert.That(result, Is.InstanceOf<MapItemDocument>());
        var mapItem = (MapItemDocument)result;

        Assert.That(mapItem.Id, Is.EqualTo("ANDROID-359975090805611"));
        Assert.That(mapItem.EventType, Is.EqualTo("a-f-G-U-C"));
        Assert.That(mapItem.PeerKey, Is.EqualTo("test-peer-123"));
        Assert.That(mapItem.AuthorUid, Is.EqualTo("ANDROID-359975090805611"));
        Assert.That(mapItem.Latitude, Is.EqualTo(34.12345).Within(0.0001));
        Assert.That(mapItem.Longitude, Is.EqualTo(-118.12345).Within(0.0001));
        Assert.That(mapItem.HeightAboveEllipsoid, Is.EqualTo(150.0).Within(0.0001));
        Assert.That(mapItem.CircularError, Is.EqualTo(10.0).Within(0.0001));
        Assert.That(mapItem.LinearError, Is.EqualTo(20.0).Within(0.0001));
        Assert.That(mapItem.How, Is.EqualTo("h-g-i-g-o"));
        Assert.That(mapItem.Version, Is.EqualTo(2));
        Assert.That(mapItem.Removed, Is.False);
        Assert.That(mapItem.Name, Is.EqualTo("ALPHA-1")); // Extracted from contact callsign
        Assert.That(mapItem.Visible, Is.True);
        Assert.That(mapItem.Source, Is.EqualTo("cot-converter-csharp"));

        // Verify detail fields were parsed
        Assert.That(mapItem.Detail, Is.Not.Empty);
        Assert.That(mapItem.DetailContainsKey("contact"), Is.True);
    }

    [Test]
    public void ConvertChatEventToChatDocument()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleChatXml, "test-peer-456");

        // Assert
        Assert.That(result, Is.InstanceOf<ChatDocument>());
        var chat = (ChatDocument)result;

        Assert.That(chat.Id, Is.EqualTo("GeoChat.ANDROID-359975090805611.TEAM-1.123456789"));
        Assert.That(chat.EventType, Is.EqualTo("b-t-f"));
        Assert.That(chat.PeerKey, Is.EqualTo("test-peer-456"));
        Assert.That(chat.AuthorUid, Is.EqualTo("GeoChat.ANDROID-359975090805611.TEAM-1.123456789"));
        Assert.That(chat.Message, Is.EqualTo("CoT Event: GeoChat.ANDROID-359975090805611.TEAM-1.123456789"));
        Assert.That(chat.Room, Is.EqualTo("cot-events"));
        Assert.That(chat.AuthorUidField, Is.EqualTo("GeoChat.ANDROID-359975090805611.TEAM-1.123456789"));
        Assert.That(chat.AuthorType, Is.EqualTo("b-t-f"));
        Assert.That(chat.Location, Is.EqualTo("34.12345,-118.12345,150"));
        Assert.That(chat.Version, Is.EqualTo(2));
        Assert.That(chat.Removed, Is.False);
    }

    [Test]
    public void ConvertEmergencyEventToApiDocument()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleEmergencyXml, "test-peer-789");

        // Assert
        Assert.That(result, Is.InstanceOf<ApiDocument>());
        var api = (ApiDocument)result;

        Assert.That(api.Id, Is.EqualTo("EMERGENCY-359975090805611"));
        Assert.That(api.EventType, Is.EqualTo("a-u-emergency-g"));
        Assert.That(api.PeerKey, Is.EqualTo("test-peer-789"));
        Assert.That(api.AuthorUid, Is.EqualTo("EMERGENCY-359975090805611"));
        Assert.That(api.Mime, Is.EqualTo("application/vnd.cot.emergency+json"));
        Assert.That(api.ContentType, Is.EqualTo("emergency"));
        Assert.That(api.IsFile, Is.False);
        Assert.That(api.IsRemoved, Is.False);
        Assert.That(api.Title, Is.EqualTo("CoT Event: EMERGENCY-359975090805611"));
        Assert.That(api.Data, Is.EqualTo("EMERGENCY-359975090805611"));
        Assert.That(api.Version, Is.EqualTo(2));
        Assert.That(api.Removed, Is.False);
    }

    [Test]
    public void ConvertCoTEventTimestamps()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer");

        // Assert
        Assert.That(result, Is.InstanceOf<MapItemDocument>());
        var mapItem = (MapItemDocument)result;

        // Verify timestamps are converted properly
        var expectedTime = DateTimeOffset.Parse("2023-01-15T10:30:00.000Z").ToUnixTimeMilliseconds();
        var expectedStart = DateTimeOffset.Parse("2023-01-15T10:30:00.000Z").ToUnixTimeMilliseconds() * 1000;
        var expectedStale = DateTimeOffset.Parse("2023-01-15T10:35:00.000Z").ToUnixTimeMilliseconds() * 1000;

        Assert.That(mapItem.TimeMillis, Is.EqualTo(expectedTime).Within(1000)); // Allow 1 second variance
        Assert.That(mapItem.StartTime, Is.EqualTo(expectedStart).Within(1000000)); // Allow 1 second variance in microseconds
        Assert.That(mapItem.StaleTime, Is.EqualTo(expectedStale).Within(1000000)); // Allow 1 second variance in microseconds
    }

    [Test]
    public void ConvertDetailSectionToMap()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer");

        // Assert
        Assert.That(result, Is.InstanceOf<MapItemDocument>());
        var mapItem = (MapItemDocument)result;

        // Verify detail map contains expected elements
        Assert.That(mapItem.DetailContainsKey("contact"), Is.True);
        Assert.That(mapItem.DetailContainsKey("track"), Is.True);
        Assert.That(mapItem.DetailContainsKey("__group"), Is.True);

        // Verify contact information
        var contactValue = mapItem.GetDetailValue("contact");
        Assert.That(contactValue, Is.Not.Null);
        
        // Handle both Dictionary and JObject types that can come from JSON deserialization
        if (contactValue is Newtonsoft.Json.Linq.JObject jObj)
        {
            Assert.That(jObj.ContainsKey("callsign"), Is.True);
        }
        else if (contactValue is Dictionary<string, object> dict)
        {
            Assert.That(dict.ContainsKey("callsign"), Is.True);
        }
        else
        {
            Assert.Fail($"Contact value was unexpected type: {contactValue?.GetType()}");
        }
    }

    [Test]
    public void ExtractCallsignFromDetail()
    {
        // Act
        var result = DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer");

        // Assert
        Assert.That(result, Is.InstanceOf<MapItemDocument>());
        var mapItem = (MapItemDocument)result;

        Assert.That(mapItem.Callsign, Is.EqualTo("ALPHA-1"));
    }

    [Test]
    public void ConvertUnknownTypeToGenericDocument()
    {
        var unknownXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""UNKNOWN-123"" type=""x-unknown-type"" time=""2023-01-15T10:30:00.000Z"" start=""2023-01-15T10:30:00.000Z"" stale=""2023-01-15T10:35:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""34.12345"" lon=""-118.12345"" hae=""150.0"" ce=""10.0"" le=""20.0""/>
    <detail></detail>
</event>";

        // Act
        var result = DocumentConverter.ConvertXmlToDocument(unknownXml, "test-peer");

        // Assert
        Assert.That(result, Is.InstanceOf<GenericDocument>());
        var generic = (GenericDocument)result;

        Assert.That(generic.Id, Is.EqualTo("UNKNOWN-123"));
        Assert.That(generic.EventType, Is.EqualTo("x-unknown-type"));
        Assert.That(generic.Version, Is.EqualTo(2));
        Assert.That(generic.Source, Is.EqualTo("cot-converter-csharp"));
    }

    [Test]
    public void ConvertFileEventToFileDocument()
    {
        var fileXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""FILE-123"" type=""b-f-t-a"" time=""2023-01-15T10:30:00.000Z"" start=""2023-01-15T10:30:00.000Z"" stale=""2023-01-15T10:35:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""34.12345"" lon=""-118.12345"" hae=""150.0"" ce=""10.0"" le=""20.0""/>
    <detail>
        <fileshare filename=""test.jpg"" size=""2048"" mime=""image/jpeg""/>
    </detail>
</event>";

        // Act
        var result = DocumentConverter.ConvertXmlToDocument(fileXml, "test-peer");

        // Assert
        Assert.That(result, Is.InstanceOf<FileDocument>());
        var file = (FileDocument)result;

        Assert.That(file.Id, Is.EqualTo("FILE-123"));
        Assert.That(file.EventType, Is.EqualTo("b-f-t-a"));
        Assert.That(file.FileName, Is.EqualTo("FILE-123.xml"));
        Assert.That(file.File, Is.EqualTo("FILE-123"));
        Assert.That(file.Mime, Is.EqualTo("application/xml"));
        Assert.That(file.ContentType, Is.EqualTo("file"));
        Assert.That(file.ItemId, Is.EqualTo("FILE-123"));
        Assert.That(file.Size, Is.EqualTo(1024.0));
    }

    [Test]
    public void ConvertDocumentToJson()
    {
        // Arrange
        var result = DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer");

        // Act
        var json = DocumentConverter.ConvertDocumentToJson(result);

        // Assert
        Assert.That(json, Is.Not.Null);
        Assert.That(json, Contains.Substring("test-peer"));
        Assert.That(json, Contains.Substring("ANDROID-359975090805611"));
        Assert.That(json, Contains.Substring("a-f-G-U-C"));

        // Verify it's valid JSON by parsing it
        Assert.DoesNotThrow(() => JsonConvert.DeserializeObject(json));
    }

    [Test]
    public void ConvertDocumentToMapAndBack()
    {
        // Arrange
        var originalDoc = (MapItemDocument)DocumentConverter.ConvertXmlToDocument(SampleFriendlyUnitXml, "test-peer");

        // Act - Convert to JSON and back
        var json = DocumentConverter.ConvertDocumentToJson(originalDoc);
        var reconstructedDoc = DocumentConverter.ConvertJsonToDocument<MapItemDocument>(json);

        // Assert
        Assert.That(reconstructedDoc.Id, Is.EqualTo(originalDoc.Id));
        Assert.That(reconstructedDoc.EventType, Is.EqualTo(originalDoc.EventType));
        Assert.That(reconstructedDoc.PeerKey, Is.EqualTo(originalDoc.PeerKey));
        Assert.That(reconstructedDoc.Latitude, Is.EqualTo(originalDoc.Latitude));
        Assert.That(reconstructedDoc.Longitude, Is.EqualTo(originalDoc.Longitude));
        Assert.That(reconstructedDoc.Version, Is.EqualTo(originalDoc.Version));
    }

    [Test]
    public void ParseInvalidXmlThrowsException()
    {
        var invalidXml = @"<invalid>xml content</not-closed>";

        // Act & Assert
        Assert.Throws<InvalidOperationException>(() => 
            DocumentConverter.ConvertXmlToDocument(invalidXml, "test-peer"));
    }

    [Test]
    public void ParseEmptyXmlThrowsException()
    {
        // Act & Assert
        Assert.Throws<InvalidOperationException>(() => 
            DocumentConverter.ConvertXmlToDocument("", "test-peer"));
    }
}