using NUnit.Framework;
using Ditto.Cot;
using Ditto.Cot.Models;
using System;

namespace DittoCoTClient.Tests
{
    [TestFixture]
    public class TimestampDebugTest
    {
        [Test]
        public void Debug_Timestamp_Preservation()
        {
            // Test simple CoT XML with milliseconds
            string testXml = @"<event version=""2.0"" type=""a-f-G-U-C"" uid=""TEST-001"" 
                time=""2024-01-15T10:30:00.000Z"" start=""2024-01-15T10:30:00.000Z"" stale=""2024-01-15T11:30:00.000Z"" how=""m-g"">
                <point lat=""40.7128"" lon=""-74.0060"" hae=""10.0"" ce=""5.0"" le=""2.0""/>
                <detail>
                    <contact callsign=""ALPHA-1""/>
                </detail>
            </event>";

            Console.WriteLine("🔍 Original XML:");
            Console.WriteLine(testXml);

            // Step 1: Parse to CoTEvent
            var cotEvent = DocumentConverter.ParseCoTXml(testXml);
            Console.WriteLine($"\n📋 Parsed CoTEvent timestamps:");
            Console.WriteLine($"  Time: '{cotEvent.Time}'");
            Console.WriteLine($"  Start: '{cotEvent.Start}'");
            Console.WriteLine($"  Stale: '{cotEvent.Stale}'");

            // Step 2: Convert to Document
            var document = DocumentConverter.ConvertCoTEventToDocument(cotEvent, "test-peer");
            Console.WriteLine($"\n📋 Document as CommonDocument:");
            if (document is CommonDocument commonDoc)
            {
                Console.WriteLine($"  TimeOriginal: '{commonDoc.TimeOriginal}'");
                Console.WriteLine($"  StartOriginal: '{commonDoc.StartOriginal}'");
                Console.WriteLine($"  StaleOriginal: '{commonDoc.StaleOriginal}'");
                Console.WriteLine($"  TimeMillisFromEpoch: {commonDoc.TimeMillisFromEpoch}");
                Console.WriteLine($"  StartTime: {commonDoc.StartTime}");
                Console.WriteLine($"  StaleTime: {commonDoc.StaleTime}");
            }

            // Step 3: Convert back to CoTEvent
            CoTEvent reconverted = document switch
            {
                ApiDocument api => DocumentConverter.ConvertApiDocumentToCoTEvent(api),
                ChatDocument chat => DocumentConverter.ConvertChatDocumentToCoTEvent(chat),
                FileDocument file => DocumentConverter.ConvertFileDocumentToCoTEvent(file),
                GenericDocument generic => DocumentConverter.ConvertGenericDocumentToCoTEvent(generic),
                MapItemDocument mapItem => DocumentConverter.ConvertMapItemDocumentToCoTEvent(mapItem),
                _ => throw new ArgumentException($"Unknown document type: {document.GetType()}")
            };

            Console.WriteLine($"\n📋 Reconverted CoTEvent timestamps:");
            Console.WriteLine($"  Time: '{reconverted.Time}'");
            Console.WriteLine($"  Start: '{reconverted.Start}'");
            Console.WriteLine($"  Stale: '{reconverted.Stale}'");

            // Step 4: Convert to XML
            var finalXml = DocumentConverter.ConvertCoTEventToXml(reconverted);
            Console.WriteLine($"\n🔍 Final XML:");
            Console.WriteLine(finalXml);

            // Validate preservation
            Assert.That(reconverted.Time, Is.EqualTo(cotEvent.Time), "Time should be preserved exactly");
            Assert.That(reconverted.Start, Is.EqualTo(cotEvent.Start), "Start should be preserved exactly");
            Assert.That(reconverted.Stale, Is.EqualTo(cotEvent.Stale), "Stale should be preserved exactly");
        }
    }
}