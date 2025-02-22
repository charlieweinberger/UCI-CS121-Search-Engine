# tokenizer to split the input text into tokens
# ! also stemmer and normalizer and remove stop words & punctuations
# * NTLK: https://www.nltk.org/
# * classification, tokenization, stemming, tagging, parsing, and semantic reasoning, wrappers for industrial-strength NLP libraries
from typing import List
import nltk
from nltk.corpus import stopwords
from nltk.stem import PorterStemmer
from nltk.tokenize import TweetTokenizer as NLTKTokenizer

class Tokenizer:
    def __init__(self):
        self.tokenizer = NLTKTokenizer()
        self.stop_words = set(stopwords.words('english'))
        self.porter_stemmer = PorterStemmer()

    def tokenize(self, text: str) -> List[str]:
        # split the input text into tokens
        tokens = self.tokenizer.tokenize(text.lower())
        # remove stop words
        tokens = [self.porter_stemmer.stem(word, to_lowercase=True) for word in tokens if word.lower()
                  not in self.stop_words]
        
        #  ? We could potentially down the route achieve a porter stemmer using something like a dictionary we get online and seeing hte stemmed values
        #  ? and then if the dict fails use the porter stemmer available in nltk
        tokens = list(filter(lambda x: x.isalnum(), tokens))
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