import httpx
import pickle
import datetime
from pathlib import Path
from tempfile import gettempdir
from .access_token import AccessToken
from .config import *


class KISAuth:
    def __init__(self, client: httpx.Client):
        self.__client = client

    def get_access_token(self) -> AccessToken:
        access_token = self.__load_access_token_from_file()
        access_token = self.__validate_access_token(access_token)

        if access_token is not None:
            return access_token

        access_token = self.__get_access_token()
        self.__save_access_token_to_file(access_token)
        return access_token

    @staticmethod
    def __load_access_token_from_file() -> AccessToken | None:
        token_dir = Path(gettempdir()) / 'kis'

        try:
            with open(token_dir / "token.pkl", "rb") as f:
                return pickle.load(f)
        except FileNotFoundError:
            return None

    def __validate_access_token(self, access_token: AccessToken | None) -> AccessToken | None:
        if access_token is None:
            return access_token
        if access_token.expires_at < datetime.datetime.now() + datetime.timedelta(hours=6):
            return None

        access_token = self.__check_access_token(access_token)
        return access_token

    def __check_access_token(self, access_token: AccessToken | None) -> AccessToken | None:
        endpoint = uri["origin"]["production"] + uri["domestic-stock"]["inquire-price"]
        header = {"tr_id": tr_id["domestic-stock"]["inquire-price"]}
        header.update(self.get_base_header(access_token))

        params = {
            "FID_COND_MRKT_DIV_CODE": "J",
            "FID_INPUT_ISCD": "000060"
        }

        response = self.__client.get(
            endpoint,
            params=params,
            headers=header
        ).json()

        return access_token if response["msg_cd"] == "MCA00000" else None

    @staticmethod
    def get_base_header(token: AccessToken) -> dict[str, str]:
        return {
            "content-type": "application/json; charset=utf-8",
            "authorization": "Bearer " + token.value,
            "appkey": secret["appkey"],
            "appsecret": secret["appsecret"]
        }

    def __get_access_token(self) -> AccessToken:
        endpoint = uri["origin"]["production"] + uri["OAuth"]["access_token_request"]
        data = {
            "grant_type": "client_credentials",
            "appkey": secret["appkey"],
            "appsecret": secret["appsecret"]
        }

        return self.__parse_access_token(self.__client.post(endpoint, json=data))

    @staticmethod
    def __parse_access_token(res: httpx.Response) -> AccessToken:
        target = res.json()

        token = AccessToken()
        token.value = target["access_token"]
        token.expires_at = datetime.datetime.strptime(
            target["access_token_token_expired"],
            "%Y-%m-%d %H:%M:%S"
        )

        return token

    @staticmethod
    def __save_access_token_to_file(token: AccessToken):
        token_dir = Path(gettempdir()) / 'kis'
        token_dir.mkdir(parents=True, exist_ok=True)

        with open(token_dir / "token.pkl", "wb") as f:
            pickle.dump(token, f)
