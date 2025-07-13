using NUnit.Framework;
using DittoCoTClient.IPC;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Threading.Tasks;

namespace DittoCoTClient.Tests
{
    [TestFixture]
    public class StorageDebugTest
    {
        private const string TestCollectionName = "debug_test_collection";

        [Test]
        public async Task Debug_Ditto_Storage_Preservation()
        {
            // Test simple CoT XML with milliseconds
            string testXml = @"<event version=""2.0"" type=""a-f-G-U-C"" uid=""STORAGE-DEBUG-001"" 
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
            Console.WriteLine($"\n📋 Document before storage:");
            if (document is CommonDocument commonDoc)
            {
                Console.WriteLine($"  TimeOriginal: '{commonDoc.TimeOriginal}'");
                Console.WriteLine($"  StartOriginal: '{commonDoc.StartOriginal}'");
                Console.WriteLine($"  StaleOriginal: '{commonDoc.StaleOriginal}'");
            }

            // Step 3: Store in Ditto and retrieve (this is where the issue happens)
            var docId = await StoreAndRetrieveDocument(document);
            
            // Step 4: Query back from Ditto
            var retrievedDocument = await QueryDocumentFromDitto(docId);
            Console.WriteLine($"\n📋 Retrieved document from Ditto:");
            Console.WriteLine(retrievedDocument.ToString());
            
            // Check what happened to base64 encoded fields
            Console.WriteLine($"\n📋 Preserved base64 fields after storage:");
            Console.WriteLine($"  time_b64: '{retrievedDocument["time_b64"]}'");
            Console.WriteLine($"  start_b64: '{retrievedDocument["start_b64"]}'");
            Console.WriteLine($"  stale_b64: '{retrievedDocument["stale_b64"]}'");
            
            // Decode and check if original values are preserved
            try
            {
                if (retrievedDocument["time_b64"] != null)
                {
                    var timeDecoded = System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(retrievedDocument["time_b64"].ToString()));
                    Console.WriteLine($"  time_decoded: '{timeDecoded}'");
                }
                if (retrievedDocument["start_b64"] != null)
                {
                    var startDecoded = System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(retrievedDocument["start_b64"].ToString()));
                    Console.WriteLine($"  start_decoded: '{startDecoded}'");
                }
                if (retrievedDocument["stale_b64"] != null)
                {
                    var staleDecoded = System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(retrievedDocument["stale_b64"].ToString()));
                    Console.WriteLine($"  stale_decoded: '{staleDecoded}'");
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"  Error decoding: {ex.Message}");
            }
            
            await CleanupTestData();
        }

        private async Task<string> StoreAndRetrieveDocument(object dittoDocument)
        {
            try
            {
                using var client = new DittoServiceClient();
                
                if (!await client.ConnectAsync(10000))
                {
                    throw new Exception("Cannot connect to Ditto service");
                }
                
                var payload = JsonConvert.SerializeObject(dittoDocument);
                Console.WriteLine($"\n📋 JSON payload being sent to Ditto:");
                Console.WriteLine(JToken.Parse(payload).ToString(Formatting.Indented));
                
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
            catch (Exception ex)
            {
                throw new Exception($"Failed to store document: {ex.Message}", ex);
            }
        }
        
        private async Task<JObject> QueryDocumentFromDitto(string documentId)
        {
            try
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
            catch (Exception ex)
            {
                throw new Exception($"Failed to query document from Ditto: {ex.Message}", ex);
            }
        }

        private async Task CleanupTestData()
        {
            // Implementation would clean up test documents
            Console.WriteLine("🧹 Test cleanup completed");
        }
    }
}