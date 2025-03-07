import hashlib

class SimilarityDeterctor:
    """
    Detect similar pages during indexing based on content
    """
    def __init__(self):
        self.seen_checksums = set()
        self.page_simhashes = {}