# ! also stemmer and normalizer and remove stop words & punctuations
# * NTLK: https://www.nltk.org/
# * classification, tokenization, stemming, tagging, parsing, and semantic reasoning, wrappers for industrial-strength NLP libraries

import nltk # type: ignore

class Tokenizer:
    def __init__(self):
        self.porter_stemmer = nltk.stem.PorterStemmer()

    def tokenize(self, text: str) -> list[str]:

        #! Split the input text into tokens, using Porter Stemmer.
        
        #! Get rid of prefixes and suffixes, so that each word can
        #! be a token and be the same token as other similar words.

        #! Note: We can't convert tokens into a set, since we would
        #! lose count of how many times each token shows up. Therefore
        #! we are doing tokenization during both indexing and querying.

        #! Tokenize the text with regex, so that we only get alphanumeric
        tokens = nltk.regexp_tokenize(text, r'[A-Za-z0-9]+')
        #! Return a list of stemmed tokens
        return [self.porter_stemmer.stem(token, to_lowercase=True) for token in tokens]

#! Testing the tokenizer
if __name__ == "__main__":
    tokenizer = Tokenizer()
    assert tokenizer.tokenize("This is a simple text.") == [
        "thi", "is", "a", "simpl", "text"]
    assert tokenizer.tokenize("THIS IS  A simpler TEXT WITH MORE WORDS.") == [
        "thi", "is", "a", "simpler", "text", "with", "more", "word"]
    assert tokenizer.tokenize("iftekhar ahmed") == [
        "iftekhar", "ahm"]
