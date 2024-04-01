import httpx
from .access_token import AccessToken
from .auth import KISAuth
from .config import *
from .domestic import KISDomestic


class KIS:
    def __init__(self):
        self.__client = httpx.Client()
        self.__auth = KISAuth(self.__client)

        token = self.__auth.get_access_token()
        base_header = self.__get_base_header(token)

        self.domestic = KISDomestic(self.__client, base_header)


    @staticmethod
    def __get_base_header(token: AccessToken) -> dict[str, str]:
        return {
            "content-type": "application/json; charset=utf-8",
            "authorization": "Bearer " + token.value,
            "appkey": secret["appkey"],
            "appsecret": secret["appsecret"]
        }
