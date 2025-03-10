import os
from query import SearchEngine
import json
from fastapi import FastAPI, HTTPException, Body
from pydantic import BaseModel
import uvicorn
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

PHONEBOOK_PATH = "phonebook.json"

searcher = None
with open(PHONEBOOK_PATH, 'r') as file:
    data = json.load(file)
    searcher = SearchEngine()


class SearchRequest(BaseModel):
    query: str
    search_type: str = "name"  # Default search type


@app.post("/search")
async def search(request: SearchRequest = Body(...)):
    try:
        searcher.set_query(request.query)
        results = searcher.search()
        formatted_results = [{"url": url, "content": content}
                             for url, content in results]
        return {
            "results": formatted_results[:5],
            "time": searcher.get_time()
        }
    except Exception as e:
        print(e)
        raise HTTPException(status_code=500, detail=str(e))


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=3000)


# from query import SearchEngine
# import json
# import os

# SRC_DIR = os.path.dirname(os.path.abspath(__file__))
# PHONEBOOK_PATH = os.path.join(SRC_DIR, "phonebook.json")

# def main():
#     with open(PHONEBOOK_PATH, "r") as file:
#         data = json.load(file)
#         searcher = SearchEngine()
#         while True:
#             searcher.get_query()
#             searcher.search()

# if __name__ == "__main__":
#     main()
