using DittoCoTClient.Service;

namespace DittoCoTService;

class ServiceProgram
{
    static async Task<int> Main(string[] args)
    {
        return await ServiceHost.Main(args);
    }
}