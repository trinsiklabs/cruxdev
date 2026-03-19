"""Credential management — startup validation.

Fail fast before convergence starts if required credentials are missing.
"""

import os
from dataclasses import dataclass


class CredentialError(Exception):
    """Required credentials are missing or invalid."""
    pass


@dataclass
class ProviderCredentials:
    provider: str
    api_key: str


PROVIDER_ENV_VARS = {
    "anthropic": "ANTHROPIC_API_KEY",
    "openai": "OPENAI_API_KEY",
    "ollama": None,  # No key needed
}


def get_credential(provider: str) -> ProviderCredentials:
    """Get API credential for a provider from environment.

    Raises CredentialError if required and missing.
    """
    env_var = PROVIDER_ENV_VARS.get(provider)
    if env_var is None:
        # Provider doesn't need credentials (e.g., Ollama)
        return ProviderCredentials(provider=provider, api_key="")

    api_key = os.environ.get(env_var, "")
    if not api_key:
        raise CredentialError(
            f"Missing {env_var} for provider '{provider}'. "
            f"Set it in your environment before starting convergence."
        )

    return ProviderCredentials(provider=provider, api_key=api_key)


def validate_credentials(provider: str) -> bool:
    """Check if credentials are available. Returns True/False, doesn't raise."""
    try:
        get_credential(provider)
        return True
    except CredentialError:
        return False


def validate_all_required(providers: list[str]) -> list[str]:
    """Validate credentials for all required providers.

    Returns list of provider names with missing credentials.
    """
    missing = []
    for provider in providers:
        if not validate_credentials(provider):
            missing.append(provider)
    return missing
