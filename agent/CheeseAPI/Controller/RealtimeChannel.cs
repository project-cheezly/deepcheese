using Grpc.Core;

namespace CheeseAPI.Controller
{
    public class RealtimeChannel
    {
        public Dictionary<string, List<IServerStreamWriter<FutureLimitOrderBookResponse>>> LimitOrderBookStream { get; } = [];
        public Dictionary<string, List<IServerStreamWriter<FutureCurrentPriceResponse>>> CurrentPriceStream { get; } = [];

        public void AddService(string stockCode, IServerStreamWriter<FutureLimitOrderBookResponse> serverStreamWriter)
        {
            lock (LimitOrderBookStream)
            {
                if (LimitOrderBookStream.TryGetValue(
                    stockCode, 
                    out List<IServerStreamWriter<FutureLimitOrderBookResponse>>? value
                ))
                    value.Add(serverStreamWriter);
                else
                    LimitOrderBookStream[stockCode] = [serverStreamWriter];
            }
        }

        public void RemoveService(string stockCode, IServerStreamWriter<FutureLimitOrderBookResponse> serverStreamWriter)
        {
            lock (LimitOrderBookStream)
                LimitOrderBookStream[stockCode].Remove(serverStreamWriter);
        }

        public void AddService(string stockCode, IServerStreamWriter<FutureCurrentPriceResponse> serverStreamWriter)
        {
            lock (CurrentPriceStream)
            {
                if (CurrentPriceStream.TryGetValue(
                    stockCode, 
                    out List<IServerStreamWriter<FutureCurrentPriceResponse>>? value
                ))
                    value.Add(serverStreamWriter);
                else
                    CurrentPriceStream[stockCode] = [serverStreamWriter];
            }
        }

        public void RemoveService(string stockCode, IServerStreamWriter<FutureCurrentPriceResponse> serverStreamWriter)
        {
            lock (CurrentPriceStream)
                CurrentPriceStream[stockCode].Remove(serverStreamWriter);
        }
    }
}
