# save the inverted index to a file
import os
from indexer import InvertedIndex
import json

def save_inverted_index(inverted_index: InvertedIndex, output_path: str):
    """
    Save inverted index to a file along with statistics
    Args:
        inverted_index: InvertedIndex object to save
        output_path: Path where to save the index
    """
    # Convert index to serializable format
    index_data = {}
    for (token, postings) in inverted_index.dictionary.dictionary.items():
        index_data[token] = [
            {"doc_id": posting.doc_id, "term_freq": posting.frequency}
            for posting in postings.postings
        ]

    num_documents = inverted_index.current_doc_id - 1  # Subtract 1 since current_doc_id is next available ID
    num_unique_tokens = len(inverted_index.dictionary.dictionary)
    # Create output dictionary with statistics
    output = {
        "statistics": {
            "num_documents": num_documents,
            "num_unique_tokens": num_unique_tokens,
        },
        "index": index_data
    }

    # Save to file
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2)

    # Calculate and add file size in KB
    file_size_kb = os.path.getsize(output_path) / 1024

    # Print statistics
    print("Index saved successfully:")
    print(f"Number of indexed documents: {num_documents}")
    print(f"Number of unique tokens: {num_unique_tokens}")
    print(f"Index size on disk: {file_size_kb:.2f} KB")