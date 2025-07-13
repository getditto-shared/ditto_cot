using System.CommandLine;

namespace DittoCoTClient.Service;

public class ServiceHost
{
    public static async Task<int> Main(string[] args)
    {
        var rootCommand = new RootCommand("Ditto CoT Background Service");

        var appIdOption = new Option<string>(
            name: "--app-id",
            description: "Ditto App ID",
            getDefaultValue: () => "ditto-cot-service");

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

        var portOption = new Option<int>(
            name: "--port",
            description: "Port for HTTP API (future use)",
            getDefaultValue: () => 8080);

        rootCommand.AddOption(appIdOption);
        rootCommand.AddOption(sharedKeyOption);
        rootCommand.AddOption(tokenOption);
        rootCommand.AddOption(offlineOption);
        rootCommand.AddOption(portOption);

        rootCommand.SetHandler(async (string appId, string? sharedKey, string? token, bool offline, int port) =>
        {
            await RunServiceAsync(appId, sharedKey, token, offline, port);
        }, appIdOption, sharedKeyOption, tokenOption, offlineOption, portOption);

        return await rootCommand.InvokeAsync(args);
    }

    private static async Task RunServiceAsync(string appId, string? sharedKey, string? token, bool offline, int port)
    {
        Console.WriteLine("================================================================================");
        Console.WriteLine("                        Ditto CoT Background Service");
        Console.WriteLine("================================================================================");
        Console.WriteLine($"📱 App ID: {appId}");
        Console.WriteLine($"🔐 Auth Mode: {GetAuthMode(sharedKey, token, offline)}");
        Console.WriteLine($"🔌 Mode: {(offline ? "Offline-only" : "Online/Offline")}");
        Console.WriteLine($"🌐 Port: {port} (future use)");
        Console.WriteLine();

        var service = new DittoBackgroundService();
        var cancellationTokenSource = new CancellationTokenSource();

        // Handle Ctrl+C gracefully
        Console.CancelKeyPress += (sender, e) =>
        {
            e.Cancel = true;
            Console.WriteLine("\n🛑 Shutdown requested...");
            cancellationTokenSource.Cancel();
        };

        try
        {
            // Start the service
            var serviceTask = service.StartAsync(appId, sharedKey, token, offline);

            Console.WriteLine("✅ Service is running. Press Ctrl+C to stop.");
            Console.WriteLine("📡 Clients can connect via named pipe: DittoCoTService");
            Console.WriteLine();

            // Wait for cancellation
            await Task.Delay(Timeout.Infinite, cancellationTokenSource.Token);
        }
        catch (OperationCanceledException)
        {
            // Expected when cancellation is requested
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Service error: {ex.Message}");
        }
        finally
        {
            service.Stop();
        }

        Console.WriteLine("👋 Service shutdown complete");
    }

    private static string GetAuthMode(string? sharedKey, string? token, bool offline)
    {
        if (offline) return "Offline Development";
        if (!string.IsNullOrEmpty(sharedKey) && !string.IsNullOrEmpty(token)) return "Shared Key + Token";
        if (!string.IsNullOrEmpty(token)) return "Token Authentication";
        return "Offline Development (default)";
    }
}