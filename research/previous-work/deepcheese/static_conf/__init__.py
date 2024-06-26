import logging

try:
    from ._config import Config
except ImportError:
    from ._build import build
    from ._singleton import Singleton
    logging.warn("Config file not found. Building config file...")
    build()

    from ._config import Config
