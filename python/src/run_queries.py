import json

from fastapi import FastAPI, HTTPException, Body
from pydantic import BaseModel
import uvicorn
from fastapi.middleware.cors import CORSMiddleware

from query import SearchEngine

#! Set up basic FastAPI backend, for communicating with the frontend
app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

#! Create our search engine
PHONEBOOK_PATH = "phonebook.json"
searcher = None
with open(PHONEBOOK_PATH, 'r') as file:
    data = json.load(file)
    searcher = SearchEngine()

#! Specify the search request type
class SearchRequest(BaseModel):
    query: str
    search_type: str = "name"  # Default search type

#! The API route that the frontend requests
@app.post("/search")
async def search(request: SearchRequest = Body(...)):
    try:
        #! Set and search for the incoming query
        searcher.set_query(request.query)
        results = searcher.search()
        #! Format the results for the frontend
        formatted_results = [{"url": url, "content": content}
                             for url, content in results]
        #! Return the top 5 results, and the amount of time it took to search
        return {
            "results": formatted_results[:5],
            "time": searcher.get_time()
        }
    #! If an error occured, return a 500 server error
    except Exception as e:
        print(e)
        raise HTTPException(status_code=500, detail=str(e))

#! Running the server locally
if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=3000)


"""

#! Old code for running the search engine locally, through the terminal

from query import SearchEngine
import json
import os

SRC_DIR = os.path.dirname(os.path.abspath(__file__))
PHONEBOOK_PATH = os.path.join(SRC_DIR, "..", "phonebook.json")

def main():
    with open(PHONEBOOK_PATH, "r") as file:
        data = json.load(file)
        searcher = SearchEngine()
        while True:
            searcher.get_query()
            searcher.search()

if __name__ == "__main__":
    main()

"""