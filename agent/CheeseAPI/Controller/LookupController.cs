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
                "27031555538",
                "01",
                grpcRequest.Password,
                "1"
            });

            var requestResult = await indiBroker.SendRequest(request);
            return await indiBroker.ReceiveResponse<AccountDepositInfoResponse>(requestResult);
        }
    }
}
