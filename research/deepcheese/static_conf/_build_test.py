def test_config():
    from . import Config

    config = Config()
    config2 = Config()

    assert config.data_loader == config2.data_loader
