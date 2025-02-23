# A signle posting, which points to the next as well as the document it is associated with and the frequency of the term in the document

from typing import Self, Dict, List, Tuple
import heapq

class SinglePosting:
    def __init__(self, doc_id: int, frequency: int):
        self.doc_id: int = doc_id
        self.frequency: int = frequency

    def __str__(self):
        return f"doc_id: {self.doc_id}, frequency: {self.frequency}"

    def update_frequency(self, frequency: int):
        self.frequency += frequency

    def __lt__(self, other: Self):
        return self.doc_id < other.doc_id

    def __eq__(self, other: Self):
        return self.doc_id == other.doc_id

    def __gt__(self, other: Self):
        return self.doc_id > other.doc_id


class Postings:
    def __init__(self, word: str):
        self.postings: List[SinglePosting] = []
        self.size: int = 0
        # ? for later perhaps adding a skip index for each posting list
        self.skip_index: Tuple[int, int] = []  # (doc_id, index)
        # ? We could perhaps remove this since the dictionary manages this and we want to save every single bit possible
        self.word: str = word

    def add_posting(self, doc_id: int, frequency: int):
        self.size += 1
        new_posting = SinglePosting(doc_id, frequency)
        # using a minheap to maintain the order of the postings,
        # the nice thing about heapq, it is a min heap by default and keeps the data structure as a list, so we can index it and use it as a list
        heapq.heappush(self.postings, new_posting)

    def update_frequency(self, doc_id: int, frequency_increment: int):
        left, right = 0, len(self.postings) - 1

        while left <= right:
            mid = (left + right) // 2
            if self.postings[mid].doc_id == doc_id:
                self.postings[mid].update_frequency(frequency_increment)
                return
            elif self.postings[mid].doc_id < doc_id:
                left = mid + 1
            else:
                right = mid - 1

        # If we get here, the doc_id wasn't found
        # ! This adds the posting if not there, might cause confusing behaviour
        self.add_posting(doc_id, frequency_increment)

    def __str__(self):
        return f"word: {self.word}, postings: {self.postings}"

    def __len__(self):
        return self.size


class Dictionary:
    def __init__(self):
        self.dictionary: Dict[str, Postings] = {}
        # ? We want to maintain the keys in a sorted order for faster retrieval
        self.keys: str = []

    def add_posting(self, word: str, doc_id: int, frequency: int):
        # if word not in self.dictionary:
        # add the word to the dictionary with the default value of a Posting with the word and empty list of postings
        if word not in self.dictionary:
            self.dictionary[word] = Postings(word)
            heapq.heappush(self.keys, word)
        self.dictionary[word].add_posting(doc_id, frequency)

    def update_frequency(self, word: str, doc_id: int, frequency_increment: int):
        # ! This adds the word if not there, might cause confusing behaviour
        if word not in self.dictionary:
            self.add_posting(word, doc_id, frequency_increment)
        else:
            self.dictionary[word].update_frequency(doc_id, frequency_increment)

    def add_token(self, token: str, doc_id: int):
        if token not in self.dictionary:
            self.add_posting(token, doc_id, 1)
        else:
            self.update_frequency(token, doc_id, 1)

    def __str__(self):
        return str(self.dictionary)
