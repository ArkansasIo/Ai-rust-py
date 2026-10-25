"""Early thermal-budget model for the human.brain skull prototype.

This is an engineering estimator, not CFD or a substitute for component
vendor thermal specifications. Replace provisional module powers with measured
values after selecting silicon and board components.
"""

REGIONS_W = {
    "brainstem": 25.0,
    "thalamus": 30.0,
    "cerebellum": 30.0,
    "hippocampus": 30.0,
    "amygdala": 25.0,
    "basal_ganglia": 30.0,
    "frontal": 60.0,
    "parietal": 30.0,
    "temporal": 45.0,
    "occipital": 60.0,
    "motor": 30.0,
    "somatosensory": 30.0,
}

MASTER_W = 80.0
AUXILIARY_W = 35.0


def estimate() -> dict[str, float]:
    regional = sum(REGIONS_W.values())
    total = regional + MASTER_W + AUXILIARY_W
    return {
        "regional_w": regional,
        "master_w": MASTER_W,
        "auxiliary_w": AUXILIARY_W,
        "estimated_heat_w": total,
        "design_margin_w": total * 0.20,
        "design_heat_w": total * 1.20,
    }


if __name__ == "__main__":
    for key, value in estimate().items():
        print(f"{key}: {value:.1f} W")
