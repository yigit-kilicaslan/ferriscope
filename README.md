# Ferriscope

A blazingly fast, high-performance web scraping and parsing library built in Rust and exposed to Python via PyO3. Designed for efficient, configurable parsing operations with native async support to enable scalable batch processing and high-throughput workloads.

## Features

- **Fast HTTP Requests**: Built on async reqwest for efficient web requests
- **Async Processing**: Support for concurrent batch processing of multiple URLs
- **Text Extraction**: Clean text extraction from HTML content with optional language detection
- **Link Extraction**: Extract links with intelligent grouping (internal/external/by domain) and filtering
- **Social Metadata Extraction**: Extract Twitter Cards and Open Graph metadata
- **Video/Book Metadata Extraction**: Extract video and book metadata from structured data
- **Product Metadata Extraction**: Extract product information including pricing, ratings, and reviews
- **Article Metadata Extraction**: Extract article information including title, author, dates, and categories
- **Language Detection**: Automatic language detection using whatlang
- **Robots.txt Support**: Check robots.txt compliance with in-memory and Redis caching
- **HTTP Client Configuration**: Custom timeouts, user agents (including random rotation), and headers
- **HTML Input Support**: Can work with provided HTML content instead of downloading
- **DOM Index Optimization**: Efficient single-pass parsing with reusable DOM index
- **Optimized Operations**: Only performs requested operations to minimize overhead
- **Modular Architecture**: Clean, maintainable codebase with proper error handling

## Installation

### Prerequisites

- Rust (latest stable version) - [Install Rust](https://www.rust-lang.org/tools/install)
- Python 3.7+
- maturin: `pip install maturin`

### Build and Install

**For development (recommended for testing):**
```bash
# This will build and install the package in development mode
maturin develop
```

**For production:**
```bash
# Build wheel
maturin build --release

# Install the wheel
pip install target/wheels/ferriscope-*.whl
```

**Note:** The first build may take a few minutes as it compiles the Rust code. Subsequent builds will be faster thanks to incremental compilation.

## Usage

### Basic Usage

```python
from scrape_tools import WebExtractor

# Create an extractor instance
extractor = WebExtractor(url="https://example.com/article")

# Configure the activities you want to perform
extractor.extract_text(language_detection=True)  # Extract text with language detection
extractor.extract_article()  # Extract all article metadata
extractor.extract_socials()  # Extract all social metadata
extractor.extract_links()  # Extract all links

# Run the extraction
result = extractor.run()

# Access the results
print(f"URL: {result.url}")
print(f"Text length: {len(result.text) if result.text else 0}")
print(f"Language: {result.language} (confidence: {result.language_confidence})")
print(f"Article title: {result.article.get('title')}")
print(f"Article author: {result.article.get('author')}")
print(f"Social metadata: {result.socials}")

# Access grouped links
if result.links:
    print(f"Total links: {result.links['summary']['total']}")
    print(f"Internal links: {result.links['summary']['internal_count']}")
    print(f"External links: {result.links['summary']['external_count']}")

# Convert to grouped dictionary
data = result.to_dict()  # Returns: {"text": {...}, "links": {...}, "socials": {...}, ...}
```

### Extract from HTML String

```python
from scrape_tools import WebExtractor

# Provide HTML content directly
html_content = "<html><body><h1>Hello World</h1></body></html>"
extractor = WebExtractor(url="https://example.com", html=html_content)
extractor.extract_text()
result = extractor.run()
print(result.text)
```

### Link Extraction with Filtering

```python
from scrape_tools import WebExtractor

extractor = WebExtractor(url="https://example.com")

# Extract only internal links
extractor.extract_links(internal=True)

# Extract only external links
extractor.extract_links(external=True)

# Extract all links (default)
extractor.extract_links(all=True)
# or simply
extractor.extract_links()

result = extractor.run()

# Links are grouped by type
if result.links:
    internal_links = result.links['internal']  # List of internal links
    external_links = result.links['external']  # List of external links
    by_domain = result.links['by_domain']  # Dict of {domain: [links]}
    summary = result.links['summary']  # Summary statistics
```

### Selective Field Extraction

```python
from scrape_tools import WebExtractor

extractor = WebExtractor(url="https://example.com/product")

# Extract only specific article fields
extractor.extract_article(fields=["title", "author", "publication_date"])

# Extract only specific social fields
extractor.extract_socials(fields=["og:title", "twitter:card"])

# Extract only specific product fields
extractor.extract_product(fields=["price", "currency", "rating"])

# Extract only specific video fields
extractor.extract_video(fields=["video_duration", "video_release_date"])

result = extractor.run()
```

### HTTP Client Configuration

```python
from scrape_tools import WebExtractor

extractor = WebExtractor(url="https://example.com")

# Set custom timeout (in seconds)
extractor.set_timeout(60)

# Set custom user agent
extractor.set_user_agent("MyBot/1.0")

# Enable random user agent rotation
extractor.set_random_user_agent(enabled=True)

# Add custom headers
extractor.add_header("Authorization", "Bearer token123")
extractor.add_header("Accept", "application/json")

# Or set multiple headers at once
extractor.set_headers({
    "Authorization": "Bearer token123",
    "Accept": "application/json",
    "X-Custom-Header": "value"
})

extractor.extract_text()
result = extractor.run()
```

### Robots.txt Checking

```python
from scrape_tools import WebExtractor

# Enable robots.txt checking with in-memory cache
extractor = WebExtractor(url="https://example.com/page")
extractor.enable_robots_check()

# Check if URL is allowed before scraping
if extractor.check_robots_allowed():
    extractor.extract_text()
    result = extractor.run()
else:
    print("URL is disallowed by robots.txt")

# With Redis caching (for distributed systems)
extractor = WebExtractor(url="https://example.com/page")
extractor.enable_robots_check_with_redis("redis://localhost:6379")
extractor.set_robots_redis_ttl(3600)  # Cache for 1 hour

# Manually set robots.txt content
extractor.set_robots_txt("User-agent: *\nDisallow: /private/")

# Clear cache
extractor.clear_robots_cache()  # Clear in-memory cache
extractor.remove_robots_from_redis()  # Remove from Redis

# Use context manager for automatic cleanup
with WebExtractor(url="https://example.com") as extractor:
    extractor.enable_robots_check()
    extractor.extract_text()
    result = extractor.run()
    # Cache is automatically cleared when exiting
```

### Async Usage for Batch Processing

```python
import asyncio
from scrape_tools import AsyncWebExtractor, batch_extract

# Single URL with async
async def single_extraction():
    extractor = AsyncWebExtractor(url="https://example.com/article")
    extractor.extract_text(language_detection=True)
    extractor.extract_article()
    extractor.extract_socials()
    result = await extractor.run()
    print(result.article.get("title"))
    print(result.socials)

# Batch processing - extract from multiple URLs concurrently
async def batch_extraction():
    urls = [
        "https://example.com/article1",
        "https://example.com/article2",
        "https://example.com/article3",
    ]
    
    results = await batch_extract(
        urls,
        extract_text=True,
        language_detection=True,
        extract_links_all=True,
        extract_socials_all=True,
        extract_article_all=True,
        extract_product_all=True,
        extract_video_all=True,
        max_concurrent=10  # Process up to 10 URLs concurrently
    )
    
    for result in results:
        print(f"Title: {result.article.get('title')}")
        print(f"Author: {result.article.get('author')}")

# Run async code
asyncio.run(batch_extraction())
```

## Supported Fields

### Article Metadata Fields

Extract article information using `extract_article()`:

- `title` - Article title (from og:title, twitter:title, JSON-LD, or <title> tag)
- `author` - Article author (from article:author, meta author, or schema.org)
- `description` - Article description
- `publication_date` - Publication date with confidence scores
- `modified_date` - Last modified date
- `article_section` - Article section/category
- `article_tag` - Article tags
- `article_author` - Article author (Open Graph)
- `article_published_time` - Published time (ISO 8601)
- `article_modified_time` - Modified time (ISO 8601)
- `article_expiration_time` - Expiration time (ISO 8601)
- `categories` - Categories/tags (from article:tag, keywords, or JSON-LD)

**Aliases supported:**
- `pub_date` → `publication_date`
- `pub_date_time` → `article_published_time`
- `modified_time` → `article_modified_time`
- `expiration_time` → `article_expiration_time`
- `section` → `article_section`
- `tag` or `tags` → `article_tag`
- `category` → `categories`

### Social Metadata Fields

Extract social metadata using `extract_socials()`:

**Twitter Cards:**
- `twitter_card` - Twitter card type
- `twitter_site` - Twitter site handle
- `twitter_creator` - Twitter creator handle
- `twitter_title` - Twitter title
- `twitter_description` - Twitter description
- `twitter_image` - Twitter image URL

**Open Graph:**
- `og_url` - Open Graph URL
- `og_type` - Open Graph type
- `og_title` - Open Graph title
- `og_description` - Open Graph description
- `og_image` - Open Graph image URL
- `og_image_width` - Image width
- `og_image_height` - Image height
- `og_image_alt` - Image alt text
- `og_site_name` - Site name
- `og_locale` - Language/locale

### Product Metadata Fields

Extract product information using `extract_product()`:

**Basic Information:**
- `product_title` - Product title
- `product_description` - Product description
- `product_brand` - Product brand
- `product_category` - Product category
- `product_sku` - SKU (Stock Keeping Unit)
- `product_mpn` - MPN (Manufacturer Part Number)
- `product_image` - Product image URL

**Pricing:**
- `product_price` - Product price
- `product_currency` - Currency code
- `product_availability` - Availability status
- `product_original_price` - Original price (before discount)

**Reviews:**
- `product_rating` - Average rating
- `product_review_count` - Number of reviews
- `product_best_rating` - Best possible rating
- `product_worst_rating` - Worst possible rating

**Aliases supported:**
- `title` → `product_title`
- `description` → `product_description`
- `price` → `product_price`
- `brand` → `product_brand`
- `category` → `product_category`
- `sku` → `product_sku`
- `mpn` → `product_mpn`
- `image` → `product_image`
- `currency` → `product_currency`
- `availability` → `product_availability`
- `original_price` → `product_original_price`
- `rating` → `product_rating`
- `review_count` → `product_review_count`
- `best_rating` → `product_best_rating`
- `worst_rating` → `product_worst_rating`

### Video/Book Metadata Fields

Extract video and book metadata using `extract_video()`:

**Video:**
- `video_duration` - Video duration
- `video_release_date` - Release date
- `video_tag` - Video tags
- `video_actor` - Video actors
- `video_director` - Video director
- `video_writer` - Video writer
- `video_series` - Video series name

**Book:**
- `book_author` - Book author
- `book_isbn` - ISBN
- `book_release_date` - Release date
- `book_tag` - Book tags

### Extract All Fields

You can extract all available fields for any category by omitting the `fields` parameter or passing `["all"]`:

```python
# Extract all article fields
extractor.extract_article()  # Extracts all fields
extractor.extract_article(fields=["all"])  # Same as above

# Extract all social fields
extractor.extract_socials()  # Extracts all fields

# Extract all product fields
extractor.extract_product()  # Extracts all fields

# Extract all video/book fields
extractor.extract_video()  # Extracts all fields
```

## API Reference

### WebExtractor

Synchronous extractor for single URL processing.

#### `__init__(url: str, html: Optional[str] = None)`
Initialize the extractor with a URL. Optionally provide HTML content directly.

#### `extract_text(language_detection: bool = False) -> None`
Enable text extraction from the page.
- `language_detection`: Whether to detect the language of the extracted text

#### `extract_links(*, internal: bool = False, external: bool = False, all: bool = False) -> None`
Enable link extraction with filtering options.
- `internal`: Extract only internal links (same domain)
- `external`: Extract only external links (different domain)
- `all`: Extract all links (default if no options specified)

#### `extract_socials(fields: Optional[List[str]] = None) -> None`
Enable social metadata extraction.
- `fields`: List of specific fields to extract. If `None`, extracts all fields.

#### `extract_video(fields: Optional[List[str]] = None) -> None`
Enable video/book metadata extraction.
- `fields`: List of specific fields to extract. If `None`, extracts all fields.

#### `extract_product(fields: Optional[List[str]] = None) -> None`
Enable product metadata extraction.
- `fields`: List of specific fields to extract. If `None`, extracts all fields.

#### `extract_article(fields: Optional[List[str]] = None) -> None`
Enable article metadata extraction.
- `fields`: List of specific fields to extract. If `None`, extracts all fields.

#### `set_timeout(timeout_secs: float) -> None`
Set the HTTP request timeout in seconds.

#### `set_user_agent(user_agent: str) -> None`
Set a custom user agent string.

#### `set_random_user_agent(enabled: bool = True) -> None`
Enable or disable random user agent generation.

#### `add_header(name: str, value: str) -> None`
Add a custom HTTP header.

#### `set_headers(headers: Dict[str, str]) -> None`
Set multiple HTTP headers at once, replacing any existing headers.

#### `enable_robots_check() -> None`
Enable robots.txt checking with in-memory cache.

#### `enable_robots_check_with_redis(redis_url: str) -> None`
Enable robots.txt checking with both in-memory and Redis cache.

#### `set_robots_redis_ttl(ttl_secs: int) -> None`
Set the TTL (time-to-live) for robots.txt entries in Redis cache.

#### `set_robots_txt(content: str) -> None`
Set robots.txt content manually.

#### `check_robots_allowed() -> bool`
Check if the current URL is allowed by robots.txt.

#### `remove_robots_from_redis() -> None`
Remove robots.txt from Redis cache for the current domain.

#### `clear_robots_cache() -> None`
Clear the in-memory robots.txt cache.

#### `run() -> ExtractionResult`
Execute the extraction and return results.

### AsyncWebExtractor

Async extractor for better performance with single or multiple URLs. All methods are the same as `WebExtractor`, except:

#### `async run() -> ExtractionResult`
Execute the extraction asynchronously and return results.

### batch_extract()

Convenience function for batch processing multiple URLs.

#### `batch_extract(urls: List[str], extract_text: bool = False, language_detection: bool = False, extract_links_internal: bool = False, extract_links_external: bool = False, extract_links_all: bool = False, extract_socials_all: bool = True, extract_video_all: bool = True, extract_product_all: bool = True, extract_article_all: bool = True, max_concurrent: int = 10) -> List[ExtractionResult]`

Extract data from multiple URLs concurrently.

### ExtractionResult

Result object containing extracted data, organized by category.

#### Properties

- `url`: The scraped URL
- `text`: Extracted text content (if `extract_text()` was called)
- `language`: Detected language code (e.g., "en", "fr") if language detection was enabled
- `language_confidence`: Confidence score (0.0 to 1.0) for language detection
- `links`: Dictionary with grouped links containing:
  - `internal`: List of internal links
  - `external`: List of external links
  - `by_domain`: Dictionary mapping domains to their links
  - `summary`: Dictionary with statistics (total, internal_count, external_count, unique_domains)
- `socials`: Dictionary of extracted social metadata (Twitter Cards and Open Graph)
- `videos`: Dictionary of extracted video/book metadata
- `product`: Dictionary of extracted product metadata
- `article`: Dictionary of extracted article metadata
- `content`: Dictionary with content information (text, text_length)

#### Methods

- `to_dict() -> Dict[str, Any]`: Convert the result to a grouped dictionary organized by extraction category
- `result`: Property that returns the same as `to_dict()`

## Project Structure

```
ferriscope/
├── src/                    # Rust source code
│   ├── lib.rs             # Main library entry point and Python bindings
│   ├── extractor.rs       # Core extraction logic with async support
│   ├── error.rs           # Error handling types
│   ├── types.rs           # Common data structures
│   ├── text_extractor.rs  # Text extraction module
│   ├── link_extractor/    # Link extraction module
│   │   ├── mod.rs
│   │   └── helpers.rs
│   ├── socials_extractor/ # Social metadata extraction
│   │   └── mod.rs
│   ├── videos_extractor/  # Video/book metadata extraction
│   │   └── mod.rs
│   ├── products_extractor/ # Product metadata extraction
│   │   └── mod.rs
│   ├── article_extractor/ # Article metadata extraction
│   │   └── mod.rs
│   ├── dom_index.rs       # DOM indexing for efficient parsing
│   └── robots.rs          # Robots.txt checking with caching
├── python/                # Python package
│   └── scrape_tools/
│       ├── __init__.py    # Package initialization
│       ├── extractor.py   # Synchronous WebExtractor wrapper
│       ├── async_extractor.py  # Async WebExtractor wrapper
│       └── constants.py   # Constants and warnings
├── Cargo.toml            # Rust dependencies
├── pyproject.toml        # Python package configuration
└── README.md             # This file
```

## Performance

- **Async Processing**: Uses async/await for non-blocking I/O operations
- **Concurrent Batch Processing**: Process multiple URLs simultaneously with configurable concurrency limits
- **DOM Index Optimization**: Single-pass HTML parsing with reusable DOM index for efficient extraction
- **Optimized Parsing**: Only parses HTML once and extracts requested fields
- **Memory Efficient**: Doesn't store unnecessary data
- **Caching**: Robots.txt caching (in-memory and Redis) reduces redundant requests

## Optimization

The library is optimized to only perform the operations you request. For example:
- If you don't need links, it won't parse them
- Text extraction and metadata extraction share the same HTML parsing step
- DOM index is built once and reused for all extraction operations
- Async operations allow concurrent processing of multiple URLs
- Robots.txt is cached to avoid repeated fetches

## Error Handling

The library provides comprehensive error handling:
- HTTP errors (timeouts, connection failures)
- Parse errors (invalid HTML, malformed data)
- Invalid URL errors
- Robots.txt disallow errors
- Custom error types for better debugging

## License

MIT
