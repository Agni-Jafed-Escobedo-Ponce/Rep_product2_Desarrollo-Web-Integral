// dotnet/clisoap2/Program.cs
// Uso: dotnet run  -> http://localhost:5002/?n=10
// Consume el servicio SOAP y traduce al español con GTranslate

using System.ServiceModel;
using GTranslate.Translators;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddHttpClient();
var app = builder.Build();

app.MapGet("/", async (int n = 10) =>
{
    // 1. Llamada SOAP con BasicHttpsBinding
    var binding  = new BasicHttpsBinding();
    var endpoint = new EndpointAddress(
        "https://www.dataaccess.com/webservicesserver/NumberConversion.wso");
    var factory  = new ChannelFactory<INumberConversion>(binding, endpoint);
    var cliente  = factory.CreateChannel();
    var en       = await cliente.NumberToWordsAsync(n);

    // 2. Traducción inglés -> español
    var traductor = new GoogleTranslator();
    var resultado = await traductor.TranslateAsync(en, "es");

    return Results.Text(resultado.Translation);
});

app.Run();

[ServiceContract(Namespace = "http://www.dataaccess.com/webservicesserver/")]
public interface INumberConversion
{
    [OperationContract(Action = "NumberToWords")]
    Task<string> NumberToWordsAsync(long ubiNum);
}