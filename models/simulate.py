#!/usr/bin/env python3
"""
Monte Carlo Simulation: Dynamic Swarm vs Fixed Agentic

Reproduces the calculations from agent-formulas.yaml.
Run with: python models/simulate.py

Reference: docs/FORMULA_DERIVATIONS.md
"""

import random
from dataclasses import dataclass
from typing import List, Tuple

# Seed for reproducibility
RANDOM_SEED = 42
TRIALS = 10_000


@dataclass
class AgentModel:
    """Parameters for an agent architecture."""
    name: str
    base_accuracy: float
    detection_rate: float  # In-context error detection
    fix_rate: float  # Self-correction success
    agent_count: int = 1
    channel_error_rate: float = 0.0

    @property
    def channels(self) -> int:
        """Number of communication channels (n*(n-1)/2)."""
        n = self.agent_count
        return n * (n - 1) // 2

    @property
    def overhead_factor(self) -> float:
        """Communication overhead as error accumulation."""
        if self.channels == 0:
            return 0.0
        return 1 - (1 - self.channel_error_rate) ** self.channels

    @property
    def effective_pre_correction(self) -> float:
        """Accuracy after communication overhead, before self-correction."""
        return self.base_accuracy * (1 - self.overhead_factor)

    @property
    def effective_accuracy(self) -> float:
        """Final effective accuracy including self-correction."""
        pre = self.effective_pre_correction
        return pre + (1 - pre) * self.detection_rate * self.fix_rate


# Model definitions from agent-formulas.yaml
DYNAMIC_SWARM = AgentModel(
    name="Dynamic Swarm + HOTL",
    base_accuracy=0.97,
    detection_rate=0.75,
    fix_rate=0.90,
    agent_count=1,  # Single orchestrator context
    channel_error_rate=0.0,
)

FIXED_INDEPENDENT = AgentModel(
    name="Fixed Agentic (Independent)",
    base_accuracy=0.80,
    detection_rate=0.40,
    fix_rate=0.60,
    agent_count=4,
    channel_error_rate=0.05,
)

FIXED_COORDINATED = AgentModel(
    name="Fixed Agentic (Coordinated)",
    base_accuracy=0.88,
    detection_rate=0.55,
    fix_rate=0.75,
    agent_count=4,
    channel_error_rate=0.03,
)


def analytical_success_rate(model: AgentModel, steps: int) -> float:
    """Calculate success rate analytically: accuracy^steps."""
    return model.effective_accuracy ** steps


def monte_carlo_success_rate(model: AgentModel, steps: int, trials: int = TRIALS) -> Tuple[float, float]:
    """
    Run Monte Carlo simulation.
    Returns (success_rate, standard_error).
    """
    random.seed(RANDOM_SEED)
    successes = 0

    for _ in range(trials):
        # Simulate each step
        success = True
        for _ in range(steps):
            if random.random() > model.effective_accuracy:
                success = False
                break
        if success:
            successes += 1

    rate = successes / trials
    # Standard error for binomial proportion
    se = (rate * (1 - rate) / trials) ** 0.5
    return rate, se


def print_model_params(model: AgentModel) -> None:
    """Print model parameters."""
    print(f"\n{model.name}")
    print(f"  Base accuracy:     {model.base_accuracy:.2%}")
    print(f"  Agent count:       {model.agent_count}")
    print(f"  Channels:          {model.channels}")
    print(f"  Channel error:     {model.channel_error_rate:.2%}")
    print(f"  Overhead factor:   {model.overhead_factor:.2%}")
    print(f"  Pre-correction:    {model.effective_pre_correction:.2%}")
    print(f"  Detection rate:    {model.detection_rate:.2%}")
    print(f"  Fix rate:          {model.fix_rate:.2%}")
    print(f"  Effective acc:     {model.effective_accuracy:.4%}")


def main():
    print("=" * 70)
    print("Monte Carlo Simulation: Dynamic Swarm vs Fixed Agentic")
    print("=" * 70)
    print(f"Trials: {TRIALS:,}")
    print(f"Random seed: {RANDOM_SEED}")

    models = [DYNAMIC_SWARM, FIXED_INDEPENDENT, FIXED_COORDINATED]

    # Print model parameters
    print("\n" + "-" * 70)
    print("MODEL PARAMETERS")
    print("-" * 70)
    for model in models:
        print_model_params(model)

    # Run simulations
    print("\n" + "-" * 70)
    print("SIMULATION RESULTS")
    print("-" * 70)

    steps_list = [5, 10, 20, 50]

    # Header
    print(f"\n{'Steps':>6} | ", end="")
    for model in models:
        print(f"{model.name[:20]:>22} | ", end="")
    print("Advantage (DS/FI)")
    print("-" * 100)

    for steps in steps_list:
        print(f"{steps:>6} | ", end="")

        rates = []
        for model in models:
            analytical = analytical_success_rate(model, steps)
            mc_rate, mc_se = monte_carlo_success_rate(model, steps)
            rates.append((analytical, mc_rate, mc_se))
            print(f"{analytical:>8.2%} (MC:{mc_rate:>6.2%}) | ", end="")

        # Calculate advantage (Dynamic Swarm / Fixed Independent)
        ds_rate = rates[0][0]
        fi_rate = rates[1][0]
        if fi_rate > 0.0001:
            advantage = ds_rate / fi_rate
            print(f"{advantage:>10.1f}x")
        else:
            print(f"{'∞':>10}")

    # Confidence intervals
    print("\n" + "-" * 70)
    print("95% CONFIDENCE INTERVALS (Monte Carlo)")
    print("-" * 70)

    for steps in [10, 20]:
        print(f"\nAt {steps} steps:")
        for model in models:
            mc_rate, mc_se = monte_carlo_success_rate(model, steps)
            ci_low = max(0, mc_rate - 1.96 * mc_se)
            ci_high = min(1, mc_rate + 1.96 * mc_se)
            print(f"  {model.name[:30]:30} {mc_rate:>8.2%} [{ci_low:>6.2%}, {ci_high:>6.2%}]")

    print("\n" + "=" * 70)
    print("Reproduce with: python models/simulate.py")
    print("See: docs/FORMULA_DERIVATIONS.md")
    print("=" * 70)


if __name__ == "__main__":
    main()
