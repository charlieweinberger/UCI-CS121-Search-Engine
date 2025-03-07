import hashlib
WORD_NGRAM_SIZE = 2
WORD_SIMILARITY_THRESHOLD = 0.75
SIMHASH_BIT_SIZE = 64


# * Understanding from https://zlib.net/crc_v3.txt, https://en.wikipedia.org/wiki/Cyclic_redundancy_check#CRCs_and_data_integrity
def python_crc32(data: str):
    # we use the CRC-32 polynomial for the CRC calculation
    # crc = Width of the CRC (actual position of the highest bit)
    crc32_polynomial = 0x04c11db7
    # crc32_polynomial = 0x04c11db7: 0000 0100 1100 0001 0001 1101 1011 0111
    data = data.encode("utf-8")
    if len(data) < 1024*5:
        print("Warning: Data is too small for CRC32 calculation")
    # crc is masked to 32 bits just incase
    crc = 0x00000000
    # Divide the data by the polynomial and the remainder is the CRC
    for byte in data:
        # Each byte is XORed with the CRC
        crc ^= byte
        for _ in range(8):
            # The polynomial is XORed with the remainder if the highest bit of the remainder is 1 else if 0, the remainder is shifted right by 1
            crc = (crc >> 1) ^ (crc32_polynomial if crc & 1 else 0)
    return crc

def generate_word_ngrams(text: str, n: int) -> set[str]:
    words = text.split()
    if len(words) < n:
        return set()
    return {' '.join(words[i:i+n]) for i in range(len(words - n + 1))}


class SimilarityDeterctor:
    """
    Detect similar pages during indexing based on content
    """
    def __init__(self):
        self.seen_checksums = set()
        self.page_simhashes = {}
        
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
        checksum = python_crc32(clean_text)
        if checksum in self.seen_checksums:
            return True, "exact_duplicate"
        simhash_value = simhash(clean_text)
        self.seen_checksums.add(checksum)
        return False, "unique"
