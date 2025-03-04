# create the inverted index along with the given tokens
import posting
import tokenizer
import os
import json

SRC_DIR = os.path.dirname(os.path.abspath(__file__))
PHONEBOOK_PATH = os.path.join(SRC_DIR, "phonebook.json")

class InvertedIndex:
    def __init__(self, current_doc_id: int = 1):
        self.current_doc_id: int = current_doc_id
        self.dictionary: posting.Dictionary = posting.Dictionary()
        self.tokenizer: tokenizer.Tokenizer = tokenizer.Tokenizer()
        self.phonebook = self.load_phonebook()

    def load_phonebook(self):
        """Loads the phonebook.json if it exists; otherwise, returns an empty dictionary."""
        if os.path.exists(PHONEBOOK_PATH):
            with open(PHONEBOOK_PATH, "r", encoding="utf-8") as f:
                return json.load(f)
        return {}

    def save_phonebook(self):
        """Writes the phonebook dictionary to a JSON file."""
        with open(PHONEBOOK_PATH, "w", encoding="utf-8") as f:
            json.dump(self.phonebook, f, indent=4)

    def add_document(self, text: str):
        """Adds a document to the inverted index and updates the phonebook correctly."""
        doc_id = self.current_doc_id  # Store the current doc ID before processing
        tokens = self.tokenizer.tokenize(text)
        
        for token in tokens:
            self.dictionary.update_frequency(token, doc_id, 1)

        # Ensure the correct doc_id matches the document text in the phonebook
        self.phonebook[str(doc_id)] = text
        self.save_phonebook()

        self.current_doc_id += 1  # Increment doc_id only after storing