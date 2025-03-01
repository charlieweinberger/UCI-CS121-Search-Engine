import time
import os
from collections import defaultdict
from tokenizer import Tokenizer

OUTPUT_DIR = "./merged_indexes"

# ! Do we have a function for this? idk
def get_postings(token):
    """
    Get the postings list for a given token from the inverted index.
    """
    first_char = token[0].lower()
    file_path = os.path.join(OUTPUT_DIR, f"{first_char}.txt")
    postings = {}
    if os.path.exists(file_path):
        with open(file_path, 'r', encoding='utf-8') as f:
            for line in f:
                parts = line.strip().split()
                if parts[0] == token:
                    for posting in parts[1:]:
                        doc_id, freq = map(int, posting.split(':'))
                        postings[doc_id] = freq
    return postings

class Candidate:
    def __init__(self, doc_id):
        self.doc_id = doc_id
        self.tokens_matched = {}
    
    def update_score(self, token, frequency):
        self.tokens_matched[token] = frequency
    
    def has_all_tokens(self, query_tokens):
        return all(token in self.tokens_matched for token in query_tokens)
    # def calculate_score(self, total_tokens):
    #     for token in self.tokens:
    #         if token in total_tokens:
    #             self.score += total_tokens[token]

class SearchEngine:
    def __init__(self):
        self.query = ""
        self.tokens = []
    
    def get_query(self):
        self.query = input("Enter your query: ").strip()
        tokenizer = Tokenizer()
        self.tokens = tokenizer.tokenize(self.query)
    
    def search(self):
        start_time = time.time()
        print(f"Searching...: {self.query}")
        print(f"Tokens: {self.tokens}")
        candidates = {}
        
        for token in self.tokens:
            # get the postings list for the token
            postings = get_postings(token)   
            for doc_id, frequency in postings.items():
                if doc_id not in candidates:
                    candidates[doc_id] = Candidate(doc_id)
                candidates[doc_id].update_tokens(token, frequency)
        valid_candidates = get_valid_candidates(candidates, self.tokens)
        
        print(f"Found {len(valid_candidates)}")
        if valid_candidates:
            print("Matching Documents:")
            for i, candidate in enumerate(valid_candidates):
                print(f"{i+1}. Document ID: {candidate.doc_id}, Score: {candidate.score}")
        print(f"Search completed in {time.time() - start_time:.2f} seconds.")
def get_valid_candidates(candidates, query_tokens):
    valid_candidates = []
    for c in candidates.values():
        if c.has_all_tokens(query_tokens):
            valid_candidates.append(c)
    return valid_candidates
                