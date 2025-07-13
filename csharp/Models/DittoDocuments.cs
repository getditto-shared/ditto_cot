using Newtonsoft.Json;

namespace Ditto.Cot.Models;

public abstract class CommonDocument
{
    [JsonProperty("_id")]
    public string Id { get; set; } = string.Empty;

    [JsonProperty("_c")]
    public int Counter { get; set; } = 0;

    [JsonProperty("_v")]
    public int Version { get; set; } = 2;

    [JsonProperty("_r")]
    public bool Removed { get; set; } = false;

    [JsonProperty("a")]
    public string PeerKey { get; set; } = string.Empty;

    [JsonProperty("b")]
    public double TimeMillisFromEpoch { get; set; }

    [JsonProperty("d")]
    public string AuthorUid { get; set; } = string.Empty;

    [JsonProperty("e")]
    public string Callsign { get; set; } = string.Empty;

    [JsonProperty("g")]
    public string VersionString { get; set; } = string.Empty;

    [JsonProperty("h")]
    public double? CircularError { get; set; }

    [JsonProperty("i")]
    public double? HeightAboveEllipsoid { get; set; }

    [JsonProperty("j")]
    public double? Latitude { get; set; }

    [JsonProperty("k")]
    public double? LinearError { get; set; }

    [JsonProperty("l")]
    public double? Longitude { get; set; }

    // For time field - not part of main schema but used in conversion
    [JsonIgnore]
    public double TimeMillis { get; set; }

    [JsonProperty("n")]
    public long StartTime { get; set; }

    [JsonProperty("o")]
    public long StaleTime { get; set; }

    [JsonProperty("p")]
    public string How { get; set; } = string.Empty;

    [JsonProperty("q")]
    public string Access { get; set; } = string.Empty;

    [JsonProperty("r")]
    public Dictionary<string, object> Detail { get; set; } = new();

    // Helper method to set detail as JSON string for CRDT compatibility
    public void SetDetailAsJson(Dictionary<string, object> detailMap)
    {
        Detail = new Dictionary<string, object> 
        { 
            ["_json"] = JsonConvert.SerializeObject(detailMap) 
        };
    }

    // Helper method to get detail from JSON string
    public Dictionary<string, object> GetDetailFromJson()
    {
        if (Detail.TryGetValue("_json", out var jsonValue) && jsonValue is string json)
        {
            try
            {
                return JsonConvert.DeserializeObject<Dictionary<string, object>>(json) ?? new Dictionary<string, object>();
            }
            catch
            {
                return new Dictionary<string, object>();
            }
        }
        return Detail; // Fallback to direct dictionary access for legacy compatibility
    }

    // Helper method to check if a key exists in the detail (works with both direct and JSON storage)
    public bool DetailContainsKey(string key)
    {
        if (Detail.ContainsKey(key))
            return true;
            
        var detailMap = GetDetailFromJson();
        return detailMap.ContainsKey(key);
    }

    // Helper method to get a value from detail (works with both direct and JSON storage)
    public object? GetDetailValue(string key)
    {
        if (Detail.TryGetValue(key, out var directValue))
            return directValue;
            
        var detailMap = GetDetailFromJson();
        return detailMap.TryGetValue(key, out var jsonValue) ? jsonValue : null;
    }

    [JsonProperty("s")]
    public string Opex { get; set; } = string.Empty;

    [JsonProperty("t")]
    public string Qos { get; set; } = string.Empty;

    [JsonProperty("u")]
    public string Caveat { get; set; } = string.Empty;

    [JsonProperty("v")]
    public string ReleasableTo { get; set; } = string.Empty;

    [JsonProperty("w")]
    public string EventType { get; set; } = string.Empty;

    [JsonProperty("source")]
    public string? Source { get; set; }

    // Store original timestamp strings for precision preservation (base64 encoded to prevent parsing)
    [JsonProperty("time_b64")]
    public string? TimeOriginal { get; set; }
    
    [JsonProperty("start_b64")]
    public string? StartOriginal { get; set; }
    
    [JsonProperty("stale_b64")]
    public string? StaleOriginal { get; set; }
    
    // Store original point values for precision preservation (base64 encoded to prevent parsing)
    [JsonProperty("ce_b64")]
    public string? CeOriginal { get; set; }
    
    [JsonProperty("le_b64")]
    public string? LeOriginal { get; set; }
    
    [JsonProperty("lat_b64")]
    public string? LatOriginal { get; set; }
    
    [JsonProperty("lon_b64")]
    public string? LonOriginal { get; set; }
    
    [JsonProperty("hae_b64")]
    public string? HaeOriginal { get; set; }

    // Helper methods for base64 encoding/decoding to preserve exact string values
    public void SetTimeOriginal(string? value) => TimeOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetTimeOriginal() => TimeOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(TimeOriginal)) : null;
    
    public void SetStartOriginal(string? value) => StartOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetStartOriginal() => StartOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(StartOriginal)) : null;
    
    public void SetStaleOriginal(string? value) => StaleOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetStaleOriginal() => StaleOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(StaleOriginal)) : null;
    
    public void SetCeOriginal(string? value) => CeOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetCeOriginal() => CeOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(CeOriginal)) : null;
    
    public void SetLeOriginal(string? value) => LeOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetLeOriginal() => LeOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(LeOriginal)) : null;
    
    public void SetLatOriginal(string? value) => LatOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetLatOriginal() => LatOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(LatOriginal)) : null;
    
    public void SetLonOriginal(string? value) => LonOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetLonOriginal() => LonOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(LonOriginal)) : null;
    
    public void SetHaeOriginal(string? value) => HaeOriginal = value != null ? Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(value)) : null;
    public string? GetHaeOriginal() => HaeOriginal != null ? System.Text.Encoding.UTF8.GetString(Convert.FromBase64String(HaeOriginal)) : null;
}

public class ApiDocument : CommonDocument
{
    [JsonProperty("isFile")]
    public bool? IsFile { get; set; }

    [JsonProperty("title")]
    public string? Title { get; set; }

    [JsonProperty("mime")]
    public string? Mime { get; set; }

    [JsonProperty("contentType")]
    public string? ContentType { get; set; }

    [JsonProperty("tag")]
    public string? Tag { get; set; }

    [JsonProperty("data")]
    public string? Data { get; set; }

    [JsonProperty("isRemoved")]
    public bool? IsRemoved { get; set; }

    [JsonProperty("timeMillis")]
    public long? TimeMillisField { get; set; }
}

public class ChatDocument : CommonDocument
{
    [JsonProperty("authorCallsign")]
    public string? AuthorCallsign { get; set; }

    [JsonProperty("authorType")]
    public string? AuthorType { get; set; }

    [JsonProperty("authorUid")]
    public string? AuthorUidField { get; set; }

    [JsonProperty("location")]
    public string? Location { get; set; }

    [JsonProperty("message")]
    public string? Message { get; set; }

    [JsonProperty("parent")]
    public string? Parent { get; set; }

    [JsonProperty("room")]
    public string? Room { get; set; }

    [JsonProperty("roomId")]
    public string? RoomId { get; set; }

    [JsonProperty("time")]
    public string? Time { get; set; }
}

public class FileDocument : CommonDocument
{
    [JsonProperty("c")]
    public string? FileName { get; set; }

    [JsonProperty("contentType")]
    public string? ContentType { get; set; }

    [JsonProperty("file")]
    public string? File { get; set; }

    [JsonProperty("itemId")]
    public string? ItemId { get; set; }

    [JsonProperty("mime")]
    public string? Mime { get; set; }

    [JsonProperty("sz")]
    public double? Size { get; set; }
}

public class GenericDocument : CommonDocument
{
    // Generic document only has the common fields
}

public class MapItemDocument : CommonDocument
{
    [JsonProperty("c")]
    public string? Name { get; set; }

    [JsonProperty("f")]
    public bool? Visible { get; set; }
}

public abstract class CotDocument
{
    public static CotDocument FromJson(string json)
    {
        var obj = JsonConvert.DeserializeObject<Dictionary<string, object>>(json);
        
        if (obj == null || !obj.TryGetValue("w", out var typeValue) || typeValue == null)
        {
            throw new ArgumentException("Document is missing 'w' field or is invalid JSON");
        }

        var eventType = typeValue.ToString() ?? string.Empty;

        // Determine document type based on event type (following Rust implementation pattern)
        if (eventType == "a-u-emergency-g")
        {
            var doc = JsonConvert.DeserializeObject<ApiDocument>(json) ?? throw new ArgumentException("Failed to deserialize as ApiDocument");
            return new ApiDocumentWrapper(doc);
        }
        else if (eventType.Contains("b-t-f") || eventType.Contains("chat"))
        {
            var doc = JsonConvert.DeserializeObject<ChatDocument>(json) ?? throw new ArgumentException("Failed to deserialize as ChatDocument");
            return new ChatDocumentWrapper(doc);
        }
        else if (eventType.Contains("file") || eventType.Contains("attachment") || eventType.Contains("b-f-t-a"))
        {
            var doc = JsonConvert.DeserializeObject<FileDocument>(json) ?? throw new ArgumentException("Failed to deserialize as FileDocument");
            return new FileDocumentWrapper(doc);
        }
        else if (eventType.Contains("a-u-r-loc-g") || eventType.Contains("a-f-G-U-C") || 
                 eventType.Contains("a-f-G-U") || eventType.Contains("a-f-G-U-I") || 
                 eventType.Contains("a-f-G-U-T") || eventType.Contains("a-u-S") || 
                 eventType.Contains("a-u-A") || eventType.Contains("a-u-G"))
        {
            var doc = JsonConvert.DeserializeObject<MapItemDocument>(json) ?? throw new ArgumentException("Failed to deserialize as MapItemDocument");
            return new MapItemDocumentWrapper(doc);
        }
        else
        {
            var doc = JsonConvert.DeserializeObject<GenericDocument>(json) ?? throw new ArgumentException("Failed to deserialize as GenericDocument");
            return new GenericDocumentWrapper(doc);
        }
    }

    public abstract CoTEvent ToCoTEvent();
}

public class ApiDocumentWrapper : CotDocument
{
    public ApiDocument Document { get; }

    public ApiDocumentWrapper(ApiDocument document)
    {
        Document = document;
    }

    public override CoTEvent ToCoTEvent()
    {
        return DocumentConverter.ConvertApiDocumentToCoTEvent(Document);
    }
}

public class ChatDocumentWrapper : CotDocument
{
    public ChatDocument Document { get; }

    public ChatDocumentWrapper(ChatDocument document)
    {
        Document = document;
    }

    public override CoTEvent ToCoTEvent()
    {
        return DocumentConverter.ConvertChatDocumentToCoTEvent(Document);
    }
}

public class FileDocumentWrapper : CotDocument
{
    public FileDocument Document { get; }

    public FileDocumentWrapper(FileDocument document)
    {
        Document = document;
    }

    public override CoTEvent ToCoTEvent()
    {
        return DocumentConverter.ConvertFileDocumentToCoTEvent(Document);
    }
}

public class GenericDocumentWrapper : CotDocument
{
    public GenericDocument Document { get; }

    public GenericDocumentWrapper(GenericDocument document)
    {
        Document = document;
    }

    public override CoTEvent ToCoTEvent()
    {
        return DocumentConverter.ConvertGenericDocumentToCoTEvent(Document);
    }
}

public class MapItemDocumentWrapper : CotDocument
{
    public MapItemDocument Document { get; }

    public MapItemDocumentWrapper(MapItemDocument document)
    {
        Document = document;
    }

    public override CoTEvent ToCoTEvent()
    {
        return DocumentConverter.ConvertMapItemDocumentToCoTEvent(Document);
    }
}