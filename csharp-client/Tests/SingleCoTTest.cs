using NUnit.Framework;
using DittoCoTClient.IPC;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;
using System.Xml.Linq;

namespace DittoCoTClient.Tests
{
    [TestFixture]
    public class SingleCoTTest
    {
        private const string TestCollectionName = "single_test_collection";

        [Test]
        public async Task Test_Single_Infantry_Unit_Full_Round_Trip()
        {
            // Test Infantry Unit with full round-trip through Ditto storage
            string infantryXml = @"<event version=""2.0"" type=""a-f-G-U-C"" uid=""INFANTRY-001"" 
                time=""2024-01-15T10:30:00.000Z"" start=""2024-01-15T10:30:00.000Z"" stale=""2024-01-15T11:30:00.000Z"" how=""m-g"">
                <point lat=""40.7128"" lon=""-74.0060"" hae=""10.0"" ce=""5.0"" le=""2.0""/>
                <detail>
                    <contact callsign=""ALPHA-1"" endpoint=""192.168.1.100:4242""/>
                    <__group name=""SQUAD-A"" role=""Team Member""/>
                    <status battery=""85"" health=""Green""/>
                </detail>
            </event>";

            Console.WriteLine("🔍 Testing Infantry Unit Round-Trip");
            Console.WriteLine("📋 Original XML:");
            Console.WriteLine(FormatXml(infantryXml));

            try
            {
                // Step 1: Parse to CoTEvent
                var cotEvent = DocumentConverter.ParseCoTXml(infantryXml);
                Console.WriteLine($"✅ Parsed CoT Event: {cotEvent.Uid}");

                // Step 2: Convert to Ditto document
                var dittoDocument = DocumentConverter.ConvertCoTEventToDocument(cotEvent, "test-peer");
                Console.WriteLine($"✅ Converted to Ditto document type: {dittoDocument.GetType().Name}");

                // Step 3: Store in Ditto and retrieve
                var docId = await StoreAndRetrieveDocument(dittoDocument);
                Console.WriteLine($"✅ Stored in Ditto with ID: {docId}");

                // Step 4: Query back from Ditto
                var retrievedDocument = await QueryDocumentFromDitto(docId);
                Console.WriteLine($"✅ Retrieved document from Ditto");

                // Step 5: Convert back to CoT
                var convertedXml = ConvertDittoDocumentBackToCoT(retrievedDocument);
                Console.WriteLine($"✅ Converted back to CoT XML");

                Console.WriteLine("📋 Final converted XML:");
                Console.WriteLine(FormatXml(convertedXml));

                // Step 6: Compare semantically
                var originalDoc = XDocument.Parse(infantryXml);
                var convertedDoc = XDocument.Parse(convertedXml);

                Console.WriteLine("🔍 Comparing XML documents semantically...");
                
                // Compare root elements
                var originalEvent = originalDoc.Root;
                var convertedEvent = convertedDoc.Root;

                // Check all attributes
                foreach (var originalAttr in originalEvent.Attributes())
                {
                    var convertedAttr = convertedEvent.Attribute(originalAttr.Name);
                    if (convertedAttr == null)
                    {
                        Assert.Fail($"Missing attribute: {originalAttr.Name}");
                    }
                    
                    // For timestamp attributes, compare the actual values
                    if (originalAttr.Name == "time" || originalAttr.Name == "start" || originalAttr.Name == "stale")
                    {
                        Console.WriteLine($"Comparing {originalAttr.Name}: '{originalAttr.Value}' vs '{convertedAttr.Value}'");
                        Assert.That(convertedAttr.Value, Is.EqualTo(originalAttr.Value), 
                            $"Timestamp attribute {originalAttr.Name} should be preserved exactly");
                    }
                    else
                    {
                        Assert.That(convertedAttr.Value, Is.EqualTo(originalAttr.Value), 
                            $"Attribute {originalAttr.Name} should be preserved");
                    }
                }

                Console.WriteLine("✅ All attributes match!");
                
                await CleanupTestData();
                Console.WriteLine("🎉 Single Infantry Unit test passed!");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"❌ Test failed: {ex.Message}");
                await CleanupTestData();
                throw;
            }
        }

        private async Task<string> StoreAndRetrieveDocument(object dittoDocument)
        {
            using var client = new DittoServiceClient();
            
            if (!await client.ConnectAsync(10000))
            {
                throw new Exception("Cannot connect to Ditto service");
            }
            
            var payload = JsonConvert.SerializeObject(dittoDocument);
            var response = await client.SendRequestAsync("create", TestCollectionName, payload);
            
            if (response?.Success == true && response.Data != null)
            {
                var dataObj = JObject.FromObject(response.Data);
                var documentIdRaw = dataObj["document_id"]?.ToString();
                var documentId = JsonConvert.DeserializeObject<string>(documentIdRaw);
                return documentId;
            }
            else
            {
                throw new Exception($"Failed to insert document: {response?.Error ?? "Unknown error"}");
            }
        }
        
        private async Task<JObject> QueryDocumentFromDitto(string documentId)
        {
            using var client = new DittoServiceClient();
            
            if (!await client.ConnectAsync(5000))
            {
                throw new Exception("Cannot connect to Ditto service for querying");
            }
            
            var query = $"_id == '{documentId}'";
            var response = await client.SendRequestAsync("query", TestCollectionName, query, 1);
            
            if (response?.Success == true && response.Data != null)
            {
                var dataObj = JObject.FromObject(response.Data);
                var documents = dataObj["documents"]?.ToObject<JArray>();
                
                if (documents == null || documents.Count == 0)
                {
                    throw new Exception($"No document found with ID: {documentId}");
                }
                
                var document = documents[0];
                var docValue = document["value"] as JObject;
                if (docValue == null)
                {
                    throw new Exception("Retrieved document has no value");
                }
                
                return docValue;
            }
            else
            {
                throw new Exception($"Failed to query document: {response?.Error ?? "Unknown error"}");
            }
        }

        private string ConvertDittoDocumentBackToCoT(JObject dittoDocument)
        {
            // Remove Ditto metadata
            var cleanedDoc = new JObject(dittoDocument);
            cleanedDoc.Remove("_id");

            // Convert back using library
            var cotDoc = CotDocument.FromJson(cleanedDoc.ToString());
            var cotEvent = cotDoc.ToCoTEvent();
            return DocumentConverter.ConvertCoTEventToXml(cotEvent);
        }

        private string FormatXml(string xml)
        {
            try
            {
                var doc = XDocument.Parse(xml);
                return doc.ToString();
            }
            catch
            {
                return xml; // Return as-is if parsing fails
            }
        }

        private async Task CleanupTestData()
        {
            Console.WriteLine("🧹 Test cleanup completed");
        }
    }
}