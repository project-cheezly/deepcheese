using AxshinhanINDI64Lib;

namespace CheeseAPI.Controller
{
    public static class Marshaller
    {
        
        private static int GetIntFromControl(short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetSingleData(idx).ToString() ?? "");
        }

        private static int GetIntFromMultiControl(short row, short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetMultiData(row, idx).ToString() ?? "");
        }

        private static int GetDoubleFromControl(short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetSingleData(idx).ToString()?.Replace(".", string.Empty) ?? "");
        }

        private static int GetDoubleFromMultiControl(short row, short idx, AxshinhanINDI64 control)
        {
            return int.Parse(control.GetMultiData(row, idx).ToString()?.Replace(".", string.Empty) ?? "");
        }
        
        private static string GetStringFromControl(short idx, AxshinhanINDI64 control)
        {
            return control.GetSingleData(idx).ToString() ?? "";
        }

        private static string GetStringFromMultiControl(short row, short idx, AxshinhanINDI64 control)
        {
            return control.GetMultiData(row, idx).ToString() ?? "";
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
                Candle = new Candle
                {
                    Open = GetDoubleFromControl(12, control),
                    High = GetDoubleFromControl(13, control),
                    Low = GetDoubleFromControl(14, control),
                    Close = GetDoubleFromControl(4, control),
                }
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

        internal static ContinuousFutureCandleResponse LookupContinuousFutureCandle(AxshinhanINDI64 control, short v)
        {
            var response = new ContinuousFutureCandleResponse();

            for (short row = 0; row < v; row++)
            {
                var rawDate = GetIntFromMultiControl(row, 0, control);
                var rawTime = GetIntFromMultiControl(row, 1, control);

                var date = new Date
                {
                    Year = rawDate / 10000,
                    Month = (rawDate % 10000) / 100,
                    Day = rawDate % 100,
                };

                var time = new Time
                {
                    Minute = rawTime / 10000,
                    Second = (rawTime % 10000) / 100,
                    Millisecond = rawTime % 100,
                };

                var candle = new Candle
                {
                    Date = date,
                    Time = time,
                    Open = GetDoubleFromMultiControl(row, 2, control),
                    High = GetDoubleFromMultiControl(row, 3, control),
                    Low = GetDoubleFromMultiControl(row, 4, control),
                    Close = GetDoubleFromMultiControl(row, 5, control),
                    Volume = GetIntFromMultiControl(row, 9, control),
                };
            }

            return response;
        }

        public static TradeFutureOptionResponse TradeFutureOption(AxshinhanINDI64 control)
        {
            return new TradeFutureOptionResponse
            {
                OrderNumber = GetStringFromControl(0, control),
                OrcOrderNumber = GetStringFromControl(1, control),
            };
        }

        public static FutureOptionContractResponse LookupFutureOptionContract(AxshinhanINDI64 control, short v)
        {
            var result = new FutureOptionContractResponse();

            for (short row = 0; row < v; row++)
            {
                result.List.Add(new FutureOptionContract
                {
                    Code = GetStringFromMultiControl(row, 0, control),
                    TradeSep = GetStringFromMultiControl(row, 2, control) switch
                    {
                        "01" => TradeSep.Ask,
                        "02" => TradeSep.Bid,
                        "03" => TradeSep.Modify,
                        _ => TradeSep.Cancel,
                    },
                    Amount = GetIntFromMultiControl(row, 3, control),
                    ClosableAmount = GetIntFromMultiControl(row, 4, control),
                });
            }

            return result;
        }

        public static FuturesInfoResponse LookupFuturesInfo(AxshinhanINDI64 control, short v)
        {
            var result = new FuturesInfoResponse();

            for (short row = 0; row < v; row++)
            {
                var rawDate = GetIntFromMultiControl(row, 7, control);
                var date = new Date
                {
                    Year = rawDate / 10000,
                    Month = (rawDate % 10000) / 100,
                    Day = rawDate % 100,
                };

                var spreadLeadMonthStandardCode = GetStringFromMultiControl(row, 5, control);
                var spreadBackMonthStandardCode = GetStringFromMultiControl(row, 6, control);

                var target = new FuturesInfo
                {
                    StandardCode = GetStringFromMultiControl(row, 0, control),
                    AbbrCode = GetStringFromMultiControl(row, 1, control),
                    Name = GetStringFromMultiControl(row, 2, control),
                    AbbrName = GetStringFromMultiControl(row, 3, control),
                    SupplementCode = GetStringFromMultiControl(row, 4, control),
                    BaseAssetCode = GetStringFromMultiControl(row, 8, control),
                    
                    FinalTradeDate = date,
                    Multiplier = GetIntFromMultiControl(row, 9, control),
                };

                if (spreadLeadMonthStandardCode != "")
                {
                    target.SpreadLeadMonthStandardCode = spreadLeadMonthStandardCode;
                }

                if (spreadBackMonthStandardCode != "")
                {
                    target.SpreadBackMonthStandardCode = spreadBackMonthStandardCode;
                }

                result.List.Add(target);
            }

            return result;
        }

        public static FuturePreviousCandleResponse LookupFuturePreviousCandle(AxshinhanINDI64 control)
        {
            return new FuturePreviousCandleResponse()
            {
                Open = GetDoubleFromMultiControl(0, 0, control),
                High = GetDoubleFromMultiControl(0, 1, control),
                Low = GetDoubleFromMultiControl(0, 2, control),
                Close = GetDoubleFromMultiControl(0, 3, control),
            };
        }
    }
}
