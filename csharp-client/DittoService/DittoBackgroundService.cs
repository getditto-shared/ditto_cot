using System.IO.Pipes;
using System.Text;
using DittoSDK;
using Newtonsoft.Json;
using DittoCoTClient.Config;

namespace DittoCoTClient.Service;

public class DittoBackgroundService
{
    private DittoSDK.Ditto? _ditto;
    private NamedPipeServerStream? _pipeServer;
    private bool _isRunning = false;
    private readonly string _pipeName = "DittoCoTService";
    private readonly CancellationTokenSource _cancellationTokenSource = new();
    private readonly List<DittoStoreObserver> _observers = new();

    public async Task StartAsync(string appId, string? sharedKey, string? token, bool offlineOnly = false)
    {
        Console.WriteLine("🚀 Starting Ditto Background Service...");

        try
        {
            // Clean up any existing lock files first
            await CleanupLockFilesAsync();

            // Initialize Ditto
            _ditto = InitializeDitto(appId, sharedKey, token, offlineOnly);

            // Transport config will be set directly on the Ditto instance in InitializeDitto

            if (!offlineOnly)
            {
                _ditto.StartSync();
                Console.WriteLine("✅ Ditto sync started");
            }

            Console.WriteLine($"💾 Ditto initialized (App ID: {appId})");

            // Setup observers based on configuration
            await SetupObserversAsync();

            // Start IPC server
            _isRunning = true;
            await StartIpcServerAsync();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to start service: {ex.Message}");
            _isRunning = false;
            throw;
        }
    }

    public void Stop()
    {
        Console.WriteLine("🛑 Stopping Ditto Background Service...");

        _isRunning = false;
        _cancellationTokenSource.Cancel();

        _pipeServer?.Close();
        _pipeServer?.Dispose();

        // Clean up observers (they will be cleaned up when Ditto is disposed)
        _observers.Clear();

        _ditto?.StopSync();
        _ditto?.Dispose();

        Console.WriteLine("✅ Service stopped");
    }

    private async Task CleanupLockFilesAsync()
    {
        try
        {
            var dittoDir = Path.Combine(Directory.GetCurrentDirectory(), "ditto");
            var lockFile = Path.Combine(dittoDir, "__ditto_lock_file");

            if (File.Exists(lockFile))
            {
                try
                {
                    File.Delete(lockFile);
                    Console.WriteLine("🧹 Cleaned up existing lock file");
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"⚠️  Could not delete lock file: {ex.Message}");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"⚠️  Error during cleanup: {ex.Message}");
        }
    }

    private DittoSDK.Ditto InitializeDitto(string appId, string? sharedKey, string? token, bool offlineOnly)
    {
        DittoSDK.Ditto ditto;

        // DEBUG: Log the incoming parameters
        Console.WriteLine($"🔍 DEBUG - InitializeDitto called with:");
        Console.WriteLine($"   App ID: '{appId}'");
        Console.WriteLine($"   Shared Key: '{sharedKey ?? "null/empty"}'");
        Console.WriteLine($"   Token: '{token ?? "null/empty"}'");
        Console.WriteLine($"   Offline Only: {offlineOnly}");

        // Create a temporary directory like in the example (to rule out persistence directory issues)
        var tempDir = Path.Combine(
            Path.GetTempPath(),
            "DittoCOTService-" + Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);
        var serviceWorkingDir = tempDir;
        
        Console.WriteLine($"🔍 DEBUG - Using temp persistence directory: {serviceWorkingDir}");

        if (!string.IsNullOrEmpty(sharedKey))
        {
            // SharedKey authentication for production use
            Console.WriteLine("🔍 DEBUG - Attempting shared key authentication");

            try
            {
                Console.WriteLine($"🔍 DEBUG - Creating SharedKey identity with explicit siteId");
                // Create SharedKey identity with explicit siteId (siteId=0 default might be causing issues)
                var siteId = (uint)Math.Abs(Environment.MachineName.GetHashCode());
                Console.WriteLine($"🔍 DEBUG - Using siteId: {siteId}");
                var identity = DittoIdentity.SharedKey(
                    appId: appId,
                    sharedKey: sharedKey,
                    siteId: siteId
                );
                Console.WriteLine($"🔍 DEBUG - SharedKey identity created successfully");

                Console.WriteLine($"🔍 DEBUG - Creating Ditto instance with persistence dir: {serviceWorkingDir}");
try {
                    ditto = new DittoSDK.Ditto(identity, serviceWorkingDir);
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"❌ Failed to create Ditto instance: {ex.Message}");
                    throw;
                }                Console.WriteLine($"🔍 DEBUG - Ditto instance created successfully");

                // Always set offline license token for SharedKey authentication (as per example)
                if (!string.IsNullOrEmpty(token))
                {
                    Console.WriteLine($"🔍 DEBUG - Setting offline license token");
                    ditto.SetOfflineOnlyLicenseToken(token);
                    Console.WriteLine($"✅ Offline license token set");
                }
                else
                {
                    Console.WriteLine($"⚠️  No token provided for SharedKey authentication");
                }

                // Configure transport using TransportConfig properties (as per example)
                ditto.TransportConfig.PeerToPeer.BluetoothLE.Enabled = true;
                ditto.TransportConfig.PeerToPeer.Lan.Enabled = true;
                // Note: Awdl might not be available in this SDK version, using what we have
                Console.WriteLine($"🔍 DEBUG - Transport configuration set");

                // Disable sync with v3 peers for DQL compatibility
                ditto.DisableSyncWithV3();
                Console.WriteLine($"🔍 DEBUG - Disabled sync with v3 peers");

                Console.WriteLine($"✅ Using SharedKey authentication with offline license token");
            }
            catch (Exception sharedKeyEx)
            {
                Console.WriteLine($"❌ SharedKey authentication failed: {sharedKeyEx.Message}");
                throw;
            }
        }
        else if (!string.IsNullOrEmpty(token) && !offlineOnly)
        {
            // Online playground mode (for development/testing)
            var identity = DittoIdentity.OnlinePlayground(
                appId: appId,
                token: token,
                enableDittoCloudSync: true
            );
            ditto = new DittoSDK.Ditto(identity, serviceWorkingDir);
            Console.WriteLine($"🌐 Using online playground mode");
        }
        else if (!string.IsNullOrEmpty(token) && offlineOnly)
        {
            // Manual identity with offline token
            var identity = DittoIdentity.Manual(appId);
            ditto = new DittoSDK.Ditto(identity, serviceWorkingDir);
            ditto.SetOfflineOnlyLicenseToken(token);
            Console.WriteLine($"💾 Using Manual identity with offline-only token");
        }
        else
        {
            // Offline development mode
            var identity = DittoIdentity.OfflinePlayground(appId: appId);
            ditto = new DittoSDK.Ditto(identity, serviceWorkingDir);
            Console.WriteLine($"💾 Using offline development mode");
        }

        return ditto;
    }

    private async Task SetupObserversAsync()
    {
        try
        {
            if (_ditto == null)
            {
                Console.WriteLine("⚠️  Cannot setup observers: Ditto not initialized");
                return;
            }

            // Load configuration to check observer settings
            var config = ConfigurationManager.Load();
            
            if (!config.Observers.Enabled)
            {
                Console.WriteLine("📋 Observers disabled in configuration");
                return;
            }

            Console.WriteLine($"🔄 Setting up subscriptions and observers for {config.Observers.Collections.Count} collections...");

            foreach (var collection in config.Observers.Collections)
            {
                await SetupCollectionSubscriptionAndObserver(collection, config.Observers);
            }

            Console.WriteLine($"✅ {_observers.Count} observers and subscriptions registered successfully");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to setup observers: {ex.Message}");
        }
    }

    private async Task SetupCollectionSubscriptionAndObserver(string collectionName, ObserverConfiguration observerConfig)
    {
        try
        {
            if (_ditto == null) return;

            var query = $"SELECT * FROM {collectionName}";
            
            // First, register subscription to ensure Ditto syncs this collection
            Console.WriteLine($"🔄 Registering subscription for collection: {collectionName}");
            _ditto.Sync.RegisterSubscription(query);
            Console.WriteLine($"✅ Subscription registered for {collectionName}");

            // Then, register observer to watch for changes
            Console.WriteLine($"👁️  Registering observer for collection: {collectionName}");
            var observer = _ditto.Store.RegisterObserver(
                query,
                (result) =>
                {
                    HandleObserverChange(collectionName, result, observerConfig);
                });

            _observers.Add(observer);
            Console.WriteLine($"✅ Observer registered for {collectionName}");

            // Execute an initial query to populate any existing data
            try 
            {
                Console.WriteLine($"📊 Executing initial query for {collectionName}...");
                var initialResult = await _ditto.Store.ExecuteAsync(query);
                Console.WriteLine($"📋 Initial query for {collectionName}: {initialResult.Items.Count} existing documents");
                
                // Process initial data through observer handler
                if (observerConfig.LogChanges && initialResult.Items.Count > 0)
                {
                    Console.WriteLine($"📤 Processing initial data for {collectionName}...");
                    HandleObserverChange(collectionName, initialResult, observerConfig);
                }

                // Optional: EVICT to clean up old/stale data (uncomment if needed)
                // await _ditto.Store.ExecuteAsync($"EVICT FROM {collectionName}");
                // Console.WriteLine($"🧹 EVICT executed for {collectionName}");
            }
            catch (Exception queryEx)
            {
                Console.WriteLine($"⚠️  Initial query failed for {collectionName}: {queryEx.Message}");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to setup subscription/observer for {collectionName}: {ex.Message}");
        }
    }

    private void HandleObserverChange(string collectionName, DittoQueryResult result, ObserverConfiguration config)
    {
        try
        {
            if (config.LogChanges)
            {
                Console.WriteLine($"📡 OBSERVER EVENT: {collectionName} collection changed - {result.Items.Count} documents");
                Console.WriteLine($"⏰ Timestamp: {DateTimeOffset.UtcNow:yyyy-MM-dd HH:mm:ss.fff} UTC");
                
                if (config.VerboseLogging)
                {
                    // Log every single document in detail (existing verbose behavior)
                    for (int i = 0; i < result.Items.Count; i++)
                    {
                        var item = result.Items[i];
                        Console.WriteLine($"");
                        Console.WriteLine($"==================== DOCUMENT #{i + 1}/{result.Items.Count} ====================");
                        Console.WriteLine($"📋 Collection: {collectionName}");
                        
                        try
                        {
                            // Serialize the entire document to JSON for detailed logging
                            var documentJson = JsonConvert.SerializeObject(item.Value, Formatting.Indented);
                            Console.WriteLine($"📄 Full Document Content:");
                            Console.WriteLine(documentJson);
                            
                            // Also show key summary
                            var keys = item.Value.Keys.ToList();
                            Console.WriteLine($"🔑 Document Keys ({keys.Count}): [{string.Join(", ", keys)}]");
                            
                            // Show specific important fields if they exist
                            if (item.Value.ContainsKey("uid"))
                            {
                                Console.WriteLine($"🆔 UID: {item.Value["uid"]}");
                            }
                            if (item.Value.ContainsKey("callsign"))
                            {
                                Console.WriteLine($"📞 Callsign: {item.Value["callsign"]}");
                            }
                            if (item.Value.ContainsKey("type"))
                            {
                                Console.WriteLine($"🏷️  Type: {item.Value["type"]}");
                            }
                            if (item.Value.ContainsKey("how"))
                            {
                                Console.WriteLine($"📍 How: {item.Value["how"]}");
                            }
                            if (item.Value.ContainsKey("point"))
                            {
                                var point = item.Value["point"];
                                Console.WriteLine($"🌍 Location: {point}");
                            }
                        }
                        catch (Exception docEx)
                        {
                            Console.WriteLine($"❌ Error logging document #{i + 1}: {docEx.Message}");
                            Console.WriteLine($"📄 Raw document keys: [{string.Join(", ", item.Value.Keys)}]");
                        }
                        
                        Console.WriteLine($"========================================================");
                    }
                    
                    if (result.Items.Count == 0)
                    {
                        Console.WriteLine("📭 No documents in this observer event (collection may be empty)");
                    }
                    
                    Console.WriteLine($"✅ Observer event processing complete for {collectionName}");
                }
                else
                {
                    // Simple summary announcement (default behavior)
                    if (result.Items.Count > 0)
                    {
                        Console.WriteLine($"📄 Summary: {result.Items.Count} document(s) updated in {collectionName}");
                        
                        // Show quick overview of document types if available
                        var documentTypes = new HashSet<string>();
                        foreach (var item in result.Items)
                        {
                            if (item.Value.ContainsKey("w"))
                            {
                                var eventType = item.Value["w"]?.ToString();
                                if (!string.IsNullOrEmpty(eventType))
                                {
                                    documentTypes.Add(eventType);
                                }
                            }
                        }
                        
                        if (documentTypes.Count > 0)
                        {
                            Console.WriteLine($"🏷️  Event types: [{string.Join(", ", documentTypes)}]");
                        }
                    }
                    else
                    {
                        Console.WriteLine("📭 Observer event received with no documents");
                    }
                    
                    Console.WriteLine($"✅ Observer update processed for {collectionName}");
                }
                Console.WriteLine("");
            }

            // Here you could add additional logic like:
            // - Trigger sync operations
            // - Send notifications  
            // - Update caches
            // - Process specific document types
            // - Convert to CoT format
            // - Forward to other systems
            
            if (config.AutoSync)
            {
                // Auto-sync is already handled by Ditto StartSync()
                // This could trigger additional sync operations if needed
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error handling observer change for {collectionName}: {ex.Message}");
            Console.WriteLine($"🔍 Exception details: {ex}");
        }
    }

    private async Task StartIpcServerAsync()
    {
        Console.WriteLine($"🔌 Starting IPC server on pipe: {_pipeName}");

        while (_isRunning && !_cancellationTokenSource.Token.IsCancellationRequested)
        {
            try
            {
                _pipeServer = new NamedPipeServerStream(
                    _pipeName,
                    PipeDirection.InOut,
                    1);

                Console.WriteLine("⏳ Waiting for client connections...");
                await _pipeServer.WaitForConnectionAsync(_cancellationTokenSource.Token);
                Console.WriteLine("✅ Client connected");

                await HandleClientAsync(_pipeServer);
            }
            catch (OperationCanceledException)
            {
                // Expected when cancellation is requested
                break;
            }
            catch (Exception ex)
            {
                Console.WriteLine($"⚠️  IPC error: {ex.Message}");
                await Task.Delay(1000, _cancellationTokenSource.Token);
            }
            finally
            {
                _pipeServer?.Dispose();
                _pipeServer = null;
            }
        }
    }

    private async Task HandleClientAsync(NamedPipeServerStream pipe)
    {
        try
        {
            using var reader = new StreamReader(pipe, Encoding.UTF8);
            using var writer = new StreamWriter(pipe, Encoding.UTF8) { AutoFlush = true };

            while (pipe.IsConnected && _isRunning)
            {
                var requestJson = await reader.ReadLineAsync();
                if (string.IsNullOrEmpty(requestJson))
                {
                    break;
                }

                Console.WriteLine($"📨 Received request: {requestJson}");

                var response = await ProcessRequestAsync(requestJson);
                await writer.WriteLineAsync(response);

                Console.WriteLine($"📤 Sent response");
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"⚠️  Client handling error: {ex.Message}");
        }
    }

    private async Task<string> ProcessRequestAsync(string requestJson)
    {
        try
        {
            var request = JsonConvert.DeserializeObject<ServiceRequest>(requestJson);
            if (request == null)
            {
                return CreateErrorResponse("Invalid request format");
            }

            return request.Action switch
            {
                "ping" => CreateSuccessResponse(new { status = "alive", ditto_status = _ditto != null ? "initialized" : "not_initialized" }),
                "query" => await HandleQueryRequestAsync(request),
                "list" => await HandleListRequestAsync(request),
                "create" => await HandleCreateRequestAsync(request),
                _ => CreateErrorResponse($"Unknown action: {request.Action}")
            };
        }
        catch (Exception ex)
        {
            return CreateErrorResponse($"Request processing error: {ex.Message}");
        }
    }

    private Task<string> HandleQueryRequestAsync(ServiceRequest request)
    {
        try
        {
            if (_ditto == null)
            {
                return Task.FromResult(CreateErrorResponse("Ditto not initialized"));
            }

            var collection = _ditto.Store.Collection(request.Collection ?? "cot_events");
            var query = request.Query ?? "true";
            var limit = request.Limit ?? 10;

            var documents = collection.Find(query).Limit(limit).Exec();

            var results = documents.Select(doc => new
            {
                id = doc.Id,
                value = doc.Value
            }).ToList();

            return Task.FromResult(CreateSuccessResponse(new
            {
                documents = results,
                count = results.Count,
                collection = request.Collection,
                query = query
            }));
        }
        catch (Exception ex)
        {
            return Task.FromResult(CreateErrorResponse($"Query failed: {ex.Message}"));
        }
    }

    private Task<string> HandleListRequestAsync(ServiceRequest request)
    {
        try
        {
            if (_ditto == null)
            {
                return Task.FromResult(CreateErrorResponse("Ditto not initialized"));
            }

            var collection = _ditto.Store.Collection(request.Collection ?? "cot_events");
            var limit = request.Limit ?? 10;

            var documents = collection.FindAll().Limit(limit).Exec();

            var results = documents.Select(doc => new
            {
                id = doc.Id,
                value = doc.Value
            }).ToList();

            return Task.FromResult(CreateSuccessResponse(new
            {
                documents = results,
                count = results.Count,
                collection = request.Collection
            }));
        }
        catch (Exception ex)
        {
            return Task.FromResult(CreateErrorResponse($"List failed: {ex.Message}"));
        }
    }

    private async Task<string> HandleCreateRequestAsync(ServiceRequest request)
    {
        try
        {
            if (_ditto == null)
            {
                return CreateErrorResponse("Ditto not initialized");
            }

            if (string.IsNullOrEmpty(request.Collection))
            {
                return CreateErrorResponse("Collection name is required");
            }

            if (string.IsNullOrEmpty(request.Payload))
            {
                return CreateErrorResponse("Payload is required");
            }

            // Parse and validate JSON payload
            Dictionary<string, object> documentData;
            try
            {
                documentData = JsonConvert.DeserializeObject<Dictionary<string, object>>(request.Payload);
                if (documentData == null)
                {
                    return CreateErrorResponse("Invalid JSON payload - could not parse");
                }
            }
            catch (JsonException ex)
            {
                return CreateErrorResponse($"Invalid JSON payload: {ex.Message}");
            }

            // Insert document into collection
            var collection = _ditto.Store.Collection(request.Collection);
            var insertedDocumentId = collection.Upsert(documentData);

            Console.WriteLine($"✅ Document inserted into {request.Collection} with ID: {insertedDocumentId}");

            return CreateSuccessResponse(new
            {
                document_id = insertedDocumentId.ToString(),
                collection = request.Collection,
                status = "created"
            });
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Create operation failed: {ex.Message}");
            return CreateErrorResponse($"Create failed: {ex.Message}");
        }
    }

    private string CreateSuccessResponse(object data)
    {
        return JsonConvert.SerializeObject(new
        {
            success = true,
            timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            data = data
        });
    }

    private string CreateErrorResponse(string error)
    {
        return JsonConvert.SerializeObject(new
        {
            success = false,
            timestamp = DateTimeOffset.UtcNow.ToUnixTimeMilliseconds(),
            error = error
        });
    }
}

public class ServiceRequest
{
    public string Action { get; set; } = string.Empty;
    public string? Collection { get; set; }
    public string? Query { get; set; }
    public string? Payload { get; set; }
    public int? Limit { get; set; }
}

public class ServiceResponse
{
    public bool Success { get; set; }
    public long Timestamp { get; set; }
    public object? Data { get; set; }
    public string? Error { get; set; }
}