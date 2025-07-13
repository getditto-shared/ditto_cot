using NUnit.Framework;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;

namespace Ditto.Cot.Tests;

[TestFixture]
public class DittoToCoTConversionTests
{
    [Test]
    public void ConvertMapItemDocumentToCoTEvent()
    {
        // Arrange
        var mapItem = new MapItemDocument
        {
            Id = "TEST-UNIT-123",
            Counter = 1,
            Version = 2,
            Removed = false,
            PeerKey = "test-peer-456",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T10:30:00.000Z").ToUnixTimeMilliseconds(),
            AuthorUid = "TEST-UNIT-123",
            Callsign = "BRAVO-2",
            VersionString = "2.0",
            CircularError = 5.0,
            HeightAboveEllipsoid = 200.0,
            Latitude = 35.0,
            LinearError = 10.0,
            Longitude = -119.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T10:30:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T10:35:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            EventType = "a-f-G-U-C",
            Detail = new Dictionary<string, object>
            {
                ["contact"] = new Dictionary<string, object>
                {
                    ["callsign"] = "BRAVO-2",
                    ["endpoint"] = "*:-1:stcp"
                },
                ["track"] = new Dictionary<string, object>
                {
                    ["speed"] = "0.0",
                    ["course"] = "0.0"
                }
            },
            Name = "BRAVO-2",
            Visible = true,
            Source = "cot-converter-csharp"
        };

        // Act
        var cotEvent = DocumentConverter.ConvertMapItemDocumentToCoTEvent(mapItem);

        // Assert
        Assert.That(cotEvent.Version, Is.EqualTo("2.0"));
        Assert.That(cotEvent.Uid, Is.EqualTo("TEST-UNIT-123"));
        Assert.That(cotEvent.Type, Is.EqualTo("a-f-G-U-C"));
        Assert.That(cotEvent.How, Is.EqualTo("h-g-i-g-o"));

        // Verify timestamps
        Assert.That(cotEvent.Time, Contains.Substring("2023-01-15T10:30:00"));
        Assert.That(cotEvent.Start, Contains.Substring("2023-01-15T10:30:00"));
        Assert.That(cotEvent.Stale, Contains.Substring("2023-01-15T10:35:00"));

        // Verify point data
        Assert.That(cotEvent.Point.LatDouble, Is.EqualTo(35.0).Within(0.0001));
        Assert.That(cotEvent.Point.LonDouble, Is.EqualTo(-119.0).Within(0.0001));
        Assert.That(cotEvent.Point.HaeDouble, Is.EqualTo(200.0).Within(0.0001));
        Assert.That(cotEvent.Point.CeDouble, Is.EqualTo(5.0).Within(0.0001));
        Assert.That(cotEvent.Point.LeDouble, Is.EqualTo(10.0).Within(0.0001));

        // Verify detail conversion
        Assert.That(cotEvent.Detail, Is.Not.Null);
        Assert.That(cotEvent.Detail.RawXml, Contains.Substring("contact"));
    }

    [Test]
    public void ConvertChatDocumentToCoTEvent()
    {
        // Arrange
        var chat = new ChatDocument
        {
            Id = "CHAT-MESSAGE-456",
            Counter = 1,
            Version = 2,
            Removed = false,
            PeerKey = "test-peer-789",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T11:00:00.000Z").ToUnixTimeMilliseconds(),
            AuthorUid = "USER-123",
            Callsign = "CHARLIE-3",
            VersionString = "2.0",
            CircularError = 3.0,
            HeightAboveEllipsoid = 100.0,
            Latitude = 36.0,
            LinearError = 5.0,
            Longitude = -120.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T11:00:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T11:05:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            EventType = "b-t-f",
            Detail = new Dictionary<string, object>
            {
                ["__chat"] = new Dictionary<string, object>
                {
                    ["messageId"] = "456",
                    ["chatroom"] = "Team Alpha"
                },
                ["remarks"] = new Dictionary<string, object>
                {
                    ["source"] = "USER-123",
                    ["to"] = "Team Alpha"
                }
            },
            Message = "Hello team, status update from Charlie-3",
            Room = "Team Alpha",
            RoomId = "team-alpha-room",
            AuthorCallsign = "CHARLIE-3",
            AuthorUidField = "USER-123",
            AuthorType = "a-f-G-U-C",
            Time = "2023-01-15T11:00:00.000Z",
            Location = "36.0,-120.0,100.0",
            Source = "cot-converter-csharp"
        };

        // Act
        var cotEvent = DocumentConverter.ConvertChatDocumentToCoTEvent(chat);

        // Assert
        Assert.That(cotEvent.Version, Is.EqualTo("2.0"));
        Assert.That(cotEvent.Uid, Is.EqualTo("CHAT-MESSAGE-456"));
        Assert.That(cotEvent.Type, Is.EqualTo("b-t-f"));
        Assert.That(cotEvent.How, Is.EqualTo("h-g-i-g-o"));

        // Verify point data
        Assert.That(cotEvent.Point.LatDouble, Is.EqualTo(36.0).Within(0.0001));
        Assert.That(cotEvent.Point.LonDouble, Is.EqualTo(-120.0).Within(0.0001));
        Assert.That(cotEvent.Point.HaeDouble, Is.EqualTo(100.0).Within(0.0001));
    }

    [Test]
    public void ConvertApiDocumentToCoTEvent()
    {
        // Arrange
        var api = new ApiDocument
        {
            Id = "EMERGENCY-789",
            Counter = 1,
            Version = 2,
            Removed = false,
            PeerKey = "test-peer-emergency",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T12:00:00.000Z").ToUnixTimeMilliseconds(),
            AuthorUid = "EMERGENCY-USER-789",
            Callsign = "EMERGENCY-RESPONDER",
            VersionString = "2.0",
            CircularError = 2.0,
            HeightAboveEllipsoid = 50.0,
            Latitude = 37.0,
            LinearError = 3.0,
            Longitude = -121.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T12:00:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T12:30:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            EventType = "a-u-emergency-g",
            Detail = new Dictionary<string, object>
            {
                ["emergency"] = new Dictionary<string, object>
                {
                    ["type"] = "911 Alert"
                }
            },
            IsFile = false,
            Title = "Emergency Alert",
            Mime = "application/vnd.cot.emergency+json",
            ContentType = "emergency",
            Data = "Emergency data",
            IsRemoved = false,
            TimeMillisField = DateTimeOffset.Parse("2023-01-15T12:00:00.000Z").ToUnixTimeMilliseconds(),
            Source = "cot-converter-csharp"
        };

        // Act
        var cotEvent = DocumentConverter.ConvertApiDocumentToCoTEvent(api);

        // Assert
        Assert.That(cotEvent.Version, Is.EqualTo("2.0"));
        Assert.That(cotEvent.Uid, Is.EqualTo("EMERGENCY-789"));
        Assert.That(cotEvent.Type, Is.EqualTo("a-u-emergency-g"));
        Assert.That(cotEvent.How, Is.EqualTo("h-g-i-g-o"));

        // Verify point data
        Assert.That(cotEvent.Point.LatDouble, Is.EqualTo(37.0).Within(0.0001));
        Assert.That(cotEvent.Point.LonDouble, Is.EqualTo(-121.0).Within(0.0001));
        Assert.That(cotEvent.Point.HaeDouble, Is.EqualTo(50.0).Within(0.0001));
        Assert.That(cotEvent.Point.CeDouble, Is.EqualTo(2.0).Within(0.0001));
        Assert.That(cotEvent.Point.LeDouble, Is.EqualTo(3.0).Within(0.0001));
    }

    [Test]
    public void ConvertFileDocumentToCoTEvent()
    {
        // Arrange
        var file = new FileDocument
        {
            Id = "FILE-SHARE-101",
            Counter = 1,
            Version = 2,
            Removed = false,
            PeerKey = "test-peer-file",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T13:00:00.000Z").ToUnixTimeMilliseconds(),
            AuthorUid = "FILE-SENDER-101",
            Callsign = "FILE-USER",
            VersionString = "2.0",
            CircularError = 1.0,
            HeightAboveEllipsoid = 75.0,
            Latitude = 38.0,
            LinearError = 2.0,
            Longitude = -122.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T13:00:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T13:30:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            EventType = "b-f-t-a",
            Detail = new Dictionary<string, object>
            {
                ["fileshare"] = new Dictionary<string, object>
                {
                    ["filename"] = "document.pdf",
                    ["size"] = "2048",
                    ["mime"] = "application/pdf"
                }
            },
            FileName = "document.pdf",
            File = "FILE-SHARE-101",
            Mime = "application/pdf",
            ContentType = "file",
            ItemId = "FILE-SHARE-101",
            Size = 2048.0,
            Source = "cot-converter-csharp"
        };

        // Act
        var cotEvent = DocumentConverter.ConvertFileDocumentToCoTEvent(file);

        // Assert
        Assert.That(cotEvent.Version, Is.EqualTo("2.0"));
        Assert.That(cotEvent.Uid, Is.EqualTo("FILE-SHARE-101"));
        Assert.That(cotEvent.Type, Is.EqualTo("b-f-t-a"));
        Assert.That(cotEvent.How, Is.EqualTo("h-g-i-g-o"));

        // Verify point data
        Assert.That(cotEvent.Point.LatDouble, Is.EqualTo(38.0).Within(0.0001));
        Assert.That(cotEvent.Point.LonDouble, Is.EqualTo(-122.0).Within(0.0001));
    }

    [Test]
    public void ConvertGenericDocumentToCoTEvent()
    {
        // Arrange
        var generic = new GenericDocument
        {
            Id = "GENERIC-EVENT-202",
            Counter = 1,
            Version = 2,
            Removed = false,
            PeerKey = "test-peer-generic",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T14:00:00.000Z").ToUnixTimeMilliseconds(),
            AuthorUid = "GENERIC-USER-202",
            Callsign = "GENERIC-STATION",
            VersionString = "2.0",
            CircularError = 8.0,
            HeightAboveEllipsoid = 300.0,
            Latitude = 39.0,
            LinearError = 12.0,
            Longitude = -123.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T14:00:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T14:15:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            EventType = "x-custom-type",
            Detail = new Dictionary<string, object>
            {
                ["customData"] = new Dictionary<string, object>
                {
                    ["field1"] = "value1",
                    ["field2"] = "value2"
                }
            },
            Source = "cot-converter-csharp"
        };

        // Act
        var cotEvent = DocumentConverter.ConvertGenericDocumentToCoTEvent(generic);

        // Assert
        Assert.That(cotEvent.Version, Is.EqualTo("2.0"));
        Assert.That(cotEvent.Uid, Is.EqualTo("GENERIC-EVENT-202"));
        Assert.That(cotEvent.Type, Is.EqualTo("x-custom-type"));
        Assert.That(cotEvent.How, Is.EqualTo("h-g-i-g-o"));

        // Verify point data
        Assert.That(cotEvent.Point.LatDouble, Is.EqualTo(39.0).Within(0.0001));
        Assert.That(cotEvent.Point.LonDouble, Is.EqualTo(-123.0).Within(0.0001));
        Assert.That(cotEvent.Point.HaeDouble, Is.EqualTo(300.0).Within(0.0001));
        Assert.That(cotEvent.Point.CeDouble, Is.EqualTo(8.0).Within(0.0001));
        Assert.That(cotEvent.Point.LeDouble, Is.EqualTo(12.0).Within(0.0001));
    }

    [Test]
    public void ConvertDocumentToXml()
    {
        // Arrange
        var mapItem = new MapItemDocument
        {
            Id = "XML-TEST-303",
            EventType = "a-f-G-U-C",
            PeerKey = "test-peer",
            AuthorUid = "XML-TEST-303",
            Callsign = "XML-TESTER",
            VersionString = "2.0",
            CircularError = 5.0,
            HeightAboveEllipsoid = 150.0,
            Latitude = 40.0,
            LinearError = 8.0,
            Longitude = -124.0,
            StartTime = DateTimeOffset.Parse("2023-01-15T15:00:00.000Z").ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.Parse("2023-01-15T15:05:00.000Z").ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            TimeMillisFromEpoch = DateTimeOffset.Parse("2023-01-15T15:00:00.000Z").ToUnixTimeMilliseconds(),
            Detail = new Dictionary<string, object>(),
            Name = "XML-TESTER",
            Visible = true
        };

        // Act
        var xml = DocumentConverter.ConvertDocumentToXml(mapItem);

        // Assert
        Assert.That(xml, Is.Not.Null);
        Assert.That(xml, Contains.Substring("XML-TEST-303"));
        Assert.That(xml, Contains.Substring("a-f-G-U-C"));
        Assert.That(xml, Contains.Substring("lat=\"40\""));
        Assert.That(xml, Contains.Substring("lon=\"-124\""));
        Assert.That(xml, Contains.Substring("hae=\"150\""));
        Assert.That(xml, Contains.Substring("ce=\"5\""));
        Assert.That(xml, Contains.Substring("le=\"8\""));
        Assert.That(xml, Contains.Substring("<event"));
        Assert.That(xml, Contains.Substring("<point"));
        Assert.That(xml, Contains.Substring("<detail"));
    }

    [Test]
    public void RoundTripConversion_MapItemDocument()
    {
        // Arrange
        var originalXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""ROUNDTRIP-TEST-404"" type=""a-f-G-U-C"" time=""2023-01-15T16:00:00.000Z"" start=""2023-01-15T16:00:00.000Z"" stale=""2023-01-15T16:05:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""41.0"" lon=""-125.0"" hae=""250.0"" ce=""6.0"" le=""9.0""/>
    <detail>
        <contact callsign=""ROUNDTRIP-UNIT"" endpoint=""*:-1:stcp""/>
    </detail>
</event>";

        // Act - Convert to Ditto document and back to XML
        var dittoDoc = DocumentConverter.ConvertXmlToDocument(originalXml, "roundtrip-peer");
        var reconstructedXml = DocumentConverter.ConvertDocumentToXml(dittoDoc);

        // Assert - Key elements should be preserved
        Assert.That(reconstructedXml, Contains.Substring("ROUNDTRIP-TEST-404"));
        Assert.That(reconstructedXml, Contains.Substring("a-f-G-U-C"));
        Assert.That(reconstructedXml, Contains.Substring("lat=\"41"));
        Assert.That(reconstructedXml, Contains.Substring("lon=\"-125"));
        Assert.That(reconstructedXml, Contains.Substring("hae=\"250"));
        Assert.That(reconstructedXml, Contains.Substring("ce=\"6"));
        Assert.That(reconstructedXml, Contains.Substring("le=\"9"));

        // Verify we can parse the reconstructed XML
        Assert.DoesNotThrow(() => DocumentConverter.ParseCoTXml(reconstructedXml));
    }

    [Test]
    public void RoundTripConversion_ChatDocument()
    {
        // Arrange
        var originalXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""CHAT-ROUNDTRIP-505"" type=""b-t-f"" time=""2023-01-15T17:00:00.000Z"" start=""2023-01-15T17:00:00.000Z"" stale=""2023-01-15T17:05:00.000Z"" how=""h-g-i-g-o"">
    <point lat=""42.0"" lon=""-126.0"" hae=""100.0"" ce=""3.0"" le=""4.0""/>
    <detail>
        <__chat messageId=""12345"" chatroom=""Test Room""/>
        <remarks>Test message for roundtrip</remarks>
    </detail>
</event>";

        // Act - Convert to Ditto document and back to XML
        var dittoDoc = DocumentConverter.ConvertXmlToDocument(originalXml, "chat-roundtrip-peer");
        var reconstructedXml = DocumentConverter.ConvertDocumentToXml(dittoDoc);

        // Assert - Key elements should be preserved
        Assert.That(reconstructedXml, Contains.Substring("CHAT-ROUNDTRIP-505"));
        Assert.That(reconstructedXml, Contains.Substring("b-t-f"));
        Assert.That(reconstructedXml, Contains.Substring("lat=\"42"));
        Assert.That(reconstructedXml, Contains.Substring("lon=\"-126"));

        // Verify we can parse the reconstructed XML
        Assert.DoesNotThrow(() => DocumentConverter.ParseCoTXml(reconstructedXml));
    }

    [Test]
    public void ConvertDocumentWithNullValues()
    {
        // Arrange - Create document with null optional fields
        var mapItem = new MapItemDocument
        {
            Id = "NULL-TEST-606",
            EventType = "a-f-G-U-C",
            PeerKey = "test-peer",
            AuthorUid = "NULL-TEST-606",
            Callsign = "",
            VersionString = "2.0",
            CircularError = null, // Null value
            HeightAboveEllipsoid = null, // Null value
            Latitude = 43.0,
            LinearError = null, // Null value
            Longitude = -127.0,
            StartTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.UtcNow.AddMinutes(5).ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            TimeMillisFromEpoch = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            Detail = new Dictionary<string, object>(),
            Name = null, // Null value
            Visible = null // Null value
        };

        // Act & Assert - Should not throw exceptions
        Assert.DoesNotThrow(() =>
        {
            var cotEvent = DocumentConverter.ConvertMapItemDocumentToCoTEvent(mapItem);
            var xml = DocumentConverter.ConvertDocumentToXml(mapItem);
            
            // Should have default values for nulls
            Assert.That(cotEvent.Point.CeDouble, Is.EqualTo(0.0));
            Assert.That(cotEvent.Point.HaeDouble, Is.EqualTo(0.0));
            Assert.That(cotEvent.Point.LeDouble, Is.EqualTo(0.0));
        });
    }

    [Test]
    public void ConvertDocumentWithEmptyDetail()
    {
        // Arrange
        var generic = new GenericDocument
        {
            Id = "EMPTY-DETAIL-707",
            EventType = "x-test-empty",
            PeerKey = "test-peer",
            AuthorUid = "EMPTY-DETAIL-707",
            Callsign = "EMPTY-TESTER",
            VersionString = "2.0",
            CircularError = 1.0,
            HeightAboveEllipsoid = 0.0,
            Latitude = 44.0,
            LinearError = 1.0,
            Longitude = -128.0,
            StartTime = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds() * 1000,
            StaleTime = DateTimeOffset.UtcNow.AddMinutes(5).ToUnixTimeMilliseconds() * 1000,
            How = "h-g-i-g-o",
            TimeMillisFromEpoch = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            Detail = new Dictionary<string, object>() // Empty detail
        };

        // Act
        var cotEvent = DocumentConverter.ConvertGenericDocumentToCoTEvent(generic);
        var xml = DocumentConverter.ConvertDocumentToXml(generic);

        // Assert
        Assert.That(cotEvent.Detail, Is.Not.Null);
        Assert.That(xml, Contains.Substring("<detail"));
        Assert.That(xml, Contains.Substring("EMPTY-DETAIL-707"));
    }
}