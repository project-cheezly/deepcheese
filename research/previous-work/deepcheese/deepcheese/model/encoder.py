import torch
import torch.nn as nn

from static_conf import Config

device = torch.device("cuda" if torch.cuda.is_available() else "cpu")


class Encoder(nn.Module):
    """
    Base Encoder for basic moving window dataset
    """
    def __init__(self):
        super(Encoder, self).__init__()

        moving_window_config = Config().dataset.moving_window

        self.input_size = moving_window_config.time_window * 2 * moving_window_config.price_window

        self.fc0 = torch.nn.Linear(self.input_size, 256)
        self.fc1 = torch.nn.Linear(256, 128)
        self.fc2 = torch.nn.Linear(128, 64)
        self.relu = torch.nn.ReLU()

    def forward(self, x):
        x = x.view(-1, self.input_size)
        x = self.relu(self.fc0(x))
        x = self.relu(self.fc1(x))
        x = self.relu(self.fc2(x))

        return x