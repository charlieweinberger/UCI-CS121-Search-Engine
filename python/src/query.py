import time
import os
from collections import defaultdict
from typing import List
from tokenizer import Tokenizer
import math

SRC_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_DIR = os.path.join(SRC_DIR, "merged_indexes")

TOTAL_DOCUMENT_COUNT = 10000  # Replace with the actual total document count

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
                        doc_id, freq = map(
                            int, posting.removesuffix(',').split(':'))
                        postings[doc_id] = freq
                    break  # stop after finding the token
    return postings

def scoring_tf_idf(term_freq: int, posting_length: int) -> float:
    """
    Calculate the TF-IDF score for a term in a document.
    """
    tf = math.log2(term_freq) + 1.0
    idf = math.log2(TOTAL_DOCUMENT_COUNT / posting_length)
    return tf * idf

class Candidate:
    """
    A candidate document for a query.
    """

    def __init__(self, doc_id):
        self.doc_id = doc_id
        self.tokens_matched = {}

    def update_score(self, token, score):
        self.tokens_matched[token] = score

    def has_all_tokens(self, query_tokens):
        return all(token in self.tokens_matched for token in query_tokens)
    


class SearchEngine:
    """
    A search engine that takes a query and returns the matching documents.
    """

    def __init__(self):
        self.query = ""
        self.tokens = []

    def get_query(self):
        """
        Get the query from the user through the console.
        """
        self.query = input("Enter your query: ").strip()
        tokenizer = Tokenizer()
        self.tokens = tokenizer.tokenize(self.query)

    def set_query(self, query):
        """
        Set the query from the user.
        """
        self.query = query
        tokenizer = Tokenizer()
        self.tokens = tokenizer.tokenize(self.query)

    def search(self) -> List[str]:
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
                score = scoring_tf_idf(frequency, len(postings))
                candidates[doc_id].update_score(token, score)
        # Filter candidates to only those that match all query tokens
        valid_candidates = get_valid_candidates(candidates, self.tokens)

        # Sort candidates by summing frequencies across all matched tokens
        valid_candidates.sort(key=lambda c: sum(
            c.tokens_matched.values()), reverse=True)
        # Take only top 5 results if available
        valid_candidates = valid_candidates[:5]
        finished_time = time.time() - start_time
        if valid_candidates:
            print("Matching Documents:")
            for i, candidate in enumerate(valid_candidates):
                print(
                    f"{i+1}. Document ID: {candidate.doc_id}, Score: {candidate.tokens_matched[token]}")
        else:
            return [] 
        
        print(f"Search completed in {finished_time} seconds.")
        return [c.doc_id for c in valid_candidates]


def get_valid_candidates(candidates, query_tokens):
    valid_candidates = []
    for c in candidates.values():
        if c.has_all_tokens(query_tokens):
            valid_candidates.append(c)
    return valid_candidates
