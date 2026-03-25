"""Tests for agent evolution module."""

from src.improvement.agent_evolution import (
    AgentConfig,
    Population,
    mutate_config,
    validate_invariants,
)


class TestAgentConfig:
    def test_get_param(self):
        config = AgentConfig("1", "test", {"x": 10})
        assert config.get("x") == 10
        assert config.get("y", 5) == 5


class TestPopulation:
    def test_add_variant(self):
        pop = Population(max_size=3)
        assert pop.add(AgentConfig("1", "v1", fitness=0.5)) is True
        assert pop.add(AgentConfig("2", "v2", fitness=0.8)) is True
        assert len(pop.variants) == 2

    def test_max_size_enforced(self):
        pop = Population(max_size=1)
        pop.add(AgentConfig("1", "v1"))
        assert pop.add(AgentConfig("2", "v2")) is False

    def test_best(self):
        pop = Population()
        pop.add(AgentConfig("1", "low", fitness=0.3))
        pop.add(AgentConfig("2", "high", fitness=0.9))
        pop.add(AgentConfig("3", "mid", fitness=0.6))
        assert pop.best.id == "2"

    def test_best_empty(self):
        pop = Population()
        assert pop.best is None

    def test_select(self):
        pop = Population()
        pop.add(AgentConfig("1", "a", fitness=0.3))
        pop.add(AgentConfig("2", "b", fitness=0.9))
        pop.add(AgentConfig("3", "c", fitness=0.6))
        kept = pop.select(2)
        assert len(kept) == 2
        assert kept[0].fitness == 0.9
        assert kept[1].fitness == 0.6


class TestValidateInvariants:
    def test_valid_config(self):
        config = AgentConfig("1", "test", {"convergence_threshold": 2, "max_rounds_floor": 3})
        assert validate_invariants(config) == []

    def test_low_convergence_threshold(self):
        config = AgentConfig("1", "test", {"convergence_threshold": 1})
        violations = validate_invariants(config)
        assert len(violations) == 1
        assert "convergence_threshold" in violations[0]

    def test_low_max_rounds(self):
        config = AgentConfig("1", "test", {"max_rounds_floor": 2})
        violations = validate_invariants(config)
        assert len(violations) == 1

    def test_disabled_safety(self):
        config = AgentConfig("1", "test", {"safety_pipeline": False})
        assert len(validate_invariants(config)) == 1

    def test_disabled_human_escalation(self):
        config = AgentConfig("1", "test", {"human_escalation": False})
        assert len(validate_invariants(config)) == 1

    def test_multiple_violations(self):
        config = AgentConfig("1", "test", {
            "convergence_threshold": 1,
            "safety_pipeline": False,
        })
        assert len(validate_invariants(config)) == 2


class TestMutateConfig:
    def test_valid_mutation(self):
        base = AgentConfig("1", "base", {"audit_order": ["a", "b"], "convergence_threshold": 2})
        variant = mutate_config(base, {"audit_order": ["b", "a"]}, "2")
        assert variant is not None
        assert variant.params["audit_order"] == ["b", "a"]
        assert variant.params["convergence_threshold"] == 2  # preserved

    def test_blocked_mutation(self):
        base = AgentConfig("1", "base", {"convergence_threshold": 2})
        variant = mutate_config(base, {"convergence_threshold": 1}, "2")
        assert variant is None  # Violates invariant

    def test_mutation_doesnt_modify_base(self):
        base = AgentConfig("1", "base", {"x": [1, 2, 3]})
        variant = mutate_config(base, {"x": [4, 5]}, "2")
        assert base.params["x"] == [1, 2, 3]  # Unchanged
        assert variant.params["x"] == [4, 5]
