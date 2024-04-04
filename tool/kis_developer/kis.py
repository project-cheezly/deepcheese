import httpx
from .access_token import AccessToken
from .auth import KISAuth
from .config import *
from .domestic import KISDomestic
from .overseas import KISOverseas


class KIS:
    def __init__(self):
        self.__client = httpx
        self.__auth = KISAuth(self.__client)

        token = self.__auth.get_access_token()
        base_header = self.__auth.get_base_header(token)

        self.domestic = KISDomestic(self.__client, base_header)
        self.overseas = KISOverseas(self.__client, base_header)
