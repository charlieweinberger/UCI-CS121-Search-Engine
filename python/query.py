import time
import os
from collections import defaultdict
from tokenizer import Tokenizer

class Candidate:
    def __init__(self, doc_id):
        self.doc_id = doc_id
        self.score = 0.0
        self.tokens = defaultdict(int)
    
    def update_score(self, token, frequency):
        self.tokens[token] += frequency
    
    def calculate_score(self, total_tokens):
        for token in self.tokens:
            if token in total_tokens:
                self.score += total_tokens[token]

class SearchEngine:
    def __init__(self):
        self.query = ""
        self.tokens = []
    
    def get_query(self):
        self.query = input("Enter your query: ")
        tokenizer = Tokenizer()
        self.tokens = tokenizer.tokenize(self.query)
    
    def search(self, inverted_index):
        start_time = time.time()
        print(f"Searching...: {self.query}")
        print(f"Tokens: {self.tokens}")
        candidates = {}
        
        for token in self.tokens:
            if token in inverted_index:
                postings = inverted_index[token]
                for doc_id, term_freq in postings.items():
                    if doc_id not in candidates:
                        candidates[doc_id] = Candidate(doc_id)
                    candidates[doc_id].update_score(token, term_freq)
        for candidate in candidates.values():
            candidate.calculate_score(inverted_index.total_tokens)
        results = sorted(candidates.values(), key=lambda x: x.score, reverse=True)
        
        print(f"Found {len(results)} matching documents in {time.time() - start_time:.2f} seconds")
        if results:
            print("Top 10 results:")
            for candidate in results[:10]:
                print(f"Document ID: {candidate.doc_id}, Score: {candidate.score:.2f}")

def get_valid_candidates(candidates, query_tokens):
    valid_candidates = []
    for candidate in candidates:
        if all(token in candidate.tokens for token in query_tokens):
            valid_candidates.append(candidate)
    return valid_candidates
                