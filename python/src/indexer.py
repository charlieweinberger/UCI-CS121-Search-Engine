# create the inverted index along with the given tokens
import posting
import tokenizer
import os


class InvertedIndex:
    def __init__(self, current_doc_id: int = 1):
        self.current_doc_id: int = current_doc_id
        # ? We want both the postings and the dictionary keys to be in the sorted order for faster retrieval, also we cannot expect 
        # ? the hash of every word to be different so many we do something where like each dictionary points to a list of words of that alphabet alone
        self.dictionary: posting.Dictionary = posting.Dictionary()
        self.tokenizer: tokenizer.Tokenizer = tokenizer.Tokenizer()        

    def add_document(self, text: str):
        tokens = self.tokenizer.tokenize(text)
        for token in tokens:
            self.dictionary.update_frequency(token, self.current_doc_id, 1)

        self.current_doc_id += 1
    
    