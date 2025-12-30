# Changelog

## Version 0.2.0 - Major Improvements

### Project Structure
- **Modular Architecture**: Refactored Rust codebase into separate modules:
  - `error.rs` - Comprehensive error handling types
  - `types.rs` - Common data structures
  - `extractor.rs` - Core extraction logic
  - `text_extractor.rs` - Text extraction module
  - `link_extractor.rs` - Link extraction module
  - `metadata_extractor.rs` - Comprehensive metadata extraction (40+ fields)

### Performance Improvements
- **Async Support**: Converted from blocking to async HTTP client using `reqwest` async API
- **Tokio Runtime**: Added tokio runtime for async operations
- **Concurrent Processing**: Added `batch_extract()` function for concurrent processing of multiple URLs
- **Thread Pool Execution**: Python async wrapper uses thread pool to avoid blocking event loop

### Metadata Extraction Enhancements
Added support for 40+ metadata fields:

#### Basic Metadata
- `title`, `description`, `author`, `image`, `site_name`, `type`, `locale`, `keywords`, `robots`, `canonical_url`

#### Dates
- `publication_date`, `modified_date`

#### Open Graph
- `og_url`, `og_type`, `og_image_width`, `og_image_height`, `og_image_alt`

#### Twitter Cards
- `twitter_card`, `twitter_site`, `twitter_creator`, `twitter_title`, `twitter_description`, `twitter_image`

#### Article Metadata
- `article_section`, `article_tag`, `article_author`, `article_published_time`, `article_modified_time`, `article_expiration_time`

#### Video Metadata
- `video_duration`, `video_release_date`, `video_tag`

#### Book Metadata
- `book_author`, `book_isbn`, `book_release_date`

#### Profile Metadata
- `profile_first_name`, `profile_last_name`, `profile_username`

#### Structured Data
- `schema_json` - Full JSON-LD extraction
- Schema.org property extraction via microdata and JSON-LD parsing

### Python API Enhancements
- **AsyncWebExtractor**: New async class for better performance
- **batch_extract()**: Convenience function for batch processing
- **Better Error Handling**: Improved error messages and exception handling
- **Updated Documentation**: Comprehensive README with examples

### Dependencies
- Updated `reqwest` to use async API (removed blocking feature)
- Added `tokio` for async runtime
- Added `regex` for schema.org JSON-LD parsing
- Added `once_cell` for lazy static initialization (removed in final version)

### Backward Compatibility
- Synchronous `WebExtractor.run()` method still available
- All existing code continues to work without changes
- Async methods are optional additions

### Code Quality
- Proper error handling with custom error types
- Better code organization and separation of concerns
- Improved documentation and examples
- No breaking changes to existing API

## Version 0.1.0 - Initial Release

- Basic web scraping functionality
- Text extraction
- Basic metadata extraction (title, author, publication_date, categories)
- Link extraction
- Language detection
- Synchronous processing only

