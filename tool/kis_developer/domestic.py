import httpx
import datetime
from .config import uri, tr_id


class KISDomestic:
    def __init__(self, client: httpx.Client, header: dict[str, str]):
        self.__client = client
        self.__base_header = header

    def inquire_price(self, stock_code: str):
        header = {"tr_id": tr_id["domestic-stock"]["inquire-price"]}
        header.update(self.__base_header)

        params = {
            "FID_COND_MRKT_DIV_CODE": "J",
            "FID_INPUT_ISCD": stock_code
        }

        endpoint = uri["origin"]["production"] + uri["domestic-stock"]["inquire-price"]
        return self.__client.get(
            endpoint,
            params=params,
            headers=header
        ).json()

    def inquire_conclusion(self, stock_code: str):
        header = {"tr_id": tr_id["domestic-stock"]["inquire-conclusion"]}
        header.update(self.__base_header)

        params = {
            "FID_COND_MRKT_DIV_CODE": "J",
            "FID_INPUT_ISCD": stock_code
        }

        endpoint = uri["origin"]["production"] + uri["domestic-stock"]["inquire-conclusion"]

        return self.__client.get(
            endpoint,
            params=params,
            headers=header
        ).json()

    def inquire_daily_price(
            self,
            stock_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_price(stock_code, end_date, "D")

    def inquire_weekly_price(
            self,
            stock_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_price(stock_code, end_date, "W")

    def inquire_monthly_price(
            self,
            stock_code: str,
            end_date: datetime.date
    ):
        return self.__inquire_periodic_price(stock_code, end_date, "M")

    def __inquire_periodic_price(
            self,
            stock_code: str,
            end_date: datetime.date,
            period_div_code: str
    ):
        header = {"tr_id": tr_id["domestic-stock"]["inquire-periodic-price"]}
        header.update(self.__base_header)

        params = {
            "FID_COND_MRKT_DIV_CODE": "J",
            "FID_INPUT_ISCD": stock_code,
            "FID_INPUT_DATE_1": "20000101",
            "FID_INPUT_DATE_2": end_date.strftime("%Y%m%d"),
            "FID_PERIOD_DIV_CODE": period_div_code,
            "FID_ORG_ADJ_PRC": "0"
        }

        endpoint = uri["origin"]["production"] + uri["domestic-stock"]["inquire-periodic-price"]
        return self.__client.get(endpoint, headers=header, params=params).json()
