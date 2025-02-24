# save the inverted index to a file
import os
from indexer import InvertedIndex
import time

total_documents = 0
total_tokens = set()
total_size = 0
current = time.time()


def save_inverted_index(inverted_index: InvertedIndex, output_path: str):
    """
    Save inverted index to a file along with statistics
    Args:
        inverted_index: InvertedIndex object to save
        output_path: Path where to save the index
    """
    # Convert index to serializable format
    dictionary = inverted_index.dictionary.dictionary
    # Subtract 1 since current_doc_id is next available ID
    num_documents = inverted_index.current_doc_id
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        for token in inverted_index.dictionary.keys:
            postings = dictionary[token].postings
            # Create a string for the postings list: "docID:termFreq"
            postings_str = ",".join(
                f"{posting.doc_id}:{posting.frequency}" for posting in postings)
            # Write a line: "token docID1:termFreq1 docID2:termFreq2 ..."
            f.write(f"{token} {postings_str}\n")
    # Calculate and add file size in KB
    file_size_kb = os.path.getsize(output_path) / 1024
    global total_documents, total_tokens, total_size, current
    total_documents = num_documents
    total_tokens.update(inverted_index.dictionary.keys)
    total_size += file_size_kb
    # Print statistics
    print("Index saved successfully:")
    print(f"Number of indexed documents: {total_documents}")
    print(f"Number of unique tokens: {len(total_tokens)}")
    print(f"Index size on disk: {total_size} KB")
    print(time.time() - current)
