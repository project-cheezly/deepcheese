import logging
import polars as pl


logger = logging.getLogger(__name__)


def test_load_kis_data():
    from deepcheese.data_loader.kis_loader import KisLoader

    kis_loader = KisLoader()
    limit_order_book = kis_loader.lob_data()

    pl.Config.set_tbl_cols(50)
    logger.info(limit_order_book[5][0])
    logger.info(limit_order_book[5][-1])

    assert True
