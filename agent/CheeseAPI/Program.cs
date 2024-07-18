using CheeseAPI.Controller;
using CheeseAPI.Services;
using OpenTelemetry.Exporter;
using OpenTelemetry.Logs;
using OpenTelemetry.Resources;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddOpenTelemetry()
    .ConfigureResource(r =>
    {
        r.AddService(builder.Environment.ApplicationName);
    })
    .WithLogging(logging => { logging.AddOtlpExporter(); });

builder.Services.Configure<OtlpExporterOptions>(options =>
{
    options.Endpoint = new Uri(builder.Configuration.GetValue<string>("Log:Host") ?? "http://localhost:4173");
});

// Add services to the container.
builder.Services.AddSingleton<ControllerFactory>();
builder.Services.AddGrpc();

var app = builder.Build();
var _factory = app.Services.GetServices<ControllerFactory>();

// Configure the HTTP request pipeline.
app.MapGrpcService<CheeseAPIService>();
app.MapGet("/", () => "Communication with gRPC endpoints must be made through a gRPC client.");

app.Run();
