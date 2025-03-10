import time
import os
from collections import defaultdict
from typing import List
from tokenizer import Tokenizer
import math
import json

SRC_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_DIR = os.path.join(SRC_DIR, "merged_indexes")
PHONEBOOK = os.path.join(SRC_DIR, "phonebook.json")
TOTAL_DOCUMENT_COUNT = 40157  # Replace with the actual total document count

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
    tf = (math.log2(term_freq) + 1.0 )
    idf = math.log2(TOTAL_DOCUMENT_COUNT / posting_length)
    return tf * idf

class Candidate:
    """
    A candidate document for a query.
    """

    def __init__(self, doc_id):
        self.doc_id = doc_id
        self.tokens_matched = {}   
        self.total_score = 0.0   
        self.tf_idf = 0.0 
        self.info = {}  # frequency, posting_length, token
    
    def set_info(self, frequency, posting_length, token):
        self.info[token] = (frequency, posting_length)
    
    def update_score(self, token, score):
        self.tokens_matched[token] = score
        self.total_score += score

    def has_all_tokens(self, query_tokens):
        return all(token in self.tokens_matched for token in query_tokens)
    
    def get_total_score(self):
        return self.total_score
        
    def get_total_tf_idf(self):
        return self.tf_idf
    
    # def compute_tf_idf(self):
    #     for token in self.info.keys():
    #         self.compute_tf_idf_for_token(token)
    #         term_freq, posting_length = self.info[token]
    #         total_tokens_in_a_doc = get_num_tokens_of_a_doc(self.doc_id)
    #         tf = (math.log2(term_freq/total_tokens_in_a_doc) + 1.0)
    #         idf = math.log2(TOTAL_DOCUMENT_COUNT / posting_length)
    #         self.tf_idf += tf * idf
    #     return self.tf_idf        

    


class SearchEngine:
    """
    A search engine that takes a query and returns the matching documents.
    """

    def __init__(self):
        self.query = ""
        self.tokens = []
        self.time = 0

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
            postings_len = len(postings)
            for cur_doc, frequency in postings.items():
                doc_id = cur_doc + 1
                if doc_id not in candidates:
                    candidates[doc_id] = Candidate(doc_id)
                
                score = scoring_tf_idf(frequency, postings_len)
                candidates[doc_id].update_score(token, score)
                # candidates[doc_id].set_info(frequency, postings_len, token)
        # Filter candidates to only those that match all query tokens
        valid_candidates = get_valid_candidates(candidates, self.tokens)
        finished_time = int((time.time() - start_time) * 1000)
        self.time = finished_time          
        valid_candidates.sort(key=lambda c: c.get_total_score(), reverse=True)
        # valid_candidates.sort(key=lambda c: c.get_tf_idf(), reverse=True)

        # Take only top 5 results if available
        valid_candidates = valid_candidates[:5]
        doc_ids = []
        if valid_candidates:
            print("Matching Documents:")
            for i, candidate in enumerate(valid_candidates):
                print(
                    f"{i+1}. Document ID: {candidate.doc_id}, Score: {candidate.tokens_matched[token]}")
                doc_ids.append(candidate.doc_id)
        else:
            return [] 
        print(f"Search completed in {finished_time} ms.")
        
        return get_doc_info(doc_ids)
        
    def get_time(self):
        return self.time

def get_valid_candidates(candidates, query_tokens):
    valid_candidates = []
    for c in candidates.values():
        if c.has_all_tokens(query_tokens):
            valid_candidates.append(c)
    return valid_candidates

def get_doc_info(doc_ids):
    """
    Get the document information for a given document ID.
    Returns an array of a docID's URL and content
    First element has the highest score. 
    """
    with open(PHONEBOOK, 'r') as f:
        phonebook = json.load(f)
    
    result = []
    
    for doc_id in doc_ids:
        file_path = phonebook[str(doc_id)]
        print(f"File path: {file_path}")
        with open(file_path, 'r') as doc_file:
            doc_data = json.load(doc_file)
            result.append((doc_data["url"], doc_data["content"]))
    
    return result

# def get_num_tokens_of_a_doc(doc_id):
#     """
#     Get the document information for a given document ID.
#     Returns an array of a docID's URL and content
#     First element has the highest score. 
#     """
#     with open(PHONEBOOK, 'r') as f:
#         phonebook = json.load(f)
    
#     tokenizer = Tokenizer()    
#     file_path = phonebook[str(doc_id)]
#     print(f"File path: {file_path}")
#     with open(file_path, 'r') as doc_file:
#         doc_data = json.load(doc_file)
#         return len(tokenizer.tokenize(doc_data["content"]))
                
