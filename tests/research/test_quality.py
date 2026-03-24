"""Tests for research quality scoring."""

from src.research.quality import deep_score, fast_gate, standard_score


# --- Tier 1: Fast Gate ---

def test_fast_gate_too_short():
    r = fast_gate("short")
    assert not r.passed
    assert r.score == 0.0

def test_fast_gate_empty():
    r = fast_gate("")
    assert not r.passed

def test_fast_gate_few_sentences():
    r = fast_gate("A" * 300)  # Long but no sentence structure
    assert not r.passed
    assert r.details["reason"] == "few_sentences"

def test_fast_gate_low_density():
    # Enough sentences but very repetitive words
    content = ". ".join(["the cat sat on the mat and the cat sat on the mat again"] * 20) + "."
    r = fast_gate(content)
    assert r.tier == 1
    assert r.details.get("reason") == "low_density"

def test_fast_gate_good_content():
    content = (
        "Research shows that attachment theory explains many relationship patterns. "
        "Studies have demonstrated that secure attachment in childhood correlates with "
        "healthier adult relationships across multiple longitudinal samples. "
        "The seminal work by Bowlby established the theoretical framework. "
        "Ainsworth extended this with the Strange Situation experiment. "
        "Meta-analyses by Fraley confirm effect sizes in the medium to large range. "
        "However, critics like Kagan note that temperament may confound results. "
        "Cross-cultural studies by van Ijzendoorn show variation in distribution. "
        "Hazan and Shaver applied attachment to adult romantic relationships. "
        "Main and Hesse identified disorganized attachment as a fourth category. "
        "The Adult Attachment Interview provides a validated assessment tool. "
        "Neuroimaging studies link attachment security to prefrontal regulation. "
    )
    r = fast_gate(content)
    assert r.passed is True
    assert r.score >= 0.3

def test_fast_gate_boilerplate():
    # Content that passes density but has boilerplate words
    content = (
        "This comprehensive research article explores important psychological findings. "
        "Subscribe to receive updates about the latest breakthroughs in the field. "
        "Click here to access additional resources and supplementary materials. "
        "Our newsletter delivers curated content from leading researchers worldwide. "
        "Cookie preferences can be adjusted through the settings panel below. "
        "The methodology section describes the experimental design in detail. "
        "Statistical analysis revealed significant differences between groups. "
        "Participants completed standardized questionnaires measuring key variables. "
        "Results indicate a strong correlation between the independent predictors. "
        "Discussion examines the theoretical and practical implications of findings. "
        "Limitations include the relatively small sample size and geographic scope. "
        "Future research should investigate longitudinal patterns over five years. "
    )
    r = fast_gate(content)
    assert r.tier == 1
    assert "boilerplate_ratio" in r.details


# --- Tier 2: Standard Score ---

def test_standard_all_yes():
    r = standard_score(True, True, True, True, True, True)
    assert r.score == 1.0
    assert r.passed is True

def test_standard_all_no():
    r = standard_score(False, False, False, False, False, False)
    assert r.score == 0.0
    assert r.passed is False

def test_standard_threshold():
    r = standard_score(True, True, True, False, False, False)
    assert r.score == 0.5
    assert r.passed is True

def test_standard_below_threshold():
    r = standard_score(True, True, False, False, False, False)
    assert r.score < 0.5
    assert r.passed is False


# --- Tier 3: Deep Score ---

def test_deep_perfect():
    r = deep_score(25, 25, 25, 25)
    assert r.score == 1.0
    assert r.passed is True
    assert r.details["total"] == 100

def test_deep_zero():
    r = deep_score(0, 0, 0, 0)
    assert r.score == 0.0
    assert r.passed is False

def test_deep_threshold():
    r = deep_score(18, 18, 17, 17)
    assert r.score == 0.7
    assert r.passed is True

def test_deep_below_threshold():
    r = deep_score(15, 15, 15, 15)
    assert r.score == 0.6
    assert r.passed is False
