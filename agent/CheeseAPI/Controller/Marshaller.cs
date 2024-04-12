using AxshinhanINDI64Lib;

namespace CheeseAPI.Controller
{
    public static class Marshaller
    {
        
        private static int GetIntFromControl(short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetSingleData(idx).ToString() ?? "");
        }

        private static int GetDoubleFromControl(short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetSingleData(idx).ToString()?.Replace(".", string.Empty) ?? "");
        }

        private static string GetStringFromControl(short idx, AxshinhanINDI64 control)
        {
            return control.GetSingleData(idx).ToString() ?? "";
        }

        public static AccountListResponse LookupAccountList(AxshinhanINDI64 control, short nRowSize)
        {
            var data = new AccountListResponse();
            for (short i = 0; i < nRowSize; i++)
                data.List.Add((string)control.GetMultiData(i, 0));
            return data;
        }

        public static FutureLimitOrderBookResponse LookupFutureLimitOrderBook(AxshinhanINDI64 control)
        {
            var data = new FutureLimitOrderBookResponse()
            {
                Time = GetIntFromControl(2, control),
                TotalBidAmount = GetIntFromControl(33, control),
                TotalAskAmount = GetIntFromControl(34, control),
                TotalBidCount = GetIntFromControl(35, control),
                TotalAskCount = GetIntFromControl(36, control),
                MarketStatusCode = GetIntFromControl(37, control),
                ExpectedPrice = GetDoubleFromControl(38, control),
            };

            for (short i = 1; i <= 5; i++)
            {
                var bid = new LimitOrder()
                {
                    OrderLevel = i,
                    Price = GetDoubleFromControl((short)(6 * i - 3), control),
                    Amount = GetIntFromControl((short)(6 * i - 1), control),
                    Count = GetIntFromControl((short)(6 * i + 1), control),
                };

                var ask = new LimitOrder()
                {
                    OrderLevel = i,
                    Price = GetDoubleFromControl((short)(6 * i - 2), control),
                    Amount = GetIntFromControl((short)(6 * i), control),
                    Count = GetIntFromControl((short)(6 * i + 2), control),
                };

                data.Ask.Add(ask);
                data.Bid.Add(bid);
            }

            return data;
        }

        public static FutureCurrentPriceResponse LookupFutureCurrentPrice(AxshinhanINDI64 control)
        {
            return new FutureCurrentPriceResponse {
                Code = GetStringFromControl(1, control),
                Time = GetIntFromControl(2, control),
                Price = GetDoubleFromControl(4, control),
                Amount = GetIntFromControl(10, control),
                AskPrice = GetDoubleFromControl(18, control),
                BidPrice = GetDoubleFromControl(19, control),
                OpenInterest = GetIntFromControl(11, control),
            };
        }

        public static FutureOptionDepositResponse LookupFutureOptionDeposit(AxshinhanINDI64 control)
        {
            return new FutureOptionDepositResponse
            {
                TotalDeposit = GetIntFromControl(0, control),
                TradableDeposit = GetIntFromControl(4, control),
                WithdrawalAmount = GetIntFromControl(6, control),
            };
        }

        public static TransferDepositResponse TransferDeposit(AxshinhanINDI64 control)
        {
            return new TransferDepositResponse
            {
                Date = GetIntFromControl(0, control),
                WithdrawalNumber = GetStringFromControl(1, control),
                DepositNumber = GetStringFromControl(2, control),
                WithdrawalItemNumber = GetStringFromControl(3, control),
                DepositItemNumber = GetStringFromControl(4, control),
                TransferAmount = GetIntFromControl(5, control),
            };
        }

        public static AccountDepositInfoResponse LookupAccountDepositInfo(AxshinhanINDI64 control)
        {
            return new AccountDepositInfoResponse
            {
                TotalDeposit = GetIntFromControl(0, control),
                TradableDeposit = GetIntFromControl(19, control),
                WithdrawalAmount = GetIntFromControl(27, control),
            };
        }
    }
}
