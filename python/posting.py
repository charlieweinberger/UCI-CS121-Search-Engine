# A signle posting, which points to the next as well as the document it is associated with and the frequency of the term in the document

from typing import Self
import heapq


class SinglePosting:
    def __init__(self, doc_id: int, frequency: int):
        self.doc_id = doc_id
        self.frequency = frequency

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
        self.postings = []
        self.size = 0
        # ? for later perhaps adding a skip index for each posting list
        self.skip_index = []
        # ? We could perhaps remove this since the dictionary manages this and we want to save every single bit possible
        self.word = word

    def add_posting(self, doc_id: int, frequency: int):
        self.size += 1
        new_posting = SinglePosting(doc_id, frequency)
        # using a minheap to maintain the order of the postings,
        # the nice thing about heapq, it is a min heap by default and keeps the data structure as a list, so we can index it and use it as a list
        heapq.heappush(self.postings, new_posting)
        print(f"Added posting: {new_posting}")
        print(f"Postings: {self.postings}")
        print(type(self.postings))
        print(self.postings)

    def update_frequency(self, doc_id: int, frequency_increment: int):
        for posting in self.postings:
            if posting.doc_id == doc_id:
                posting.update_frequency(frequency_increment)
                return
        # ! This adds the posting if not there, might cause confusing behaviour
        self.add_posting(doc_id, frequency_increment)

    def __str__(self):
        return f"word: {self.word}, postings: {self.postings}"

    def __len__(self):
        return self.size


class Dictionary:
    def __init__(self):
        self.dictionary = {}

    def add_posting(self, word: str, doc_id: int, frequency: int):
        # if word not in self.dictionary:
        # add the word to the dictionary with the default value of a Posting with the word and empty list of postings

        self.dictionary.get(word, Postings(word)).add_posting(doc_id, frequency)
        self.dictionary[word].add_posting(doc_id, frequency)

    def update_frequency(self, word: str, doc_id: int, frequency_increment: int):
        # ! This adds the word if not there, might cause confusing behaviour
        self.dictionary.get(word, Postings(word)).update_frequency(doc_id, frequency_increment)
        self.dictionary[word].update_frequency(doc_id, frequency_increment)

    def __str__(self):
        return str(self.dictionary)
