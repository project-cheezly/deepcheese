using AxshinhanINDI64Lib;
using Google.Protobuf;

namespace CheeseAPI.Controller.IndiBroker
{
    public class LookupIndiBroker : BaseIndiBroker
    {
        public LookupIndiBroker(ILogger<ControllerFactory> logger, SynchronizationContext ctx)
            : base(logger, ctx)
        {
            InitializeControl();
        }

        private void InitializeControl()
        {
            ctx.Post(_ =>
            {
                control = new AxshinhanINDI64();
                control.CreateControl();
                control.ReceiveData += new _DshinhanINDI64Events_ReceiveDataEventHandler(ReceiveData);
            }, null);
        }

        public async Task<short> SendRequest(RequestData request)
        {
            var tcs = new TaskCompletionSource<short>();
            ctx.Post(_ =>
            {
                bool? setQueryResult = control?.SetQueryName(request.code);

                for (short i = 0; i < request.data.Count; i++)
                    control?.SetSingleData(i, request.data[i]);

                var rqid = control?.RequestData();

                if (rqid is null)
                    tcs.TrySetException(new Exception("rqid is null"));
                else
                {
                    if (rqid.Value == 0)
                        tcs.TrySetException(new Exception("rqid is 0"));
                    else
                    {
                        bool result = taskCheck.TryAdd(rqid.Value, new TaskCompletionSource<IMessage>());
                        if (result)
                            tcs.TrySetResult(rqid.Value);
                        else
                            tcs.TrySetException(new Exception("rqid is already in use"));
                    }
                }
            }, null);

            return await tcs.Task;
        }

        public void ReceiveData(object sender, _DshinhanINDI64Events_ReceiveDataEvent e)
        {
            var queryCode = (string?)control?.GetQueryName();
            var nRowSize = control?.GetMultiRowCount();
            short rqid = e.rqid;

            IMessage response = (queryCode ?? "") switch
            {
                "AccountList" => Marshaller.LookupAccountList(control!, nRowSize ?? 0),
                "SABC952Q1" => Marshaller.LookupFutureOptionDeposit(control!),
                "SACA108U1" => Marshaller.TransferDeposit(control!),
                "SABA602Q1" => Marshaller.LookupAccountDepositInfo(control!),
                "TR_FNCHART" => Marshaller.LookupContinuousFutureCandle(control!, nRowSize ?? 0),
                _ => throw new NotImplementedException(),
            };

            if (!taskCheck.TryGetValue(rqid, out TaskCompletionSource<IMessage>? value))
                throw new KeyNotFoundException("rqid not found");
            else
                value.TrySetResult(response);
        }

        public async Task<T> ReceiveResponse<T>(short rqid) where T : IMessage
        {
            if (!taskCheck.TryGetValue(rqid, out TaskCompletionSource<IMessage>? value))
                throw new KeyNotFoundException("rqid not found");

            var response = await value.Task;
            taskCheck.TryRemove(rqid, out _);
            return (T)response;
        }
    }
}
