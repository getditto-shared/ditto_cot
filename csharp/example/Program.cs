using System;
using Ditto.Cot;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Ditto.Cot
{
    /// <summary>
    /// Integration client that outputs structured JSON for cross-language testing.
    /// This client processes the same CoT XML as the Rust and Java clients and outputs
    /// comparable JSON results for integration testing.
    /// </summary>
    public class IntegrationClient
    {
        public static void Main(string[] args)
        {
            try
            {
                // Create the same sample CoT XML as Rust and Java clients
                string cotXml = @"<?xml version=""1.0"" encoding=""UTF-8""?>
<event version=""2.0"" uid=""ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279"" type=""b-m-p-s-p-loc"" how=""h-e"" start=""2023-10-15T10:30:00.000Z"" time=""2023-10-15T10:30:00.000Z"" stale=""2023-10-15T10:35:00.000Z"">
    <point lat=""35.091"" lon=""-106.558"" hae=""1618.8"" ce=""3.2"" le=""5.8""/>
    <detail>
        <contact callsign=""PINKY"" endpoint=""192.168.1.10:4242:tcp""/>
        <__group name=""Blue"" role=""Team Member""/>
        <color argb=""-1""/>
        <usericon iconsetpath=""COT_MAPPING_SPOTMAP/b-m-p-s-p-loc/spy.png""/>
        <link uid=""ANDROID-GeoChat.ANDROID-R52JB0CDC4N2877-01.10279"" type=""b-m-p-s-p-loc"" relation=""p-p""/>
        <remarks>Equipment check complete</remarks>
        <status readiness=""true""/>
        <track speed=""12.5"" course=""45.0""/>
        <precisionlocation altsrc=""GPS""/>
    </detail>
</event>";

                // Convert XML to Ditto Document
                var dittoDocument = DocumentConverter.ConvertXmlToDocument(cotXml, "csharp-integration-test");
                
                // Convert back to XML
                string roundtripXml = DocumentConverter.ConvertDocumentToXml(dittoDocument);
                
                // Create structured output
                var output = new
                {
                    lang = "csharp",
                    original_xml = cotXml,
                    ditto_document = dittoDocument,
                    roundtrip_xml = roundtripXml,
                    success = true
                };
                
                // Output JSON to stdout
                string jsonOutput = JsonConvert.SerializeObject(output, Formatting.Indented);
                Console.WriteLine(jsonOutput);
            }
            catch (Exception e)
            {
                try
                {
                    // Output error in same JSON format
                    var errorOutput = new
                    {
                        lang = "csharp",
                        success = false,
                        error = e.Message
                    };
                    string jsonOutput = JsonConvert.SerializeObject(errorOutput, Formatting.Indented);
                    Console.WriteLine(jsonOutput);
                }
                catch (Exception jsonError)
                {
                    Console.Error.WriteLine($"Error in C# integration client: {e.Message}");
                    Console.Error.WriteLine(e.StackTrace);
                }
                Environment.Exit(1);
            }
        }
    }
}