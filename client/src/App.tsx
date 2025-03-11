import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";

import { GoogleGenerativeAI } from "@google/generative-ai";

//! Set up AI model (Gemini) for generating AI website summaries
const genAI = new GoogleGenerativeAI(import.meta.env.VITE_GEMINI_API_KEY);
const model = genAI.getGenerativeModel({
  model: "gemini-2.0-flash",
  systemInstruction:
    "You are playing the role of a website summarizer. You will be given the raw content of an html page. Imagine that the html content was rendered and you were looking at the website that was generated. What information would be presented to you? What does the website say? Summarize the content of the website in three sentences. Do not make up information that is not included in the html content. Do not mention the structure of the html at all.",
});

//! The component for displaying the search bar
export function SearchBar({
  query,
  setQuery,
  handleSubmit,
}: {
  query: string;
  setQuery: (query: string) => void;
  handleSubmit: (e: React.FormEvent) => void;
}) {
  return (
    <form onSubmit={handleSubmit} className="w-full mb-6">
      <div className="flex flex-row items-center gap-2">
        <Input
          type="text"
          placeholder="Search the web..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          className="pl-4 pr-10 py-6 text-lg"
        />
        <Button type="submit" size="lg" className="px-6">
          Search
        </Button>
      </div>
    </form>
  );
}

//! The component for displaying the search results
export function SearchResults({
  websites,
  searchTime,
}: {
  websites: Website[];
  searchTime: number;
}) {
  if (websites.length === 0) {
    return;
  }

  return (
    <div className="flex flex-col gap-4">
      <p className="text-sm text-muted-foreground">
        Showing {websites.length} results in {searchTime} milliseconds
      </p>
      {websites.map((website: Website) => (
        <div
          key={website.url}
          className="bg-white text-black flex flex-col gap-2 rounded-xl p-6 overflow-hidden"
        >
          <a
            href={website.url}
            target="_blank"
            rel="noopener noreferrer"
            className="text-lg font-medium text-blue-500 hover:underline"
          >
            {website.url}
          </a>
          {website.summary ? (
            <p className="text-sm">{website.summary}</p>
          ) : (
            <div className="mt-2 space-y-2">
              <Skeleton className="h-3 w-full" />
              <Skeleton className="h-3 w-full" />
              <Skeleton className="h-3 w-4/5" />
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

export default function App() {

  //! Variables for keeping track of the current query and the most recent query results and search time
  const [query, setQuery] = useState("");
  const [websites, setWebsites] = useState<Website[]>([]);
  const [searchTime, setSearchTime] = useState(0);

  //! Functoin for handling when the user submits their query, which calls fetchResults()
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    fetchResults();
  };

  //! Function to query the search engine (in the backend) and display the results (in the frontend)
  const fetchResults = () => {
    //! Attempt to data from the backend
    fetch("http://127.0.0.1:3000/search", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        query: query, //! Include the query here
        search_type: "name",
      }),
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
      })
      //! If the response is successful, then update the UI and create the AI summariez
      .then((data) => {
        console.log(data);
        summarizeWebsites(data.results);
        setSearchTime(data.time);
      })
      //! If any errors occur, return no results
      .catch((error) => {
        console.error(`Error while searching: ${error}`);
        return [];
      });
  };

  //! Function for updating the UI and summarizing the websites with AI 
  const summarizeWebsites = async (results: Result[]) => {
    if (!query) {
      setWebsites([]);
      return;
    }

    //! Instantly display the URLs, without the summaries for now
    const initialWebsites: Website[] = results.map((result: Result) => ({
      ...result,
      summary: "",
    }));
    setWebsites(initialWebsites);

    //! Generate the AI summaries one by one with a 0.5 second delay between each.
    //! We spread out the summaries so that we don't spam Gemini's servers with API calls
    //! to summarize the large chunks of html content that we're summarizing (politeness).
    results.forEach((result: Result, index: number) => {
      setTimeout(
        () => {
          //! Start generating the AI summary, with a max character limit of 50,000 per result
          model.generateContent(result.content.substring(0, 50000)).then(AIResponse => {
            //! Update the website UI with the summary once it's been generated
            setWebsites((prevWebsites) => {
              const newWebsites: Website[] = [...prevWebsites];
              newWebsites[index] = {
                ...result,
                summary: AIResponse.response.text() ?? "AI summary failed.",
              };
              return newWebsites;
            });
          });
        },
        (index + 1) * 500,
      ) // 1 second delay for each summary
    });

  };

  //! The component for displaying the overall web GUI
  return (
    <main className="min-h-screen bg-blue-100 px-4 py-16 flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-8 text-center">Search Engine</h1>
      <div className="w-full max-w-2xl">
        <SearchBar
          query={query}
          setQuery={setQuery}
          handleSubmit={handleSubmit}
        />
        <SearchResults websites={websites} searchTime={searchTime} />
      </div>
    </main>
  );
}
