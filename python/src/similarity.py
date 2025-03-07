import hashlib

class SimilarityDeterctor:
    """
    Detect similar pages during indexing based on content
    """
    def __init__(self):
        self.seen_checksums = set()
        self.page_simhashes = {}\
        
    def clean_text(self, text:str) -> str:
        text = ' '.join(text.strip().split())
        return text.lower()
    
    def calculuate_similarity(self, ngrams1: set, ngrams2: set) -> float:
        if not ngrams1 or not ngrams2:
            return 0.0
        intersection = len(ngrams1 & ngrams2)
        union = len(ngrams1|ngrams2)
        return intersection / union if union > 0 else 0.0
    
    #not sure what we passin in to here
    def is_duplicate_or_similar(self, path: str, text:str) -> tuple[bool, str]:
        clean_text = self.clean_text(text)
        return False, "unique"
