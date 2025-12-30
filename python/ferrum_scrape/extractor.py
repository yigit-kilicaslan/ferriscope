"""
WebExtractor - Main class for web scraping and parsing.
"""

import warnings
from typing import Optional, List, Dict, Union, Literal, Any

from .constants import FIELDS_WARNING_MESSAGE

# Type aliases for better IDE support
LinkFilterOption = Literal["internal", "external", "all"]
LinkFilterOptions = Union[LinkFilterOption, List[LinkFilterOption], None]

# Import the Rust extension module (built by maturin)
try:
    # Try absolute import first (maturin installs at top level)
    import _ferrum_scrape_native as _rust_module
except ImportError:
    try:
        # Fallback to relative import
        from . import _ferrum_scrape_native as _rust_module
    except ImportError:
        # Fallback if Rust extension is not built
        _rust_module = None

if _rust_module is not None:
    _PyWebExtractor = _rust_module.PyWebExtractor
    PyExtractionResult = _rust_module.PyExtractionResult
else:
    _PyWebExtractor = None
    PyExtractionResult = None


class WebExtractor:
    """
    Web scraping and parsing extractor with configurable activities.
    
    Example:
        >>> # Extract from URL
        >>> extractor = WebExtractor(url="https://example.com/article")
        >>> extractor.extract_text(language_detection=True)
        >>> extractor.extract_article(["title", "author", "publication_date"])
        >>> extractor.extract_links()  # or extract_links(internal=True), extract_links(external=True), extract_links(all=True)
        >>> result = extractor.run()
        >>> print(result.text)
        >>> print(result.article)
        
        >>> # Extract from HTML string
        >>> html_content = "<html><body><h1>Hello World</h1></body></html>"
        >>> extractor = WebExtractor(url="https://example.com", html=html_content)
        >>> extractor.extract_text()
        >>> result = extractor.run()
        >>> print(result.text)
    """
    
    def __init__(self, url: str, html: Optional[str] = None):
        """
        Initialize the WebExtractor.
        
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
    
    def run(self):
        """
        Execute the extraction with the configured activities.
        
        Returns:
            ExtractionResult: An object containing the extracted data
            
        Raises:
            RuntimeError: If extraction fails or activities are not set
        """
        if not self._activities_set:
            raise RuntimeError(
                "No activities configured. Call extract_text(), extract_article(), extract_links(), etc. before run()."
            )
        
        result = self._extractor.run()
        return ExtractionResult(result)
    
    def enable_robots_check(self) -> None:
        """
        Enable robots.txt checking with in-memory cache.
        The cache is bound to this WebExtractor instance and will be cleared when the extractor is destroyed.
        For proper cleanup, use this with a 'with' statement.
        
        Example:
            >>> with WebExtractor(url="https://example.com") as extractor:
            ...     extractor.enable_robots_check()
            ...     extractor.extract_text()
            ...     result = extractor.run()
        """
        self._extractor.enable_robots_check()
    
    def enable_robots_check_with_redis(self, redis_url: str) -> None:
        """
        Enable robots.txt checking with both in-memory and Redis cache.
        
        Args:
            redis_url: Redis connection URL (e.g., "redis://localhost:6379")
            
        Example:
            >>> extractor = WebExtractor(url="https://example.com")
            >>> extractor.enable_robots_check_with_redis("redis://localhost:6379")
        """
        self._extractor.enable_robots_check_with_redis(redis_url)
    
    def set_robots_redis_ttl(self, ttl_secs: int) -> None:
        """
        Set the TTL (time-to-live) for robots.txt entries in Redis cache.
        
        Args:
            ttl_secs: TTL in seconds (default: 1800 = 30 minutes)
        """
        self._extractor.set_robots_redis_ttl(ttl_secs)
    
    def set_robots_txt(self, content: str) -> None:
        """
        Set robots.txt content manually (plain input).
        This will be cached in memory and/or Redis if enabled.
        
        Args:
            content: The robots.txt content as a string
        """
        self._extractor.set_robots_txt(content)
    
    def check_robots_allowed(self) -> bool:
        """
        Check if the current URL is allowed by robots.txt.
        
        Returns:
            bool: True if allowed, False if disallowed
            
        Example:
            >>> extractor = WebExtractor(url="https://example.com/page")
            >>> extractor.enable_robots_check()
            >>> if extractor.check_robots_allowed():
            ...     extractor.extract_text()
            ...     result = extractor.run()
        """
        return self._extractor.check_robots_allowed()
    
    def remove_robots_from_redis(self) -> None:
        """
        Remove robots.txt from Redis cache for the current domain.
        """
        self._extractor.remove_robots_from_redis()
    
    def clear_robots_cache(self) -> None:
        """
        Clear the in-memory robots.txt cache.
        """
        self._extractor.clear_robots_cache()
    
    def __enter__(self):
        """
        Context manager entry. Returns self for use with 'with' statement.
        
        Example:
            >>> with WebExtractor(url="https://example.com") as extractor:
            ...     extractor.enable_robots_check()  # Cache is bound to this instance
            ...     extractor.extract_text()
            ...     result = extractor.run()
            # Cache is automatically cleared when exiting the 'with' block
        """
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """
        Context manager exit. Clears in-memory robots.txt cache.
        """
        self.clear_robots_cache()
        return False  # Don't suppress exceptions


class ExtractionResult:
    """
    Result object containing extracted data.
    """
    
    def __init__(self, py_result):
        self._result = py_result
    
    @property
    def url(self) -> str:
        """The URL that was scraped."""
        return self._result.url
    
    @property
    def text(self) -> Optional[str]:
        """The extracted text content (if extract_text=True)."""
        return self._result.text
    
    @property
    def links(self) -> Optional[Dict[str, Any]]:
        """
        Grouped links organized by type and domain (if extract_links=True).
        Returns a dictionary with keys: 'internal', 'external', 'by_domain', 'summary'
        """
        return self._result.links
    
    @property
    def language(self) -> Optional[str]:
        """Detected language code (if extract_text has language_detection enabled)."""
        return self._result.language
    
    @property
    def language_confidence(self) -> Optional[float]:
        """Confidence score for language detection (0.0 to 1.0)."""
        return self._result.language_confidence
    
    @property
    def grouped_links(self) -> Optional[Dict[str, Any]]:
        """
        Deprecated: Use links property instead.
        Grouped links organized by type and domain.
        Returns a dictionary with keys: 'internal', 'external', 'by_domain', 'summary'
        """
        return self.links
    
    @property
    def socials(self) -> Optional[Dict[str, str]]:
        """
        Extracted social metadata dictionary (if extract_socials was set).
        Contains Twitter and Open Graph metadata fields.
        """
        return self._result.socials
    
    @property
    def videos(self) -> Optional[Dict[str, str]]:
        """
        Extracted video/book metadata dictionary (if extract_video was set).
        Contains video and book metadata fields.
        """
        return self._result.videos
    
    @property
    def product(self) -> Optional[Dict[str, str]]:
        """
        Extracted product metadata dictionary (if extract_product was set).
        Contains product title, description, price, currency, rating, reviews, etc.
        """
        return self._result.product
    
    @property
    def article(self) -> Optional[Dict[str, str]]:
        """
        Extracted article metadata dictionary (if extract_article was set).
        Contains article title, author, description, publication_date, categories, etc.
        """
        return self._result.article
    
    @property
    def content(self) -> Optional[Dict[str, Any]]:
        """
        Content information including text and text_length.
        Returns a dictionary with keys: 'text', 'text_length'
        """
        return self._result.content
    
    @property
    def result(self) -> Dict[str, Any]:
        """
        Get the grouped result dictionary organized by extraction category.
        Returns a dictionary with keys: 'text', 'socials', 'product', 'videos', 'article', 'links'
        """
        return self._result.to_dict()
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert the result to a grouped dictionary organized by extraction category."""
        return self._result.to_dict()
    
    def __repr__(self):
        return f"ExtractionResult(url={self.url!r}, text_length={len(self.text) if self.text else 0})"

