
using AxshinhanINDI64Lib;
using Grpc.Core;
using System.DirectoryServices.ActiveDirectory;

namespace CheeseAPI.Controller.IndiBroker
{
    public class RealtimeIndiBroker : BaseIndiBroker
    {
        public readonly RealtimeChannel channel;

        public RealtimeIndiBroker(ILogger<ControllerFactory> logger, SynchronizationContext ctx)
            : base(logger, ctx)
        {
            channel = new();
            InitializeControl();
        }

        protected void InitializeControl()
        {
            ctx.Post(_ =>
            {
                control = new AxshinhanINDI64();
                control.CreateControl();
                control.ReceiveRTData += new _DshinhanINDI64Events_ReceiveRTDataEventHandler(ReceiveRTData);
            }, null);
        }

        public void ReceiveRTData(object sender, _DshinhanINDI64Events_ReceiveRTDataEvent e)
        {
            var queryCode = (string?)control?.GetQueryName();
            var stockCode = (string?)control?.GetSingleData(1);
            logger.LogDebug("RECV: Realtime {queryCode}, {stockCode}", queryCode, stockCode);

            switch (queryCode!)
            {
                case "FH":
                    var orderBookData = Marshaller.LookupFutureLimitOrderBook(control!);
                    foreach (var writer in channel.LimitOrderBookStream[stockCode!])
                        writer?.WriteAsync(orderBookData);
                    break;
                case "FC":
                    var currentPriceData = Marshaller.LookupFutureCurrentPrice(control!);
                    foreach (var writer in channel.CurrentPriceStream[stockCode!])
                        writer?.WriteAsync(currentPriceData);
                    break;
                default:
                    break;
            }
        }

        public async Task<bool?> RegisterRealtimeEvent(string trCode, string stockCode)
        {
            var result = new TaskCompletionSource<bool?>();
            ctx.Post(_ =>
            {
                result.SetResult(control?.RequestRTReg(trCode, stockCode));
            }, null);

            return await result.Task;
        }

        public async Task<bool?> UnregisterRealtimeEvent(string trCode, string stockCode)
        {
            var result = new TaskCompletionSource<bool?>();
            ctx.Post(_ =>
            {
                result.SetResult(control?.UnRequestRTReg(trCode, stockCode));
            }, null);

            return await result.Task;
        }
    }
}
