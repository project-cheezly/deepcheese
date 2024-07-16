using CheeseAPI.Controller;
using CheeseAPI.Services;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddSingleton<ControllerFactory>();
builder.Services.AddGrpc();

var app = builder.Build();
var _factory = app.Services.GetServices<ControllerFactory>();

// Configure the HTTP request pipeline.
app.MapGrpcService<CheeseAPIService>();
app.MapGet("/", () => "gRPC server");

app.Run();
