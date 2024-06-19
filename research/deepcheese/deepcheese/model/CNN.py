import torch
import torch.nn as nn


class CNN(nn.Module):
    def __init__(self):
        super(CNN, self).__init__()

        self.conv1 = torch.nn.Conv2d(1, 32, 3, 1, 1)
        self.conv2 = torch.nn.Conv2d(32, 64, 3, 1, 1)
        self.conv3 = torch.nn.Conv2d(64, 128, 3, 1, 1)
        self.fc1 = torch.nn.Linear(3072, 2)

        self.relu = torch.nn.ReLU()
        self.max_pool = torch.nn.MaxPool2d(kernel_size=2, stride=2)

    def forward(self, x):
        x = x.permute(0, 1, 3, 2)
        x = self.relu(self.conv1(x))
        x = self.max_pool(x)
        x = self.relu(self.conv2(x))
        x = self.max_pool(x)
        x = self.relu(self.conv3(x))
        x = self.max_pool(x)
        x = x.view(x.shape[0], -1)
        x = self.relu(self.fc1(x))

        return x


class CNN3d(nn.Module):
    def __init__(self):
        super(CNN3d, self).__init__()

        self.conv1 = torch.nn.Conv3d(1, 32, (3, 3, 3), 1, 1)
        self.conv2 = torch.nn.Conv3d(32, 64, (3, 3, 3), 1, 1)
        self.conv3 = torch.nn.Conv3d(64, 128, (3, 3, 3), 1, 1)
        self.fc1 = torch.nn.Linear(3072, 2)

        self.relu = torch.nn.ReLU()
        self.max_pool = torch.nn.MaxPool3d(kernel_size=2, stride=2)

    def forward(self, x):
        x = self.relu(self.conv1(x))
        x = self.max_pool(x)
        x = self.relu(self.conv2(x))
        x = self.max_pool(x)
        x = self.relu(self.conv3(x))
        x = self.max_pool(x)

        x = x.view(x.shape[0], -1)
        x = self.relu(self.fc1(x))

        return x