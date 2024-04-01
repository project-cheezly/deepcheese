from dataclasses import dataclass
from datetime import datetime


class AccessToken:
    value: str = None
    expires_at: datetime = None
