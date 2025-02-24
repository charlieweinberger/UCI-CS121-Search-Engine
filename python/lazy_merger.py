from io import TextIOWrapper
import os
from typing import List
from posting import Postings
import json


def list_of_needed_files(batches: List[str]) -> List[TextIOWrapper]:
    # * /indexes/batch_{number}.txt
    return [open(os.path.join(BASEPATH, batch), 'r', encoding='utf-8')
            for batch in batches]


def get_smallest_key(postings: List[Postings]) -> str:
    smallest = postings[0].word
    for posting in postings:
        if posting.word < smallest:
            smallest = posting.word
    return smallest


BASEPATH = "./indexes"
FINAL_INDEX = "final_index.txt"

if __name__ == "__main__":
    # go through all the files in indexes folder and merge them into one
    batches = []
    # * /indexes/batch_{number}.txt
    for root, _, files in os.walk(BASEPATH):
        batches.extend(files)
    # now make a reader object but dont read the file yet
    readers = list_of_needed_files(batches)
    final_file_appender = open(FINAL_INDEX, 'a+', encoding='utf-8')

    # from each reader take a line, see the first token(word),
    # compare it to all others, merge the smaller one first,
    # move to the next line and compare and repeat till all files have
    # been read
    lines = [reader.readline() for reader in readers]
    active_indices = list(range(len(lines)))

    while active_indices:
        # Filter out empty lines (end of file)
        current_indices = [i for i in active_indices if lines[i].strip()]
        # if [] we reached end
        if not current_indices:
            break

        # get all valid postings
        postings = [Postings.construct_postings(
            lines[i]) for i in current_indices]
        # get the smallest key
        smallest = get_smallest_key(postings)
        new_posting = Postings(smallest)

        # Track which readers need to move to next line
        to_advance = []
        for idx in current_indices:
            posting = Postings.construct_postings(lines[idx])
            if posting.word == smallest:
                new_posting = new_posting.merge_postings(posting)
                to_advance.append(idx)

        final_file_appender.write(str(new_posting) + "\n")

        # Move to next line for relevant readers
        for idx in to_advance:
            lines[idx] = readers[idx].readline()
            if not lines[idx].strip():  # Reader reached end
                readers[idx].close()
                active_indices.remove(idx)
    final_file_appender.close()

    # also making the phone book of getting the docid to file location
    current_path = "../developer/DEV"
    batches = []
    for root, _, files in os.walk(current_path):
        batches.extend([os.path.join(root, file) for file in files])

    # Create mapping of docid to file location
    docid_to_file = {}
    for i, file_path in enumerate(sorted(batches), 1):
        docid_to_file[i] = file_path

    # Write the mapping to phonebook.json
    with open('phonebook.json', 'w', encoding='utf-8') as f:
        json.dump(docid_to_file, f, indent=2)
