using AxshinhanINDI64Lib;
using Google.Protobuf;
using System.Collections.Concurrent;

namespace CheeseAPI.Controller.IndiBroker
{
    public class BaseIndiBroker(ILogger<ControllerFactory> _logger, SynchronizationContext _ctx)
    {
        protected readonly ILogger<ControllerFactory> logger = _logger;
        protected readonly SynchronizationContext ctx = _ctx;

        protected AxshinhanINDI64? control;
        protected readonly ConcurrentDictionary<short, TaskCompletionSource<IMessage>> taskCheck = new();
    }
}
