using System.CommandLine;
using DittoCoTClient.IPC;
using DittoCoTClient.Config;
using Ditto.Cot;
using Ditto.Cot.Models;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace DittoCoTClient;

class Program
{
    static async Task<int> Main(string[] args)
    {
        var rootCommand = new RootCommand("Ditto CoT Client - Query and convert Ditto documents to CoT format");

        // Common options
        var appIdOption = new Option<string>(
            name: "--app-id",
            description: "Ditto App ID",
            getDefaultValue: () => "ditto-cot-client");
        
        var sharedKeyOption = new Option<string?>(
            name: "--shared-key",
            description: "Ditto Shared Key for authentication");
        
        var tokenOption = new Option<string?>(
            name: "--token",
            description: "Ditto token (works for both offline and online modes)");
        
        
        var offlineOption = new Option<bool>(
            name: "--offline",
            description: "Run in offline-only mode",
            getDefaultValue: () => false);

        // Configuration commands
        var configCommand = new Command("config", "Manage client configuration");
        
        var configSetCommand = new Command("set", "Set configuration values");
        configSetCommand.AddOption(appIdOption);
        configSetCommand.AddOption(sharedKeyOption);
        configSetCommand.AddOption(tokenOption);
        configSetCommand.AddOption(offlineOption);
        
        configSetCommand.SetHandler(async (string appId, string? sharedKey, string? token, bool offline) =>
        {
            await SetConfigCommand(appId, sharedKey, token, offline);
        }, appIdOption, sharedKeyOption, tokenOption, offlineOption);

        var configShowCommand = new Command("show", "Show current configuration");
        configShowCommand.SetHandler(ShowConfigCommand);

        var configDeleteCommand = new Command("delete", "Delete configuration file");
        configDeleteCommand.SetHandler(DeleteConfigCommand);

        var configObserversCommand = new Command("observers", "Configure observers");
        
        var observersEnableOption = new Option<bool?>(
            name: "--enable",
            description: "Enable or disable observers");
        
        var observersCollectionsOption = new Option<string[]?>(
            name: "--collections",
            description: "Collections to observe (e.g., mapItem mapItems track)");
        
        var observersLogChangesOption = new Option<bool?>(
            name: "--log-changes",
            description: "Enable logging of observer changes");
        
        var observersVerboseLoggingOption = new Option<bool?>(
            name: "--verbose-logging",
            description: "Enable detailed logging of all document content (default: false, shows summary only)");
        
        configObserversCommand.AddOption(observersEnableOption);
        configObserversCommand.AddOption(observersCollectionsOption);
        configObserversCommand.AddOption(observersLogChangesOption);
        configObserversCommand.AddOption(observersVerboseLoggingOption);
        
        configObserversCommand.SetHandler(async (bool? enable, string[]? collections, bool? logChanges, bool? verboseLogging) =>
        {
            await ConfigureObserversCommand(enable, collections, logChanges, verboseLogging);
        }, observersEnableOption, observersCollectionsOption, observersLogChangesOption, observersVerboseLoggingOption);

        configCommand.AddCommand(configSetCommand);
        configCommand.AddCommand(configShowCommand);
        configCommand.AddCommand(configDeleteCommand);
        configCommand.AddCommand(configObserversCommand);

        // Service management commands
        var serviceCommand = new Command("service", "Manage the Ditto background service");
        
        var startServiceCommand = new Command("start", "Start the Ditto background service (uses config if no options provided)");
        startServiceCommand.AddOption(appIdOption);
        startServiceCommand.AddOption(sharedKeyOption);
        startServiceCommand.AddOption(tokenOption);
        startServiceCommand.AddOption(offlineOption);
        
        startServiceCommand.SetHandler(async (string appId, string? sharedKey, string? token, bool offline) =>
        {
            await StartServiceCommand(appId, sharedKey, token, offline);
        }, appIdOption, sharedKeyOption, tokenOption, offlineOption);

        var stopServiceCommand = new Command("stop", "Stop the Ditto background service");
        stopServiceCommand.SetHandler(StopServiceCommand);

        var statusServiceCommand = new Command("status", "Check the status of the Ditto background service");
        statusServiceCommand.SetHandler(async () => await StatusServiceCommand());

        serviceCommand.AddCommand(startServiceCommand);
        serviceCommand.AddCommand(stopServiceCommand);
        serviceCommand.AddCommand(statusServiceCommand);

        // Query commands (now use IPC)
        var collectionOption = new Option<string>(
            name: "--collection",
            description: "The Ditto collection to query",
            getDefaultValue: () => "cot_events");
        
        var limitOption = new Option<int>(
            name: "--limit",
            description: "Maximum number of documents to retrieve",
            getDefaultValue: () => 10);
        
        var asCotOption = new Option<bool>(
            name: "--as-cot",
            description: "Convert Ditto documents to CoT XML format",
            getDefaultValue: () => false);

        var listCommand = new Command("list", "List all documents in a collection");
        listCommand.AddOption(collectionOption);
        listCommand.AddOption(limitOption);
        listCommand.AddOption(asCotOption);

        listCommand.SetHandler(async (string collection, int limit, bool asCot) =>
        {
            await ListDocuments(collection, limit, asCot);
        }, collectionOption, limitOption, asCotOption);

        var queryCommand = new Command("query", "Query documents with a specific filter");
        var queryOption = new Option<string>(
            name: "--query",
            description: "DQL query string (e.g., 'w == \"a-f-G-U-C\"')",
            getDefaultValue: () => "true");

        queryCommand.AddOption(collectionOption);
        queryCommand.AddOption(queryOption);
        queryCommand.AddOption(limitOption);
        queryCommand.AddOption(asCotOption);

        queryCommand.SetHandler(async (string collection, string query, int limit, bool asCot) =>
        {
            await QueryDocuments(collection, query, limit, asCot);
        }, collectionOption, queryOption, limitOption, asCotOption);

        // Create command
        var createCommand = new Command("create", "Create a new document in a collection");
        var payloadOption = new Option<string>(
            name: "--payload",
            description: "JSON payload for the document");
        payloadOption.IsRequired = true;
        
        createCommand.AddOption(collectionOption);
        createCommand.AddOption(payloadOption);
        
        createCommand.SetHandler(async (string collection, string payload) =>
        {
            await CreateDocument(collection, payload);
        }, collectionOption, payloadOption);

        rootCommand.AddCommand(configCommand);
        rootCommand.AddCommand(serviceCommand);
        rootCommand.AddCommand(listCommand);
        rootCommand.AddCommand(queryCommand);
        rootCommand.AddCommand(createCommand);

        return await rootCommand.InvokeAsync(args);
    }

    static async Task SetConfigCommand(string appId, string? sharedKey, string? token, bool offline)
    {
        try
        {
            var config = ConfigurationManager.Load();
            
            // Update only provided values
            config.AppId = appId;
            if (sharedKey != null) config.SharedKey = sharedKey;
            
            if (token != null) config.Token = token;
            
            config.OfflineOnly = offline;
            
            ConfigurationManager.Save(config);
            
            Console.WriteLine("✅ Configuration saved successfully");
            Console.WriteLine($"📁 Config file: {ConfigurationManager.GetConfigPath()}");
            Console.WriteLine();
            ShowConfig(config);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to save configuration: {ex.Message}");
        }
    }

    static void ShowConfigCommand()
    {
        try
        {
            if (!ConfigurationManager.ConfigExists())
            {
                Console.WriteLine("📝 No configuration file found");
                Console.WriteLine($"💡 Use 'config set' to create configuration");
                Console.WriteLine($"📁 Config would be saved to: {ConfigurationManager.GetConfigPath()}");
                return;
            }

            var config = ConfigurationManager.Load();
            ShowConfig(config);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to load configuration: {ex.Message}");
        }
    }

    static void DeleteConfigCommand()
    {
        try
        {
            if (!ConfigurationManager.ConfigExists())
            {
                Console.WriteLine("📝 No configuration file to delete");
                return;
            }

            ConfigurationManager.Delete();
            Console.WriteLine("✅ Configuration file deleted");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to delete configuration: {ex.Message}");
        }
    }

    static void ShowConfig(ClientConfiguration config)
    {
        Console.WriteLine("📋 Current Configuration:");
        Console.WriteLine($"📁 File: {ConfigurationManager.GetConfigPath()}");
        Console.WriteLine();
        Console.WriteLine($"  App ID: {config.AppId}");
        Console.WriteLine($"  Shared Key: {(string.IsNullOrEmpty(config.SharedKey) ? "Not set" : "****** (hidden)")}");
        Console.WriteLine($"  Token: {(string.IsNullOrEmpty(config.Token) ? "Not set" : "****** (hidden)")}");
        Console.WriteLine($"  Offline Only: {config.OfflineOnly}");
        Console.WriteLine($"  Default Collection: {config.DefaultCollection}");
        Console.WriteLine($"  Default Limit: {config.DefaultLimit}");
        Console.WriteLine($"  Last Updated: {config.LastUpdated:yyyy-MM-dd HH:mm:ss} UTC");
        
        Console.WriteLine();
        Console.WriteLine("🔍 Observers Configuration:");
        Console.WriteLine($"  Enabled: {config.Observers.Enabled}");
        Console.WriteLine($"  Collections: [{string.Join(", ", config.Observers.Collections)}]");
        Console.WriteLine($"  Auto Sync: {config.Observers.AutoSync}");
        Console.WriteLine($"  Log Changes: {config.Observers.LogChanges}");
        Console.WriteLine($"  Verbose Logging: {config.Observers.VerboseLogging}");
    }

    static async Task ConfigureObserversCommand(bool? enable, string[]? collections, bool? logChanges, bool? verboseLogging)
    {
        try
        {
            var config = ConfigurationManager.Load();
            
            if (enable.HasValue)
            {
                config.Observers.Enabled = enable.Value;
                Console.WriteLine($"✅ Observers {(enable.Value ? "enabled" : "disabled")}");
            }
            
            if (collections != null && collections.Length > 0)
            {
                config.Observers.Collections = collections.ToList();
                Console.WriteLine($"✅ Observer collections set to: [{string.Join(", ", collections)}]");
            }
            
            if (logChanges.HasValue)
            {
                config.Observers.LogChanges = logChanges.Value;
                Console.WriteLine($"✅ Observer logging {(logChanges.Value ? "enabled" : "disabled")}");
            }
            
            if (verboseLogging.HasValue)
            {
                config.Observers.VerboseLogging = verboseLogging.Value;
                Console.WriteLine($"✅ Verbose logging {(verboseLogging.Value ? "enabled" : "disabled")}");
                if (verboseLogging.Value)
                {
                    Console.WriteLine("📝 Observers will show full document content");
                }
                else
                {
                    Console.WriteLine("📄 Observers will show summary information only");
                }
            }
            
            ConfigurationManager.Save(config);
            Console.WriteLine("💾 Observer configuration saved");
            
            Console.WriteLine();
            Console.WriteLine("🔍 Current Observer Configuration:");
            Console.WriteLine($"  Enabled: {config.Observers.Enabled}");
            Console.WriteLine($"  Collections: [{string.Join(", ", config.Observers.Collections)}]");
            Console.WriteLine($"  Auto Sync: {config.Observers.AutoSync}");
            Console.WriteLine($"  Log Changes: {config.Observers.LogChanges}");
            Console.WriteLine($"  Verbose Logging: {config.Observers.VerboseLogging}");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to configure observers: {ex.Message}");
        }
    }

    static async Task StartServiceCommand(string appId, string? sharedKey, string? token, bool offline)
    {
        Console.WriteLine("🚀 Starting Ditto background service...");
        
        if (DittoServiceClient.IsServiceRunning())
        {
            Console.WriteLine("⚠️  Service appears to already be running");
            Console.WriteLine("   Use 'service status' to check or 'service stop' to stop it first");
            return;
        }

        // Load config if no parameters provided
        var config = ConfigurationManager.Load();
        
        // Use config values as defaults if command-line options not provided
        var finalAppId = appId != "ditto-cot-client" ? appId : config.AppId;
        var finalSharedKey = sharedKey ?? config.SharedKey;
        
        var finalToken = token ?? config.Token;
        var finalOffline = offline || config.OfflineOnly;

        // Validate shared key + token combination for offline mode
        if (!string.IsNullOrEmpty(finalSharedKey) && finalOffline && string.IsNullOrEmpty(finalToken))
        {
            Console.WriteLine("❌ Shared key authentication in offline mode requires a token");
            Console.WriteLine("💡 Set both with: config set --shared-key YOUR_KEY --token YOUR_TOKEN --offline");
            return;
        }

        Console.WriteLine($"📱 Using App ID: {finalAppId}");
        if (!string.IsNullOrEmpty(finalSharedKey))
        {
            Console.WriteLine($"🔐 Using shared key authentication");
            Console.WriteLine($"🔑 SharedKey: {finalSharedKey}");
        }
        else if (!string.IsNullOrEmpty(finalToken))
        {
            Console.WriteLine($"🌐 Using token authentication");
        }
        else
        {
            Console.WriteLine("💾 Using offline-only mode");
        }
        
        if (!string.IsNullOrEmpty(finalToken))
        {
            Console.WriteLine($"🎫 Token: {finalToken}");
        }

        var success = await DittoServiceClient.StartServiceAsync(finalAppId, finalSharedKey, finalToken, finalOffline);
        if (success)
        {
            Console.WriteLine("💡 You can now use 'list' and 'query' commands");
        }
        else
        {
            Console.WriteLine("❌ Failed to start service");
        }
    }

    static void StopServiceCommand()
    {
        Console.WriteLine("🛑 Stopping Ditto background service...");
        DittoServiceClient.StopService();
    }

    static async Task StatusServiceCommand()
    {
        Console.WriteLine("📊 Checking service status...");
        
        using var client = new DittoServiceClient();
        var connected = await client.ConnectAsync(2000);
        
        if (connected)
        {
            var pingResponse = await client.PingAsync();
            if (pingResponse)
            {
                Console.WriteLine("✅ Service is running and responding");
                
                // Get detailed status
                var response = await client.SendRequestAsync("ping");
                if (response?.Success == true && response.Data != null)
                {
                    var statusData = JObject.FromObject(response.Data);
                    Console.WriteLine($"   Status: {statusData["status"]}");
                    Console.WriteLine($"   Ditto Status: {statusData["ditto_status"]}");
                    Console.WriteLine($"   Last Response: {DateTimeOffset.FromUnixTimeMilliseconds(response.Timestamp):yyyy-MM-dd HH:mm:ss} UTC");
                }
            }
            else
            {
                Console.WriteLine("⚠️  Service is running but not responding properly");
            }
        }
        else
        {
            Console.WriteLine("❌ Service is not running or not responding");
            Console.WriteLine("💡 Use 'service start' to start the service");
        }
    }

    static async Task ListDocuments(string collectionName, int limit, bool asCot = false)
    {
        Console.WriteLine($"🔍 Listing documents from collection: {collectionName}");
        Console.WriteLine($"📊 Limit: {limit}");
        if (asCot)
        {
            Console.WriteLine($"🔄 Output format: CoT XML");
        }
        Console.WriteLine();

        using var client = new DittoServiceClient();
        if (!await client.ConnectAsync())
        {
            Console.WriteLine("❌ Cannot connect to Ditto service");
            Console.WriteLine("💡 Start the service first: dotnet run -- service start");
            return;
        }

        try
        {
            var response = await client.SendRequestAsync("list", collectionName, null, limit);
            if (response?.Success == true)
            {
                await DisplayQueryResults(response, collectionName, "FindAll()", asCot);
            }
            else
            {
                Console.WriteLine($"❌ List failed: {response?.Error ?? "Unknown error"}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error: {ex.Message}");
        }
    }

    static async Task QueryDocuments(string collectionName, string query, int limit, bool asCot = false)
    {
        Console.WriteLine($"🔍 Querying collection: {collectionName}");
        Console.WriteLine($"🔎 Query: {query}");
        Console.WriteLine($"📊 Limit: {limit}");
        if (asCot)
        {
            Console.WriteLine($"🔄 Output format: CoT XML");
        }
        Console.WriteLine();

        using var client = new DittoServiceClient();
        if (!await client.ConnectAsync())
        {
            Console.WriteLine("❌ Cannot connect to Ditto service");
            Console.WriteLine("💡 Start the service first: dotnet run -- service start");
            return;
        }

        try
        {
            var response = await client.SendRequestAsync("query", collectionName, query, limit);
            if (response?.Success == true)
            {
                await DisplayQueryResults(response, collectionName, query, asCot);
            }
            else
            {
                Console.WriteLine($"❌ Query failed: {response?.Error ?? "Unknown error"}");
                if (response?.Error?.Contains("query") == true)
                {
                    Console.WriteLine("\nQuery syntax error. Make sure to:");
                    Console.WriteLine("  - Escape quotes in string values: \\\"value\\\"");
                    Console.WriteLine("  - Use proper field names (_id, w, e, etc.)");
                    Console.WriteLine("  - Use valid operators (==, !=, <, >, etc.)");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error: {ex.Message}");
        }
    }

    static async Task DisplayQueryResults(ServiceResponse response, string collection, string query, bool asCot = false)
    {
        if (response.Data == null)
        {
            Console.WriteLine("No data received");
            return;
        }

        var dataObj = JObject.FromObject(response.Data);
        var documents = dataObj["documents"]?.ToObject<JArray>() ?? new JArray();
        var count = dataObj["count"]?.ToObject<int>() ?? 0;

        Console.WriteLine($"📄 Found {count} document(s) in collection '{collection}'");
        Console.WriteLine($"🕐 Query executed at: {DateTimeOffset.FromUnixTimeMilliseconds(response.Timestamp):yyyy-MM-dd HH:mm:ss} UTC");
        Console.WriteLine();

        if (count == 0)
        {
            Console.WriteLine("No documents found matching the criteria.");
            Console.WriteLine("\nTip: Make sure:");
            Console.WriteLine("  - The collection name is correct");
            Console.WriteLine("  - Documents exist in the collection");
            Console.WriteLine("  - Your query syntax is valid");
            return;
        }

        int docIndex = 1;
        foreach (var doc in documents)
        {
            if (asCot)
            {
                await DisplayDocumentAsCot(doc, docIndex++);
            }
            else
            {
                DisplayDocument(doc, docIndex++);
            }
        }
    }

    static void DisplayDocument(JToken doc, int index)
    {
        Console.WriteLine(new string('=', 80));
        Console.WriteLine($"Document #{index}");
        Console.WriteLine(new string('=', 80));

        try
        {
            var docId = doc["id"]?.ToString() ?? "unknown";
            var docValue = doc["value"];

            if (docValue == null)
            {
                Console.WriteLine("⚠️  Document has no value");
                return;
            }

            // Display Ditto document
            var json = docValue.ToString(Formatting.Indented);
            Console.WriteLine($"\n📋 Ditto Document (JSON) - ID: {docId}");
            Console.WriteLine(json);

            // Try to convert to CoT
            Console.WriteLine("\n🎯 CoT Conversion:");
            try
            {
                var docDict = docValue.ToObject<Dictionary<string, object>>();
                if (docDict != null && docDict.ContainsKey("w"))
                {
                    var eventType = docDict["w"]?.ToString() ?? "";
                    
                    // Convert based on type
                    object dittoDoc = ConvertJsonToTypedDocument(json, eventType);
                    string cotXml = DocumentConverter.ConvertDocumentToXml(dittoDoc);
                    
                    Console.WriteLine("✅ Successfully converted to CoT XML:");
                    Console.WriteLine(FormatXml(cotXml));
                }
                else
                {
                    Console.WriteLine("⚠️  Cannot convert: Document missing 'w' (type) field");
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"⚠️  Conversion failed: {ex.Message}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error processing document: {ex.Message}");
        }

        Console.WriteLine();
    }

    static async Task DisplayDocumentAsCot(JToken doc, int index)
    {
        Console.WriteLine(new string('=', 80));
        Console.WriteLine($"CoT Document #{index}");
        Console.WriteLine(new string('=', 80));

        try
        {
            var docId = doc["id"]?.ToString() ?? "unknown";
            var docValue = doc["value"];

            if (docValue == null)
            {
                Console.WriteLine("⚠️  Document has no value");
                return;
            }

            Console.WriteLine($"📋 Ditto Document ID: {docId}");

            // Convert to CoT XML
            try
            {
                var json = docValue.ToString(Formatting.None);
                var docDict = docValue.ToObject<Dictionary<string, object>>();
                
                if (docDict != null && docDict.ContainsKey("w"))
                {
                    var eventType = docDict["w"]?.ToString() ?? "";
                    Console.WriteLine($"🏷️  Event Type: {eventType}");
                    
                    // Convert based on type
                    object dittoDoc = ConvertJsonToTypedDocument(json, eventType);
                    string cotXml = DocumentConverter.ConvertDocumentToXml(dittoDoc);
                    
                    Console.WriteLine("🎯 CoT XML:");
                    Console.WriteLine(FormatXml(cotXml));
                }
                else
                {
                    Console.WriteLine("⚠️  Cannot convert: Document missing 'w' (type) field");
                    Console.WriteLine("📄 Raw JSON:");
                    Console.WriteLine(docValue.ToString(Formatting.Indented));
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"⚠️  CoT conversion failed: {ex.Message}");
                Console.WriteLine("📄 Raw JSON:");
                Console.WriteLine(docValue.ToString(Formatting.Indented));
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error processing document: {ex.Message}");
        }

        Console.WriteLine();
    }

    static async Task CreateDocument(string collectionName, string payload)
    {
        Console.WriteLine($"📝 Creating document in collection: {collectionName}");
        Console.WriteLine($"📄 Payload: {payload}");
        Console.WriteLine();

        using var client = new DittoServiceClient();
        if (!await client.ConnectAsync())
        {
            Console.WriteLine("❌ Cannot connect to Ditto service");
            Console.WriteLine("💡 Start the service first: dotnet run -- service start");
            return;
        }

        try
        {
            // Validate JSON payload
            var testParse = JObject.Parse(payload);
            
            var response = await client.SendRequestAsync("create", collectionName, payload);
            if (response?.Success == true)
            {
                Console.WriteLine("✅ Document created successfully");
                
                if (response.Data != null)
                {
                    var dataObj = JObject.FromObject(response.Data);
                    var documentId = dataObj["document_id"]?.ToString();
                    var collection = dataObj["collection"]?.ToString();
                    
                    Console.WriteLine($"📋 Document ID: {documentId}");
                    Console.WriteLine($"📁 Collection: {collection}");
                    Console.WriteLine($"🕐 Created at: {DateTimeOffset.FromUnixTimeMilliseconds(response.Timestamp):yyyy-MM-dd HH:mm:ss} UTC");
                }
            }
            else
            {
                Console.WriteLine($"❌ Create failed: {response?.Error ?? "Unknown error"}");
            }
        }
        catch (JsonException ex)
        {
            Console.WriteLine($"❌ Invalid JSON payload: {ex.Message}");
            Console.WriteLine("💡 Make sure your payload is valid JSON format");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error: {ex.Message}");
        }
    }

    static object ConvertJsonToTypedDocument(string json, string eventType)
    {
        // Determine document type and deserialize
        if (eventType == "a-u-emergency-g")
        {
            return DocumentConverter.ConvertJsonToDocument<ApiDocument>(json);
        }
        else if (eventType.Contains("b-t-f") || eventType.Contains("chat"))
        {
            return DocumentConverter.ConvertJsonToDocument<ChatDocument>(json);
        }
        else if (eventType.Contains("file") || eventType.Contains("attachment") || eventType.Contains("b-f-t-a"))
        {
            return DocumentConverter.ConvertJsonToDocument<FileDocument>(json);
        }
        else if (eventType.Contains("a-u-r-loc-g") || eventType.Contains("a-f-G-U-C") || 
                 eventType.Contains("a-f-G-U") || eventType.Contains("a-f-G-U-I") || 
                 eventType.Contains("a-f-G-U-T") || eventType.Contains("a-u-S") || 
                 eventType.Contains("a-u-A") || eventType.Contains("a-u-G"))
        {
            return DocumentConverter.ConvertJsonToDocument<MapItemDocument>(json);
        }
        else
        {
            return DocumentConverter.ConvertJsonToDocument<GenericDocument>(json);
        }
    }

    static string FormatXml(string xml)
    {
        try
        {
            var doc = System.Xml.Linq.XDocument.Parse(xml);
            return doc.ToString();
        }
        catch
        {
            return xml;
        }
    }
}