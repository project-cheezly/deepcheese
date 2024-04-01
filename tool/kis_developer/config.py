import os
import tomllib
from pathlib import Path

config_path = Path(__file__).parent / "config.toml"
secret_path = "secret.toml"

with open(config_path, 'rb') as f:
    __config = tomllib.load(f)

    uri = __config['uri']
    tr_id = __config['tr-id']

with open(secret_path, 'rb') as f:
    secret = tomllib.load(f)

    if secret.get('appkey') is None:
        secret['appkey'] = os.environ.get('KIS_APP_KEY')
    if secret.get('appsecret') is None:
        secret['appsecret'] = os.environ.get('KIS_APP_SECRET')

__all__ = ['uri', 'tr_id', 'secret']
