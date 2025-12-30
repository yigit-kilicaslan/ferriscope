"""
Async WebExtractor - Async version for better performance with multiple URLs.
"""

import warnings
from typing import Optional, List, Dict, Any, Union, Literal
import asyncio

from .constants import FIELDS_WARNING_MESSAGE

# Import ExtractionResult from extractor module
try:
    from .extractor import ExtractionResult, LinkFilterOptions
except ImportError:
    from ferrum_scrape.extractor import ExtractionResult, LinkFilterOptions

# Import the Rust extension module (built by maturin)
try:
    import _ferrum_scrape_native as _rust_module
except ImportError:
    try:
        from . import _ferrum_scrape_native as _rust_module
    except ImportError:
        _rust_module = None

if _rust_module is not None:
    _PyWebExtractor = _rust_module.PyWebExtractor
    PyExtractionResult = _rust_module.PyExtractionResult
else:
    _PyWebExtractor = None
    PyExtractionResult = None


class AsyncWebExtractor:
    """
    Async web scraping and parsing extractor for batch processing.
    
    This class provides async methods for better performance when scraping
    multiple URLs concurrently.
    
    Example:
        >>> import asyncio
        >>> async def main():
        ...     # Extract from URL
        ...     extractor = AsyncWebExtractor(url="https://example.com/article")
        ...     extractor.extract_text(language_detection=True)
        ...     extractor.extract_article(["title", "author", "publication_date"])
        ...     extractor.extract_links()  # or extract_links(internal=True), extract_links(external=True), extract_links(all=True)
        ...     result = await extractor.run()
        ...     print(result.text)
        ...     
        ...     # Extract from HTML string
        ...     html_content = "<html><body><h1>Hello World</h1></body></html>"
        ...     extractor = AsyncWebExtractor(url="https://example.com", html=html_content)
        ...     extractor.extract_text()
        ...     result = await extractor.run()
        ...     print(result.text)
        >>> asyncio.run(main())
    """
    
    def __init__(self, url: str, html: Optional[str] = None):
        """
        Initialize the AsyncWebExtractor.
        
        Args:
            url: The URL (used for link resolution and metadata, can be a placeholder if html is provided)
            html: Optional HTML content. If provided, the extractor will use this HTML instead of downloading from the URL.
        """
        if _PyWebExtractor is None:
            raise ImportError(
                "Rust extension not found. Please build the package first:\n"
                "  maturin develop  # for development\n"
                "  maturin build    # for distribution"
            )
        
        self._extractor = _PyWebExtractor(url, html)
        self._activities_set = False
    
    def extract_text(self, language_detection: bool = False) -> None:
        """
        Enable text extraction from the page.
        
        Args:
            language_detection: Whether to detect the language of the extracted text (default: False)
        """
        self._extractor.extract_text(language_detection)
        self._activities_set = True
    
    def extract_links(
        self,
        *,
        internal: bool = False,
        external: bool = False,
        all: bool = False
    ) -> None:
        """
        Enable link extraction from the page.
        
        Args:
            internal: Extract only internal links (same domain) (default: False)
            external: Extract only external links (different domain) (default: False)
            all: Extract all links (both internal and external) (default: False)
            
        Note:
            If no options are specified (all False), defaults to extracting all links.
            If multiple options are True, "all" takes precedence, then both internal and external are extracted.
        """
        fields = []
        # If all is True, or both internal and external are True, extract all
        if all or (internal and external):
            fields = ["all"]
        elif internal:
            fields = ["internal"]
        elif external:
            fields = ["external"]
        else:
            # If nothing specified, default to all
            fields = ["all"]
        self._extractor.extract_links(fields)
        self._activities_set = True
    
    def extract_socials(
        self,
        fields: Optional[List[str]] = None
    ) -> None:
        """
        Enable social metadata extraction with specified fields.
        
        Args:
            fields: List of specific fields to extract. If None or not provided, extracts all fields (default: None, which extracts all)
            
        Example:
            >>> extractor.extract_socials()  # Extract all fields
            >>> extractor.extract_socials(fields=["twitter:card", "og:title"])  # Extract only specific fields
            >>> extractor.extract_socials(fields=["all"])  # Explicitly extract all fields
        """
        if fields is None:
            warnings.warn(
                FIELDS_WARNING_MESSAGE,
                UserWarning,
                stacklevel=2
            )
            fields = ["all"]
        self._extractor.extract_socials(fields)
        self._activities_set = True
    
    def extract_video(
        self,
        fields: Optional[List[str]] = None
    ) -> None:
        """
        Enable video/book metadata extraction with specified fields.
        
        Args:
            fields: List of specific fields to extract. If None or not provided, extracts all fields (default: None, which extracts all)
            
        Example:
            >>> extractor.extract_video()  # Extract all fields
            >>> extractor.extract_video(fields=["video:duration", "video:title"])  # Extract only specific fields
            >>> extractor.extract_video(fields=["all"])  # Explicitly extract all fields
        """
        if fields is None:
            warnings.warn(
                FIELDS_WARNING_MESSAGE,
                UserWarning,
                stacklevel=2
            )
            fields = ["all"]
        self._extractor.extract_video(fields)
        self._activities_set = True
    
    def extract_product(
        self,
        fields: Optional[List[str]] = None
    ) -> None:
        """
        Enable product metadata extraction with specified fields.
        
        Args:
            fields: List of specific fields to extract. If None or not provided, extracts all fields (default: None, which extracts all)
            
        Example:
            >>> extractor.extract_product()  # Extract all fields
            >>> extractor.extract_product(fields=["price", "currency"])  # Extract only specific fields
            >>> extractor.extract_product(fields=["all"])  # Explicitly extract all fields
        """
        if fields is None:
            warnings.warn(
                FIELDS_WARNING_MESSAGE,
                UserWarning,
                stacklevel=2
            )
            fields = ["all"]
        self._extractor.extract_product(fields)
        self._activities_set = True
    
    def extract_article(
        self,
        fields: Optional[List[str]] = None
    ) -> None:
        """
        Enable article metadata extraction with specified fields.
        
        Args:
            fields: List of specific fields to extract. If None or not provided, extracts all fields (default: None, which extracts all)
            
        Example:
            >>> extractor.extract_article()  # Extract all fields
            >>> extractor.extract_article(fields=["published_date"])  # Extract only published_date
            >>> extractor.extract_article(fields=["title", "author", "published_date"])  # Extract multiple specific fields
            >>> extractor.extract_article(fields=["all"])  # Explicitly extract all fields
        """
        if fields is None:
            warnings.warn(
                FIELDS_WARNING_MESSAGE,
                UserWarning,
                stacklevel=2
            )
            fields = ["all"]
        self._extractor.extract_article(fields)
        self._activities_set = True
    
    def set_timeout(self, timeout_secs: float) -> None:
        """
        Set the HTTP request timeout in seconds.
        
        Args:
            timeout_secs: Timeout in seconds (will be converted to u64)
        """
        self._extractor.set_timeout(int(timeout_secs))
    
    def set_user_agent(self, user_agent: str) -> None:
        """
        Set a custom user agent string.
        
        Args:
            user_agent: The user agent string to use
        """
        self._extractor.set_user_agent(user_agent)
    
    def set_random_user_agent(self, enabled: bool = True) -> None:
        """
        Enable or disable random user agent generation.
        When enabled, a random user agent will be selected for each request.
        
        Args:
            enabled: Whether to use random user agents (default: True)
        """
        self._extractor.set_random_user_agent(enabled)
    
    def add_header(self, name: str, value: str) -> None:
        """
        Add a custom HTTP header.
        
        Args:
            name: Header name (e.g., "Authorization", "Accept")
            value: Header value
        """
        self._extractor.add_header(name, value)
    
    def set_headers(self, headers: Dict[str, str]) -> None:
        """
        Set multiple HTTP headers at once, replacing any existing headers.
        
        Args:
            headers: Dictionary of header name-value pairs
        """
        self._extractor.set_headers(headers)
    
    async def run(self):
        """
        Execute the extraction with the configured activities (async).
        
        Note: This runs the synchronous Rust code in a thread pool to avoid
        blocking the event loop. For true async performance, use batch_extract
        for multiple URLs.
        
        Returns:
            ExtractionResult: An object containing the extracted data
            
        Raises:
            RuntimeError: If extraction fails or activities are not set
        """
        if not self._activities_set:
            raise RuntimeError(
                "No activities configured. Call extract_text(), extract_article(), extract_links(), etc. before run()."
            )
        
        # Run in thread pool to avoid blocking
        loop = asyncio.get_event_loop()
        result = await loop.run_in_executor(None, self._extractor.run)
        return ExtractionResult(result)


async def batch_extract(
    urls: List[str],
    extract_text: bool = False,
    language_detection: bool = False,
    extract_links_internal: bool = False,
    extract_links_external: bool = False,
    extract_links_all: bool = False,
    extract_socials_all: bool = True,
    extract_video_all: bool = True,
    extract_product_all: bool = True,
    extract_article_all: bool = True,
    max_concurrent: int = 10
) -> List[Any]:
    """
    Extract data from multiple URLs concurrently.
    
    Args:
        urls: List of URLs to scrape
        extract_text: Whether to extract text content from the page (default: False)
        language_detection: Whether to detect the language of the extracted text (default: False)
        extract_links_internal: Extract only internal links (same domain) (default: False)
        extract_links_external: Extract only external links (different domain) (default: False)
        extract_links_all: Extract all links (both internal and external) (default: False)
        extract_socials_all: Extract all social metadata fields (default: True)
        extract_video_all: Extract all video/book metadata fields (default: True)
        extract_product_all: Extract all product metadata fields (default: True)
        extract_article_all: Extract all article metadata fields (default: True)
        max_concurrent: Maximum number of concurrent requests
        
    Returns:
        List of ExtractionResult objects in the same order as input URLs
        
    Example:
        >>> import asyncio
        >>> async def main():
        ...     urls = ["https://example.com/1", "https://example.com/2"]
        ...     results = await batch_extract(
        ...         urls,
        ...         extract_text=True,
        ...         extract_article_all=True
        ...     )
        ...     for result in results:
        ...         print(result.article.get("title"))
        >>> asyncio.run(main())
    """
    semaphore = asyncio.Semaphore(max_concurrent)
    
    async def extract_one(url: str):
        async with semaphore:
            extractor = AsyncWebExtractor(url)
            if extract_text:
                extractor.extract_text(language_detection=language_detection)
            if extract_links_internal or extract_links_external or extract_links_all:
                extractor.extract_links(
                    internal=extract_links_internal,
                    external=extract_links_external,
                    all=extract_links_all
                )
            if extract_socials_all:
                extractor.extract_socials()  # Extracts all by default
            if extract_video_all:
                extractor.extract_video()  # Extracts all by default
            if extract_product_all:
                extractor.extract_product()  # Extracts all by default
            if extract_article_all:
                extractor.extract_article()  # Extracts all by default
            try:
                return await extractor.run()
            except Exception as e:
                # Return None for errors - could be enhanced to return error results
                return None
    
    tasks = [extract_one(url) for url in urls]
    results = await asyncio.gather(*tasks, return_exceptions=True)
    
    # Filter out None results (errors)
    return [r for r in results if r is not None and not isinstance(r, Exception)]

