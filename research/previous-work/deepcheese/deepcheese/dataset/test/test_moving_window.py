import logging

import pytest


@pytest.fixture()
def moving_window():
    from deepcheese.dataset.moving_window import MovingWindowDataset
    from deepcheese.data_loader.kis_loader import KisLoader

    return MovingWindowDataset(KisLoader())


def test_moving_window(moving_window):
    import plotly.express as px

    fig = px.imshow(moving_window[0].arr[1150].transpose((1, 0)), aspect='auto')
    fig.show()


def test_moving_window_amount_frequency(moving_window):
    import plotly.graph_objects as go

    window = moving_window[0]

    logging.info(window.arr.shape)

    raw_data = window.arr[:50000].flatten()

    fig = go.Figure(data=[go.Histogram(x=raw_data, histnorm='probability')])
    fig.show()

