using AxshinhanINDI64Lib;
using CheeseAPI.Controller.IndiBroker;
using Google.Protobuf;
using System.Collections.Concurrent;

namespace CheeseAPI.Controller
{
    public class LookupController
    {
        private readonly ILogger<ControllerFactory> logger;

        private readonly LookupIndiBroker indiBroker;

        public LookupController(ILogger<ControllerFactory> _logger, SynchronizationContext _ctx)
        {
            logger = _logger;
            indiBroker = new(logger, _ctx);
        }

/*
 *  1. 시세 TR 리스트
 */

/*
 *  1.2 선물 시세 TR 리스트
 */

        async public Task<ContinuousFutureCandleResponse>LookupContinuousFutureCandle(ContinuousFutureCandleRequest grpcRequest)
        {
            var graphType = grpcRequest.GraphType switch
            {
                GraphType.Day => "D",
                GraphType.Minute => "M",
                _ => throw new Exception("Invalid GraphType")
            };

            var startDate = string.Format("{0:yyyyMMdd}", grpcRequest.StartDate);
            var endDate = string.Format("{0:yyyyMMdd}", grpcRequest.EndDate);

            var request = new RequestData("TR_FNCHART", new List<string>
            {
                grpcRequest.Code,
                graphType,
                grpcRequest.Interval.ToString(),
                startDate,
                endDate,
                grpcRequest.Count.ToString(),
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<ContinuousFutureCandleResponse>(requestResult);
        }

        async public Task<AccountListResponse> LookupAccountList()
        {
            var request = new RequestData("AccountList", new List<string>());
            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<AccountListResponse>(requestResult);
        }

        async public Task<FutureOptionDepositResponse> LookupFutureOptionDeposit(FutureOptionDepositRequest grpceRequest)
        {
            var request = new RequestData("SABC952Q1", new List<string>
            {
                grpceRequest.AccountNumber,
                grpceRequest.Password,
                "2"
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<FutureOptionDepositResponse>(requestResult);
        }

        async public Task<TransferDepositResponse> TransferDeposit(TransferDepositRequest grpcRequest)
        {
            var request = new RequestData("SACA108U1", new List<string>
            {
                grpcRequest.AccountNumber,
                grpcRequest.WithdrawalItemNumber,
                grpcRequest.Password,
                grpcRequest.DepositItemNumber,
                grpcRequest.TransferAmount.ToString(),
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<TransferDepositResponse>(requestResult);
        }

        async public Task<AccountDepositInfoResponse> LookupAccountDepositInfo(AccountDepositInfoRequest grpcRequest)
        {
            var request = new RequestData("SABA602Q1", new List<string>
            {
                grpcRequest.AccountNumber,
                "01",
                grpcRequest.Password,
                "1"
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<AccountDepositInfoResponse>(requestResult);
        }

        async public Task<TradeFutureOptionResponse> TradeFutureOption(TradeFutureOptionRequest grpcRequest)
        {
            var request = new RequestData("SABC100U1", new List<string>
            {
                grpcRequest.AccountNumber,
                grpcRequest.Password,
                grpcRequest.StockCode,
                grpcRequest.TransactionAmount.ToString(),
                grpcRequest.Price.ToString().Insert(-2, "."), // 1000 -> 10.00
                grpcRequest.TradeCondition switch
                {
                    TradeCondition.Normal => "0",
                    TradeCondition.Ioc => "3",
                    TradeCondition.Fok => "4", 
                    _ => throw new NotImplementedException() 
                },
                grpcRequest.TradeClassification switch
                {
                    TradeSep.Ask | TradeSep.Cancel | TradeSep.Modify => "1",
                    TradeSep.Bid => "2",
                    _ => throw new NotImplementedException()
                },
                grpcRequest.OrderClassification switch
                {
                    OrderClassification.Limit => "L",
                    OrderClassification.Market => "M",
                    OrderClassification.Conditional => "C",
                    OrderClassification.Best => "B",
                    _ => throw new NotImplementedException()
                },
                grpcRequest.Arbitrage.ToString(),
                grpcRequest.TradeClassification switch
                {
                    TradeSep.Ask | TradeSep.Bid => "1",
                    TradeSep.Modify => "2",
                    TradeSep.Cancel => "3",
                    _ => throw new NotImplementedException()
                },
                grpcRequest.ModifyAmount.ToString(),
                grpcRequest.OriginalOrderNumber,
                grpcRequest.ReservationOrder
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<TradeFutureOptionResponse>(requestResult);
        }

        async public Task<FutureOptionContractResponse> LookupFutureOptionContract(FutureOptionContractRequest grpcRequest)
        {
            var request = new RequestData("SABC967Q1", new List<string>
            {
                grpcRequest.AccountNumber,
                grpcRequest.Password,
                "0",
                "0",
                "1"
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<FutureOptionContractResponse>(requestResult);
        }
    }
}
