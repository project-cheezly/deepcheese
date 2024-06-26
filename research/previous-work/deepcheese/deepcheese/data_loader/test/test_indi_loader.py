import logging
import polars as pl


logger = logging.getLogger(__name__)


def test_load_indi_data():
    from deepcheese.data_loader.indi_loader import IndiLoader

    indi_loader = IndiLoader()

    limit_order_book = [x for x in indi_loader.lob_data()]

    pl.Config.set_tbl_cols(50)
    logger.info(limit_order_book[5][0])
    logger.info(limit_order_book[5][-1])

    assert True
