import torch
import torch.nn as nn

class TransformerModel(nn.Module):

    def __init__(self, input_dim=1, model_dim=64):

        super().__init__()

        self.encoder = nn.TransformerEncoder(
            nn.TransformerEncoderLayer(d_model=model_dim, nhead=4),
            num_layers=2
        )

        self.fc = nn.Linear(model_dim, 1)

    def forward(self, x):

        out = self.encoder(x)

        return self.fc(out[-1])
