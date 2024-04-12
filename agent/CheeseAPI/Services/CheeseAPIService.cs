using CheeseAPI;
using CheeseAPI.Controller;
using CheeseAPI.Controller.IndiBroker;
using Grpc.Core;
using System.Configuration;

namespace CheeseAPI.Services
{
    public class CheeseAPIService : CheeseAPI.CheeseAPIBase
    {
        private readonly ILogger<CheeseAPIService> logger;
        private readonly LookupController lookupController;
        private readonly RealtimeController realtimeController;
        private readonly IndiHealthChecker indiHealthChecker;

        public CheeseAPIService(ILogger<CheeseAPIService> _logger, ControllerFactory factory)
        {
            logger = _logger;
            lookupController = factory.CreateLookupController();
            realtimeController = factory.CreateRealtimeController();
            indiHealthChecker = factory.CreateIndiHealthChecker();

            indiHealthChecker.StartIndi();
        }

/*
 *  1. 시세 TR 리스트
 */

/*
 *  1.2 선물 시세 TR 리스트
 */

        async public override Task<ContinuousFutureCandleResponse> LookupContinuousFutureCandle(
            ContinuousFutureCandleRequest request,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: LookupContinuousFutureCandle");
            return await lookupController.LookupContinuousFutureCandle(request);
        }

        async public override Task<AccountListResponse> LookupAccountList(Empty request, ServerCallContext context)
        {
            logger.LogInformation("REQ: LookupAccountList");
            return await lookupController.LookupAccountList();
        }

        async public override Task<FutureOptionDepositResponse> LookupFutureOptionDeposit(
            FutureOptionDepositRequest request,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: LookupFutureOptionDeposit");
            return await lookupController.LookupFutureOptionDeposit(request);
        }

        async public override Task<TransferDepositResponse> TransferDeposit(
            TransferDepositRequest request,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: TransferDeposit");
            return await lookupController.TransferDeposit(request);
        }

        public override async Task<AccountDepositInfoResponse> LookupAccountDepositInfo(
            AccountDepositInfoRequest request,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: LookupAccountDepositInfo");
            return await lookupController.LookupAccountDepositInfo(request);
        }

        async public override Task LookupFutureLimitOrderBookRealtime(
            FutureLimitOrderBookRequest request,
            IServerStreamWriter<FutureLimitOrderBookResponse> responseStream,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: LookupFutureLimitOrderBookRealtime");
            realtimeController.AddService("FH", request.Code, responseStream);

            while (!context.CancellationToken.IsCancellationRequested)
                await Task.Delay(1000);

            realtimeController.RemoveService("FH", request.Code, responseStream);
            logger.LogInformation("RELEASE: LookupFutureLimitOrderBookRealtime");
        }

        async public override Task LookupFutureCurrentPriceRealtime(
            FutureCurrentPriceRequest request,
            IServerStreamWriter<FutureCurrentPriceResponse> responseStream,
            ServerCallContext context
        ) {
            logger.LogInformation("REQ: LookupFutureCurrentPriceRealtime");
            realtimeController.AddService("FC", request.Code, responseStream);

            while (!context.CancellationToken.IsCancellationRequested)
                await Task.Delay(1000);

            realtimeController.RemoveService("FC", request.Code, responseStream);
            logger.LogInformation("RELEASE: LookupFutureCurrentPriceRealtime");
        }
    }
}
