using CheeseAPI.Controller.IndiBroker;
using Grpc.Core;

namespace CheeseAPI.Controller
{
    public class RealtimeController(ILogger<ControllerFactory> _logger, SynchronizationContext _ctx)
    {
        private readonly ILogger<ControllerFactory> logger = _logger;
        private readonly RealtimeIndiBroker indiBroker = new(_logger, _ctx);

        public async void AddService(string trCode, string stockCode, IServerStreamWriter<FutureLimitOrderBookResponse> writer)
        {
            bool? result = await indiBroker.RegisterRealtimeEvent(trCode, stockCode);

            if (result.HasValue)
                indiBroker.channel.AddService(stockCode, writer);
        }

        public async void RemoveService(string trCode, string stockCode, IServerStreamWriter<FutureLimitOrderBookResponse> writer)
        {
            bool? result = await indiBroker.UnregisterRealtimeEvent(trCode, stockCode);
            
            if (result.HasValue)
                indiBroker.channel.RemoveService(stockCode, writer);
        }

        public async void AddService(string trCode, string stockCode, IServerStreamWriter<FutureCurrentPriceResponse> writer)
        {
            bool? result = await indiBroker.RegisterRealtimeEvent(trCode, stockCode);

            if (result.HasValue)
                indiBroker.channel.AddService(stockCode, writer);
        }

        public async void RemoveService(string trCode, string stockCode, IServerStreamWriter<FutureCurrentPriceResponse> writer)
        {
            bool? result = await indiBroker.UnregisterRealtimeEvent(trCode, stockCode);
            
            if (result.HasValue)
                indiBroker.channel.RemoveService(stockCode, writer);
        }
    }
}
