import httpx
from .config import uri, tr_id


class KISDomesticFutureOption:
    def __init__(self, client: httpx.Client, header: dict[str, str]):
        self.__client = client
        self.__base_header = header

    def inquire_index_future_price(
            self,
            future_code: str
    ) -> dict:
        return self.__inquire_price('F', future_code)

    def __inquire_price(
            self,
            market_div_code: str,
            code: str
    ):
        header = {"tr_id": tr_id["domestic-future-option"]["inquire-price"]}
        header.update(self.__base_header)

        params = {
            "FID_COND_MRKT_DIV_CODE": market_div_code,
            "FID_INPUT_ISCD": code,
        }

        endpoint = uri["origin"]["production"] + uri["domestic-future-option"]["inquire-price"]
        return self.__client.get(endpoint, headers=header, params=params).json()
