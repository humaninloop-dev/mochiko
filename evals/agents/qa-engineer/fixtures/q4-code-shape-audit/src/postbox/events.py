"""A tiny in-process event bus (T2.4)."""

from collections import defaultdict
from collections.abc import Callable
from typing import Any

Handler = Callable[[dict[str, Any]], None]


class EventBus:
    def __init__(self) -> None:
        self._handlers: dict[str, list[Handler]] = defaultdict(list)

    def subscribe(self, event: str, handler: Handler) -> None:
        self._handlers[event].append(handler)

    def publish(self, event: str, payload: dict[str, Any]) -> None:
        for handler in list(self._handlers.get(event, [])):
            handler(payload)


bus = EventBus()
