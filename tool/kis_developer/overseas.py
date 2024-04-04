import datetime
import httpx
from .config import uri, tr_id


class KISOverseas:
    def __init__(self, client: httpx.Client, header: dict[str, str]):
        self.__client = client
        self.__base_header = header

    def inquire_price(self, market_code: str, stock_code: str):
        header = {"tr_id": tr_id["overseas-stock"]["inquire-price"]}
        header.update(self.__base_header)

        params = {
            "AUTH": "",
            "EXCD": market_code,
            "SYMB": stock_code
        }

        endpoint = uri["origin"]["production"] + uri["overseas-stock"]["inquire-price"]
        return self.__client.get(endpoint, headers=header, params=params).json()

    def inquire_daily_forex(
            self,
            forex_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_forex(forex_code, end_date, 'X', 'D')

    def inquire_weekly_forex(
            self,
            forex_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_forex(forex_code, end_date, 'X', 'W')

    def inquire_monthly_forex(
            self,
            forex_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_forex(forex_code, end_date, 'X', 'M')

    def __inquire_periodic_forex(
            self,
            code: str,
            end_date: datetime.date,
            market_div_code: str,
            period_div_code: str
    ):
        header = {"tr_id": tr_id["overseas-stock"]["inquire-periodic-forex"]}
        header.update(self.__base_header)

        params = {
            "FID_COND_MRKT_DIV_CODE": market_div_code,
            "FID_INPUT_ISCD": code,
            "FID_INPUT_DATE_1": "20000101",
            "FID_INPUT_DATE_2": end_date.strftime("%Y%m%d"),
            "FID_PERIOD_DIV_CODE": period_div_code
        }

        endpoint = uri["origin"]["production"] + uri["overseas-stock"]["inquire-periodic-forex"]
        return self.__client.get(endpoint, headers=header, params=params).json()

    def inquire_daily_price(self, market_code: str, stock_code: str, end_date: datetime.date):
        return self.__inquire_periodic_price(market_code, stock_code, end_date, '0')

    def inquire_weekly_price(self, market_code: str, stock_code: str, end_date: datetime.date):
        return self.__inquire_periodic_price(market_code, stock_code, end_date, '1')

    def inquire_monthly_price(self, market_code: str, stock_code: str, end_date: datetime.date):
        return self.__inquire_periodic_price(market_code, stock_code, end_date, '2')

    def __inquire_periodic_price(self, market_code: str, stock_code: str, end_date: datetime.date, period_div_code: str):
        header = {"tr_id": tr_id["overseas-stock"]["inquire-periodic-price"]}
        header.update(self.__base_header)

        params = {
            "AUTH": "",
            "EXCD": market_code,
            "SYMB": stock_code,
            "GUBN": period_div_code,
            "BYMD": end_date.strftime("%Y%m%d"),
            "MODP": "1",
        }

        endpoint = uri["origin"]["production"] + uri["overseas-stock"]["inquire-periodic-price"]
        return self.__client.get(endpoint, headers=header, params=params).json()