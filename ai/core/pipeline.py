class AIPipeline:
    def __init__(self, stages=None):
        self.stages = stages or []

    def add_stage(self, stage):
        self.stages.append(stage)
        return self

    def run(self, value):
        for stage in self.stages:
            value = stage(value)
        return value
