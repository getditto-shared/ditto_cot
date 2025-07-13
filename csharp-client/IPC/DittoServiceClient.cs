using System.Diagnostics;
using System.IO.Pipes;
using System.Text;
using Newtonsoft.Json;

namespace DittoCoTClient.IPC;

public class DittoServiceClient : IDisposable
{
    private readonly string _pipeName = "DittoCoTService";
    private NamedPipeClientStream? _pipeClient;
    private StreamReader? _reader;
    private StreamWriter? _writer;
    private bool _isConnected = false;

    public async Task<bool> ConnectAsync(int timeoutMs = 5000)
    {
        try
        {
            _pipeClient = new NamedPipeClientStream(".", _pipeName, PipeDirection.InOut);
            
            using var cts = new CancellationTokenSource(timeoutMs);
            await _pipeClient.ConnectAsync(cts.Token);
            
            _reader = new StreamReader(_pipeClient, Encoding.UTF8);
            _writer = new StreamWriter(_pipeClient, Encoding.UTF8) { AutoFlush = true };
            
            _isConnected = true;
            return true;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to connect to service: {ex.Message}");
            Dispose();
            return false;
        }
    }

    public async Task<ServiceResponse?> SendRequestAsync(string action, string? collection = null, string? query = null, int? limit = null)
    {
        if (!_isConnected || _writer == null || _reader == null)
        {
            throw new InvalidOperationException("Not connected to service");
        }

        try
        {
            var request = new ServiceRequest
            {
                Action = action,
                Collection = collection,
                Query = action == "create" ? null : query,
                Payload = action == "create" ? query : null,
                Limit = limit
            };

            var requestJson = JsonConvert.SerializeObject(request);
            await _writer.WriteLineAsync(requestJson);

            var responseJson = await _reader.ReadLineAsync();
            if (string.IsNullOrEmpty(responseJson))
            {
                throw new InvalidOperationException("Empty response from service");
            }

            return JsonConvert.DeserializeObject<ServiceResponse>(responseJson);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Request failed: {ex.Message}");
            return null;
        }
    }


    public async Task<bool> PingAsync()
    {
        try
        {
            var response = await SendRequestAsync("ping");
            return response?.Success == true;
        }
        catch (ObjectDisposedException)
        {
            // Pipe was disposed during operation
            return false;
        }
        catch (InvalidOperationException)
        {
            // Pipe operation failed
            return false;
        }
    }

    public static bool IsServiceRunning()
    {
        try
        {
            // Check if any process is listening on our named pipe
            var processes = Process.GetProcessesByName("DittoCoTClient");
            return processes.Any(p => p.ProcessName.Contains("Service") || 
                                     p.MainWindowTitle.Contains("Service"));
        }
        catch
        {
            return false;
        }
    }

    public static async Task<bool> StartServiceAsync(string appId, string? sharedKey = null, string? token = null, bool offline = false)
    {
        try
        {
            var serviceExe = GetServiceExecutablePath();
            if (!File.Exists(serviceExe))
            {
                Console.WriteLine($"❌ Service executable not found: {serviceExe}");
                return false;
            }

            var args = $"--app-id \"{appId}\"";
            if (!string.IsNullOrEmpty(sharedKey))
                args += $" --shared-key \"{sharedKey}\"";
            if (!string.IsNullOrEmpty(token))
                args += $" --token \"{token}\"";
            if (offline)
                args += " --offline";

            Console.WriteLine($"🚀 Starting Ditto service: {serviceExe} {args}");

            var startInfo = new ProcessStartInfo
            {
                FileName = serviceExe,
                Arguments = args,
                UseShellExecute = false,
                CreateNoWindow = false,
                RedirectStandardOutput = false,
                RedirectStandardError = false
            };

            var process = Process.Start(startInfo);
            if (process == null)
            {
                Console.WriteLine("❌ Failed to start service process");
                return false;
            }

            // Wait a moment for the service to start
            await Task.Delay(3000);

            // Verify it's running by trying to connect and ping
            var client = new DittoServiceClient();
            var connected = await client.ConnectAsync(5000);
            
            if (connected)
            {
                try
                {
                    var pingResult = await client.PingAsync();
                    
                    // Give a moment for the response to be processed
                    await Task.Delay(100);
                    client.Dispose();
                    
                    if (pingResult)
                    {
                        Console.WriteLine("✅ Service started successfully");
                        return true;
                    }
                    else
                    {
                        Console.WriteLine("⚠️  Service connected but ping failed");
                        return false;
                    }
                }
                catch (Exception ex) when (ex is ObjectDisposedException || ex is InvalidOperationException)
                {
                    // These exceptions are expected during disposal and don't indicate failure
                    client.Dispose();
                    Console.WriteLine("✅ Service started successfully");
                    return true;
                }
                catch (Exception ex)
                {
                    client.Dispose();
                    Console.WriteLine($"⚠️  Service ping failed: {ex.Message}");
                    return false;
                }
            }
            else
            {
                client.Dispose();
                Console.WriteLine("⚠️  Service may have started but is not responding");
                return false;
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Failed to start service: {ex.Message}");
            return false;
        }
    }

    public static void StopService()
    {
        try
        {
            var processes = Process.GetProcessesByName("DittoCoTService")
                .Concat(Process.GetProcesses().Where(p => 
                    p.ProcessName.Contains("DittoCoT") && 
                    p.MainWindowTitle.Contains("Service")));

            foreach (var process in processes)
            {
                try
                {
                    Console.WriteLine($"🛑 Stopping service process: {process.Id}");
                    process.Kill();
                    process.WaitForExit(5000);
                    Console.WriteLine("✅ Service stopped");
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"⚠️  Failed to stop process {process.Id}: {ex.Message}");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error stopping service: {ex.Message}");
        }
    }

    private static string GetServiceExecutablePath()
    {
        // Try to find the service executable
        var currentDir = Directory.GetCurrentDirectory();
        
        // Try in bin directory first (most likely location after build)
        var serviceExe = Path.Combine(currentDir, "bin", "Debug", "net9.0", "DittoCoTService");
        if (File.Exists(serviceExe))
            return serviceExe;

        // Try with .exe extension in bin directory
        serviceExe = Path.Combine(currentDir, "bin", "Debug", "net9.0", "DittoCoTService.exe");
        if (File.Exists(serviceExe))
            return serviceExe;

        // Try in current directory
        serviceExe = Path.Combine(currentDir, "DittoCoTService");
        if (File.Exists(serviceExe))
            return serviceExe;

        // Try with .exe extension in current directory
        serviceExe = Path.Combine(currentDir, "DittoCoTService.exe");
        if (File.Exists(serviceExe))
            return serviceExe;

        // Default fallback
        return "DittoCoTService";
    }

    public void Dispose()
    {
        try
        {
            _isConnected = false;
            _reader?.Dispose();
            _writer?.Dispose();
            _pipeClient?.Dispose();
        }
        catch (ObjectDisposedException)
        {
            // Expected when pipe is already closed
        }
        catch (InvalidOperationException)
        {
            // Expected when pipe operations are in progress
        }
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