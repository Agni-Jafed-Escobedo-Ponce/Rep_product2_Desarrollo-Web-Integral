// dotnet/conintl/Program.cs
// Uso: dotnet run  -> http://localhost:5003/?n=10
// Convierte un número a letras en español usando Humanizer

using Humanizer;
using System.Globalization;

var builder = WebApplication.CreateBuilder(args);
var app     = builder.Build();

app.MapGet("/", (long n = 10) =>
{
    var cultura = new CultureInfo("es");
    var texto = n.ToWords(cultura);
    return Results.Text(texto);
});

app.Run();