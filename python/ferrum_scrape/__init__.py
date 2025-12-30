"""
Web scraping and parsing library with optimized operations.
"""

from .extractor import WebExtractor, ExtractionResult
from .async_extractor import AsyncWebExtractor, batch_extract

__all__ = ["WebExtractor", "ExtractionResult", "AsyncWebExtractor", "batch_extract"]
__version__ = "0.2.0"

