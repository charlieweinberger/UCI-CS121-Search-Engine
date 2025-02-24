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
        # Complexity: O(log n) for push and O(1) for pop
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
        string = f"{self.word} "
        for posting in self.postings:
            string += f"{posting.doc_id}:{posting.frequency}, "
        string = string[:-2]
        return string

    def __len__(self):
        return self.size

    @staticmethod
    def construct_postings(line: str) -> Self:
        # line is of the form "Token docID1:termFreq1, docID2:termFreq2, ..."
        token, postings = line.split(" ", 1)
        new_posting = Postings(token)
        for posting in postings.split(","):
            doc_id, frequency = posting.split(":")
            new_posting.add_posting(int(doc_id), int(frequency))
        return new_posting

    def merge_postings(self, other: Self) -> Self:
        # merge two posting lists
        if self.word != other.word:
            raise ValueError(
                f"Cannot merge postings of different words: {self.word} and {other.word}")
        new_postings = Postings(self.word)
        i, j = 0, 0
        while i < len(self.postings) and j < len(other.postings):
            # doc_id will never be the same in two postings since the batches operate on different documents
            if self.postings[i] < other.postings[j]:
                new_postings.add_posting(
                    self.postings[i].doc_id, self.postings[i].frequency)
                i += 1
            else:
                new_postings.add_posting(
                    other.postings[j].doc_id, other.postings[j].frequency)
                j += 1
        # add the remaining postings if self is bigger
        while i < len(self.postings):
            new_postings.add_posting(
                self.postings[i].doc_id, self.postings[i].frequency)
            i += 1
        # same but other
        while j < len(other.postings):
            new_postings.add_posting(
                other.postings[j].doc_id, other.postings[j].frequency)
            j += 1
        return new_postings


class Dictionary:
    def __init__(self):
        self.dictionary: Dict[str, Postings] = {}
        # ? We want to maintain the keys in a sorted order for faster retrieval
        self.keys: List[str] = []

    def add_posting(self, word: str, doc_id: int, frequency: int):
        if word not in self.dictionary:
            self.dictionary[word] = Postings(word)
            # Insert the word in the correct position to maintain sorted order
            self._insert_key(word)
        self.dictionary[word].add_posting(doc_id, frequency)

    def _insert_key(self, word: str):
        # Binary search to find insertion point
        left, right = 0, len(self.keys)
        while left < right:
            mid = (left + right) // 2
            if self.keys[mid] < word:
                left = mid + 1
            else:
                right = mid
        self.keys.insert(left, word)


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
