# tokenizer to split the input text into tokens
# ! also stemmer and normalizer and remove stop words & punctuations
# * NTLK: https://www.nltk.org/
# * classification, tokenization, stemming, tagging, parsing, and semantic reasoning, wrappers for industrial-strength NLP libraries
from typing import List
import nltk
from nltk.corpus import stopwords
from nltk.stem import PorterStemmer


class Tokenizer:
    def __init__(self):
        self.tokenizer = r'[A-Za-z0-9]+'
        self.stop_words = set(stopwords.words('english'))
        self.porter_stemmer = PorterStemmer()

    # We cannot convert tokens int a set ever since we would lose count of how many times it shows up
    def tokenize(self, text: str) -> List[str]:
        # split the input text into tokens
        tokens = nltk.regexp_tokenize(text, self.tokenizer)
        # remove stop words
        tokens = [self.porter_stemmer.stem(word, to_lowercase=True) for word in tokens if word.lower()
                  not in self.stop_words and len(word) > 2]
        return tokens


# ? Testing the tokenizer
if __name__ == "__main__":
    tokenizer = Tokenizer()
    text = "This is a simple text."
    text2 = "THIS IS  A simpler TEXT WITH MORE WORDS."
    tokens = tokenizer.tokenize(text)
    print(tokens)
    tokens = tokenizer.tokenize(text2)
    print(tokens)
