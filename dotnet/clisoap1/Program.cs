// dotnet/clisoap1/Program.cs
// Uso: dotnet run  -> http://localhost:5001/?n=10
// Consume el servicio SOAP público NumberConversion

using System.ServiceModel;

var builder = WebApplication.CreateBuilder(args);
var app     = builder.Build();

app.MapGet("/", async (int n = 10) =>
{
    // Cambiar BasicHttpBinding por BasicHttpsBinding
    var binding  = new BasicHttpsBinding();
    var endpoint = new EndpointAddress(
        "https://www.dataaccess.com/webservicesserver/NumberConversion.wso");

    var factory  = new ChannelFactory<INumberConversion>(binding, endpoint);
    var cliente  = factory.CreateChannel();

    var resultado = await cliente.NumberToWordsAsync(n);
    return Results.Text(resultado);
});

app.Run();

// ---- Contrato del servicio (proxy simplificado) ----
[ServiceContract(Namespace = "http://www.dataaccess.com/webservicesserver/")]
public interface INumberConversion
{
    [OperationContract(Action = "NumberToWords")]
    Task<string> NumberToWordsAsync(long ubiNum);
}