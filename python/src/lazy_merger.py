from io import TextIOWrapper
import os
from typing import List
from posting import Postings
import json

def get_file_for_word(word: str)->str:
    """Chooses the file to write the word to based on the first character of the word."""
    first_char = word[0].lower()
    if first_char.isalpha():
        return os.path.join(OUTPUT_DIR, f"{first_char}.txt")
    elif first_char.isdigit():
        return os.path.join(OUTPUT_DIR, "0-9.txt")
    # should never have a word starting with a special character or space
    
def list_of_needed_files(batches: List[str]) -> List[TextIOWrapper]:
    # * /indexes/batch_{number}.txt
    return [open(os.path.join(BASEPATH, batch), 'r', encoding='utf-8')
            for batch in batches]


def get_smallest_key(postings: List[Postings]) -> str:
    """Get the smallest key from a list of postings."""
    smallest = postings[0].word
    for posting in postings:
        if posting.word < smallest:
            smallest = posting.word
    return smallest


BASEPATH = "./indexes"
FINAL_INDEX = "final_index.txt"
OUTPUT_DIR = "./merged_indexes"
PHONEBOOK_PATH = "phonebook.json"


if __name__ == "__main__":
    # * Create the output directory if it doesn't exist
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    # go through all the files in indexes folder and merge them into one
    batches = []
    # * /indexes/batch_{number}.txt
    for root, _, files in os.walk(BASEPATH):
        batches.extend(files)
    # now make a reader object but dont read the file yet
    readers = list_of_needed_files(batches)
    file_handlers = {}
    # final_file_appender = open(FINAL_INDEX, 'a+', encoding='utf-8')
    
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

        # Get the output file for the smallest word
        output_file = get_file_for_word(smallest)
        # * Write the merged posting to the appropriate file
        if output_file not in file_handlers:
            file_handlers[output_file] = open(output_file, 'a+', encoding='utf-8')
        
        file_handlers[output_file].write(str(new_posting) + "\n")
        
        # Move to next line for relevant readers
        for idx in to_advance:
            lines[idx] = readers[idx].readline()
            if not lines[idx].strip():  # Reader reached end
                readers[idx].close()
                active_indices.remove(idx)
    
    # Close all file handlers
    for handler in file_handlers.values():
        handler.close()
    
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
