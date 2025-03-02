from query import SearchEngine
import json

PHONEBOOK_PATH = "phonebook.json"

def main():
    with open(PHONEBOOK_PATH, 'r') as file:
        data = json.load(file)
        searcher = SearchEngine()
        while(True):
            searcher.get_query()
            searcher.search()

if __name__ == "__main__":
    main()