from postbox.importers.base import registry
from postbox.importers.subscribers import CsvSubscriberImporter

registry.register("subscribers", CsvSubscriberImporter)

__all__ = ["registry"]
