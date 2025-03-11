from urllib.parse import urlparse


def is_valid_page(url: str, content: str) -> bool:
    # Parse URL to validate scheme and check for fragments (anchors)
    try:
        parsed_url = urlparse(url)
    except Exception:
        return False

    # Exclude pages with anchors
    if parsed_url.fragment:
        return False
    # Check file extension
    path = parsed_url.path.lower()
    valid_extensions = {".txt", ".html", ".htm",
                        ".md", ".xml", ".xhtml", ".xhtm", ".xht"}

    if path:
        has_valid_extension = any(path.endswith(ext)
                                  for ext in valid_extensions) or '.' not in path
        if not has_valid_extension:
            return False

    # Verify content has HTML-like structure
    content_lower = content.lower()
    if not any(tag in content_lower for tag in ("html", "body", "meta", "<p", "txt", "text")):
        return False

    return True
