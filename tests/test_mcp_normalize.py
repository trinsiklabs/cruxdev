"""Tests for MCP input normalization."""

from src.mcp_normalize import (
    coerce_feature,
    normalize_competitors,
    to_dict_list,
    to_int,
    to_json_string,
    to_pipe_list,
    to_string_list,
)


class TestToJsonString:
    def test_already_string(self):
        assert to_json_string('[{"name": "x"}]') == '[{"name": "x"}]'

    def test_dict(self):
        assert to_json_string({"a": 1}) == '{"a": 1}'

    def test_list(self):
        assert to_json_string([1, 2]) == "[1, 2]"

    def test_none(self):
        assert to_json_string(None) == "[]"


class TestToStringList:
    def test_comma_separated(self):
        assert to_string_list("a, b, c") == ["a", "b", "c"]

    def test_json_array(self):
        assert to_string_list('["x", "y"]') == ["x", "y"]

    def test_actual_list(self):
        assert to_string_list(["a", "b"]) == ["a", "b"]

    def test_none(self):
        assert to_string_list(None) == []

    def test_empty_string(self):
        assert to_string_list("") == []

    def test_single_value(self):
        assert to_string_list(42) == ["42"]

    def test_strips_whitespace(self):
        assert to_string_list("  a , b  ") == ["a", "b"]

    def test_filters_empty(self):
        assert to_string_list("a,,b,") == ["a", "b"]

    def test_invalid_json_array(self):
        assert to_string_list("[not valid json") == ["[not valid json"]

    def test_json_array_of_numbers(self):
        assert to_string_list("[1, 2, 3]") == ["1", "2", "3"]


class TestToDictList:
    def test_json_string(self):
        result = to_dict_list('[{"name": "x"}, {"name": "y"}]')
        assert len(result) == 2

    def test_actual_list(self):
        result = to_dict_list([{"a": 1}, {"b": 2}])
        assert len(result) == 2

    def test_filters_non_dicts(self):
        result = to_dict_list([{"a": 1}, "not a dict", 42])
        assert len(result) == 1

    def test_none(self):
        assert to_dict_list(None) == []

    def test_single_dict(self):
        result = to_dict_list({"name": "x"})
        assert result == [{"name": "x"}]

    def test_invalid_json(self):
        assert to_dict_list("not json") == []

    def test_json_single_dict(self):
        result = to_dict_list('{"name": "x"}')
        assert result == [{"name": "x"}]

    def test_non_iterable(self):
        assert to_dict_list(42) == []


class TestToInt:
    def test_int(self):
        assert to_int(5) == 5

    def test_string(self):
        assert to_int("42") == 42

    def test_float(self):
        assert to_int(3.7) == 3

    def test_none(self):
        assert to_int(None) == 0

    def test_none_with_default(self):
        assert to_int(None, 10) == 10

    def test_invalid_string(self):
        assert to_int("abc") == 0

    def test_float_string(self):
        assert to_int("3.7") == 3

    def test_invalid_float_string(self):
        assert to_int("abc.def") == 0

    def test_non_numeric_type(self):
        assert to_int([], 5) == 5


class TestToPipeList:
    def test_pipe_separated(self):
        assert to_pipe_list("a|b|c") == ["a", "b", "c"]

    def test_none(self):
        assert to_pipe_list(None) == []

    def test_list(self):
        assert to_pipe_list(["a", "b"]) == ["a", "b"]


class TestCoerceFeature:
    def test_dict_with_name(self):
        assert coerce_feature({"name": "x", "has": True}) == {"name": "x", "has": True}

    def test_dict_without_has(self):
        assert coerce_feature({"name": "x"}) == {"name": "x", "has": True}

    def test_string(self):
        assert coerce_feature("auto-test") == {"name": "auto-test", "has": True}

    def test_none(self):
        assert coerce_feature(None) is None

    def test_dict_without_name(self):
        assert coerce_feature({"has": True}) is None

    def test_number(self):
        assert coerce_feature(42) is None


class TestNormalizeCompetitors:
    def test_json_string(self):
        raw = '[{"name": "A", "features": [{"name": "x", "has": true}]}]'
        result = normalize_competitors(raw)
        assert len(result) == 1
        assert result[0]["name"] == "A"
        assert result[0]["features"] == [{"name": "x", "has": True}]

    def test_string_features(self):
        raw = [{"name": "A", "features": ["x", "y"]}]
        result = normalize_competitors(raw)
        assert result[0]["features"] == [
            {"name": "x", "has": True},
            {"name": "y", "has": True},
        ]

    def test_missing_fields(self):
        raw = [{"name": "A"}]
        result = normalize_competitors(raw)
        assert result[0]["url"] == ""
        assert result[0]["category"] == "noted"
        assert result[0]["features"] == []

    def test_mixed_feature_types(self):
        raw = [{"name": "A", "features": [{"name": "x"}, "y", None, 42]}]
        result = normalize_competitors(raw)
        assert len(result[0]["features"]) == 2  # None and 42 filtered

    def test_none(self):
        assert normalize_competitors(None) == []

    def test_filters_non_dicts(self):
        raw = [{"name": "A"}, "not a dict"]
        result = normalize_competitors(raw)
        assert len(result) == 1
