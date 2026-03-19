"""Tests for credential management."""

import os
from unittest.mock import patch

import pytest

from src.dispatch.credentials import (
    CredentialError,
    get_credential,
    validate_all_required,
    validate_credentials,
)


def test_get_credential_present():
    with patch.dict(os.environ, {"ANTHROPIC_API_KEY": "sk-test-123"}):
        cred = get_credential("anthropic")
        assert cred.provider == "anthropic"
        assert cred.api_key == "sk-test-123"


def test_get_credential_missing():
    with patch.dict(os.environ, {}, clear=True):
        with pytest.raises(CredentialError, match="Missing ANTHROPIC_API_KEY"):
            get_credential("anthropic")


def test_get_credential_ollama_no_key():
    cred = get_credential("ollama")
    assert cred.provider == "ollama"
    assert cred.api_key == ""


def test_get_credential_openai():
    with patch.dict(os.environ, {"OPENAI_API_KEY": "sk-openai"}):
        cred = get_credential("openai")
        assert cred.api_key == "sk-openai"


def test_validate_credentials_present():
    with patch.dict(os.environ, {"ANTHROPIC_API_KEY": "sk-test"}):
        assert validate_credentials("anthropic") is True


def test_validate_credentials_missing():
    with patch.dict(os.environ, {}, clear=True):
        assert validate_credentials("anthropic") is False


def test_validate_credentials_ollama():
    assert validate_credentials("ollama") is True


def test_validate_all_required_all_present():
    with patch.dict(os.environ, {"ANTHROPIC_API_KEY": "sk", "OPENAI_API_KEY": "sk"}):
        missing = validate_all_required(["anthropic", "openai"])
        assert missing == []


def test_validate_all_required_some_missing():
    with patch.dict(os.environ, {"ANTHROPIC_API_KEY": "sk"}, clear=True):
        missing = validate_all_required(["anthropic", "openai"])
        assert missing == ["openai"]


def test_validate_all_required_none_needed():
    missing = validate_all_required(["ollama"])
    assert missing == []
