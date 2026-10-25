class SimpleBandit:
    """Minimal epsilon-greedy reinforcement-learning demonstration."""

    def __init__(self, rewards, epsilon=0.1):
        self.rewards = rewards
        self.epsilon = epsilon
        self.values = [0.0] * len(rewards)
        self.counts = [0] * len(rewards)

    def select_action(self):
        import random
        if random.random() < self.epsilon:
            return random.randrange(len(self.rewards))
        return max(range(len(self.values)), key=self.values.__getitem__)

    def update(self, action, reward):
        self.counts[action] += 1
        n = self.counts[action]
        self.values[action] += (reward - self.values[action]) / n
