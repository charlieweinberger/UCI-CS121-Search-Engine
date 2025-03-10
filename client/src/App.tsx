import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";

//! Uncomment when done testing
// import { GoogleGenerativeAI } from "@google/generative-ai";

//! Uncomment when done testing
// const genAI = new GoogleGenerativeAI(import.meta.env.VITE_GEMINI_API_KEY);
// const model = genAI.getGenerativeModel({
//   model: "gemini-1.5-flash",
//   systemInstruction: "You are playing the role of a website summarizer. You will be given the raw content of an html page. Imagine that the html content was rendered and you were looking at the website that was generated. What information would be presented to you? What does the website say? Summarize the content of the website in three sentences. Do not make up information that is not included in the html content. Do not mention the structure of the html at all.",
// });

async function summarizeWebsite(result: Result): Promise<Website> {
  //! Uncomment when done testing
  // const summary = await model.generateContent(result.content);
  // return {
  //   ...result,
  //   summary: summary.response.text() ?? "AI summary failed."
  // };
  //! Delete when done testing
  await new Promise(resolve => setTimeout(resolve, 1000));
  return {
    ...result,
    summary: "this is a website",
  };
}

export function SearchBar({ query, setQuery, handleSubmit }: {
  query: string
  setQuery: (query: string) => void
  handleSubmit: (e: React.FormEvent) => void
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

export function SearchResults({ websites, searchTime }: {
  websites: Website[]
  searchTime: number
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
        <div key={website.url} className="bg-white text-black flex flex-col gap-2 rounded-xl p-6 overflow-hidden">
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
  )
}

export default function App() {

  const [ query, setQuery ] = useState("");
  const [ websites, setWebsites ] = useState<Website[]>([]);
  const [ searchTime, setSearchTime ] = useState(0);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    fetchResults();
  }

  const fetchResults = () => {
    fetch("http://127.0.0.1:3000/search", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        query: query,
        search_type: "name",
      }),
    })
      .then(response => {
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        return response.json();
      })
      .then(data => {
        summarizeWebsites(data.results);
        setSearchTime(data.time);
      })
      .catch(error => {
        console.error(`Error while searching: ${error}`);
        return [];
      });
  }

  const summarizeWebsites = async (results: Result[]) => {
    
    if (!query) {
      setWebsites([]);
      return;
    }

    // Instantly load URLs with summaries marked as not loaded
    const initialWebsites: Website[] = results.map((result: Result) => ({
      ...result,
      summary: "",
    }));
    setWebsites(initialWebsites);

    // Load summaries one by one with 1 second delay between each
    results.forEach(async (result: Result, index: number) => {
      const summarizedWebsite: Website = await summarizeWebsite(result);
      setWebsites((prevWebsites: Website[]) => {
        const newWebsites: Website[] = [...prevWebsites];
        newWebsites[index] = summarizedWebsite;
        return newWebsites;
      })
    });

  }

  return (
    <main className="min-h-screen bg-blue-100 px-4 py-16 flex flex-col items-center">
      <h1 className="text-4xl font-bold mb-8 text-center">Search Engine</h1>
      <div className="w-full max-w-2xl">
        <SearchBar query={query} setQuery={setQuery} handleSubmit={handleSubmit} />
        <SearchResults websites={websites} searchTime={searchTime} />
      </div>
    </main>
  )
}
