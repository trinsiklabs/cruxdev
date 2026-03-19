"""Schema validation with retry logic.

Every LLM output is validated before the engine acts on it.
Failed validation retries with error feedback, then escalates.
"""

import json
from typing import Callable, TypeVar

from pydantic import BaseModel, ValidationError

T = TypeVar("T", bound=BaseModel)


class EscalationRequired(Exception):
    """LLM output failed validation after all retries."""
    pass


def validate_json(raw: str, schema: type[T]) -> T:
    """Parse and validate a JSON string against a Pydantic model."""
    try:
        data = json.loads(raw)
    except json.JSONDecodeError as e:
        raise ValidationError.from_exception_data(
            title=schema.__name__,
            line_errors=[],
        ) from e
    return schema.model_validate(data)


def validate_and_retry(
    llm_call: Callable[..., str],
    schema: type[T],
    max_retries: int = 2,
) -> T:
    """Call LLM, validate output, retry on schema failure.

    Args:
        llm_call: Callable that returns raw JSON string.
                  Accepts optional error_feedback kwarg on retries.
        schema: Pydantic model to validate against.
        max_retries: Number of retry attempts after first failure.

    Returns:
        Validated Pydantic model instance.

    Raises:
        EscalationRequired: After all retries exhausted.
    """
    last_error = None
    for attempt in range(max_retries + 1):
        if attempt == 0:
            raw = llm_call()
        else:
            raw = llm_call(error_feedback=str(last_error))

        try:
            return validate_json(raw, schema)
        except (ValidationError, json.JSONDecodeError) as e:
            last_error = e
            if attempt == max_retries:
                raise EscalationRequired(
                    f"LLM output failed validation after {max_retries + 1} attempts: {e}"
                ) from e
    # Unreachable, but satisfies type checker
    raise EscalationRequired("Unexpected validation failure")  # pragma: no cover
