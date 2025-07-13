using Newtonsoft.Json;

namespace DittoCoTClient.Config;

public class ClientConfiguration
{
    [JsonProperty("app_id")]
    public string AppId { get; set; } = "ditto-cot-client";

    [JsonProperty("shared_key")]
    public string? SharedKey { get; set; }

    [JsonProperty("token")]
    public string? Token { get; set; }

    [JsonProperty("offline_only")]
    public bool OfflineOnly { get; set; } = false;

    [JsonProperty("default_collection")]
    public string DefaultCollection { get; set; } = "cot_events";

    [JsonProperty("default_limit")]
    public int DefaultLimit { get; set; } = 10;

    [JsonProperty("last_updated")]
    public DateTimeOffset LastUpdated { get; set; } = DateTimeOffset.UtcNow;

    [JsonProperty("observers")]
    public ObserverConfiguration Observers { get; set; } = new ObserverConfiguration();

}

public class ObserverConfiguration
{
    [JsonProperty("enabled")]
    public bool Enabled { get; set; } = true;

    [JsonProperty("collections")]
    public List<string> Collections { get; set; } = new List<string> { "mapItem", "mapItems", "track" };

    [JsonProperty("auto_sync")]
    public bool AutoSync { get; set; } = true;

    [JsonProperty("log_changes")]
    public bool LogChanges { get; set; } = false;

    [JsonProperty("verbose_logging")]
    public bool VerboseLogging { get; set; } = false;
}

public class ConfigurationManager
{
    private static readonly string ConfigDirectory = Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.UserProfile), 
        ".ditto-cot-client");
    
    private static readonly string ConfigFilePath = Path.Combine(ConfigDirectory, "config.json");

    public static ClientConfiguration Load()
    {
        try
        {
            if (!File.Exists(ConfigFilePath))
            {
                return new ClientConfiguration();
            }

            var json = File.ReadAllText(ConfigFilePath);
            return JsonConvert.DeserializeObject<ClientConfiguration>(json) ?? new ClientConfiguration();
        }
        catch (Exception ex)
        {
            Console.WriteLine($"⚠️  Warning: Failed to load configuration: {ex.Message}");
            return new ClientConfiguration();
        }
    }

    public static void Save(ClientConfiguration config)
    {
        try
        {
            // Ensure config directory exists
            Directory.CreateDirectory(ConfigDirectory);

            config.LastUpdated = DateTimeOffset.UtcNow;
            var json = JsonConvert.SerializeObject(config, Formatting.Indented);
            File.WriteAllText(ConfigFilePath, json);
        }
        catch (Exception ex)
        {
            throw new InvalidOperationException($"Failed to save configuration: {ex.Message}", ex);
        }
    }

    public static string GetConfigPath()
    {
        return ConfigFilePath;
    }

    public static bool ConfigExists()
    {
        return File.Exists(ConfigFilePath);
    }

    public static void Delete()
    {
        try
        {
            if (File.Exists(ConfigFilePath))
            {
                File.Delete(ConfigFilePath);
            }
        }
        catch (Exception ex)
        {
            throw new InvalidOperationException($"Failed to delete configuration: {ex.Message}", ex);
        }
    }
}