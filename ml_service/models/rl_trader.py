import numpy as np

class RLTrader:

    def __init__(self):

        self.actions = ["BUY","SELL","HOLD"]

    def act(self, state):

        return np.random.choice(self.actions)
