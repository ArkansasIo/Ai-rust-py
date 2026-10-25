# AI Brain / Brain-Inspired AI Research — 2026

## Research direction

The project should use a hybrid synthetic-cognition architecture rather than treating a large language model as an entire brain.

### Evidence-backed design themes

1. **Neuromorphic computing** — brain-inspired architectures emphasize parallelism, event-driven processing, low latency, and energy efficiency.
2. **Spiking neural networks** — useful for temporal/event-driven processing and hardware-oriented learning.
3. **Continual learning** — online adaptation is a major research direction, especially for edge and embodied systems.
4. **Predictive/world models** — perception can be combined with prediction, memory, and action selection.
5. **Cognitive architectures** — explicit modules for attention, memory, executive control, goals, and embodiment remain useful complements to foundation models.
6. **Biohybrid/organoid intelligence** — current research explores living neural organoids as computational substrates, but this is a distinct experimental field and should not be conflated with software-only AI.
7. **Neuromorphic hardware co-design** — learning algorithms and hardware can be designed together for lower-power adaptive systems.

## Implications for Ai-rust-py

The synthetic brain should therefore contain:

- persistent episodic and semantic memory;
- attention and salience selection;
- a global-workspace-style coordination layer;
- symbolic and neural reasoning interfaces;
- emotion and homeostatic state models;
- personality and self-model state;
- continual-learning and reinforcement-learning interfaces;
- sensorimotor and embodied interfaces;
- Ollama as one language/reasoning model service, not the whole brain;
- optional spiking/neuromorphic backends;
- measurable evaluation loops for learning, safety, latency, energy, and task performance.

## Key 2026 references

- Brown & Varghese, *Computing inspired by the brain: a journey from algorithms to organoids*, Nature Computational Science, July 2026.
- Lan et al., *Toward brain-inspired intelligence: a review of photonic neuromorphic computing systems*, PhotoniX, August 2026.
- Qin et al., *Zeroth-Order Forward-Only SNN Training Inspiring Neuromorphic On-Chip Learning*, ICML 2026.
- Hajizada et al., *Online Continual Learning on Intel Loihi 2 via a Co-designed Spiking Neural Network*, 2026.
- *Artificial intelligence as a surrogate brain: bridging neural dynamical models and data*, National Science Review, 2026.

## Engineering rule

Research findings are architectural inputs, not proof that a software system possesses biological consciousness. The project should describe its target as a **synthetic cognitive architecture / brain-inspired computational system**.
