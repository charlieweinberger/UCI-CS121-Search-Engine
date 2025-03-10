import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";

const fakeData: Website[] = [
  {
    url: "https://www.wikipedia.org",
    content: "",
    summaryIsLoaded: false,
    summary:
      "Wikipedia is a free online encyclopedia created and edited by volunteers around the world. It is the largest and most popular general reference work on the internet. Wikipedia is owned by the nonprofit Wikimedia Foundation.",
  },
  {
    url: "https://github.com",
    content: "",
    summaryIsLoaded: false,
    summary:
      "GitHub is a web-based platform used for version control and collaboration. It allows developers to work together on projects from anywhere in the world. GitHub is widely used for code sharing, project management, and open source contributions.",
  },
  {
    url: "https://stackoverflow.com",
    content: "",
    summaryIsLoaded: false,
    summary:
      "Stack Overflow is a question and answer website for professional and enthusiast programmers. It features questions and answers on a wide range of topics in computer programming. The website serves as a platform for users to ask and answer questions related to software development.",
  },
  {
    url: "https://developer.mozilla.org",
    content: "",
    summaryIsLoaded: false,
    summary:
      "MDN Web Docs is a comprehensive resource for web developers, providing documentation on HTML, CSS, JavaScript, and web APIs. It offers tutorials, references, and guides for building websites and applications. MDN is maintained by Mozilla with help from the developer community.",
  },
  {
    url: "https://vercel.com",
    content: "",
    summaryIsLoaded: false,
    summary:
      "Vercel is a cloud platform for static sites and serverless functions that enables developers to deploy websites globally. It provides a seamless developer experience to deploy instantly, scale automatically, and serve personalized content. Vercel is the creator and maintainer of Next.js, a popular React framework.",
  },
]

export function SearchBar({ query, setQuery, handleSubmit }: {
  query: string
  setQuery: (query: string) => void
  handleSubmit: (e: React.FormEvent) => void
}) {
  return (
    <form onSubmit={handleSubmit} className="w-full mb-8">
      <div className="flex w-full items-center space-x-2">
        <Input
          type="text"
          placeholder="Search the web..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          className="pl-4 pr-10 py-6 text-lg w-full"
        />
        <Button type="submit" size="lg" className="px-6">
          Search
        </Button>
      </div>
    </form>
  );
}

export function SearchResults({ query, websites }: {
  query: string
  websites: Website[]
}) {

  if (!query) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">Enter a search term to see results</p>
      </div>
    )
  }

  if (websites.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">No results found for "{query}"</p>
      </div>
    )
  }

  return (
    <div className="space-y-4">
      <p className="text-sm text-muted-foreground mb-4">
        Showing {websites.length} results for "{query}"
      </p>
      {websites.map((website: Website) => (
        <Card key={website.url} className="overflow-hidden hover:shadow-md transition-shadow">
          <CardContent className="p-4">
            <a
              href={website.url}
              target="_blank"
              rel="noopener noreferrer"
              className="text-lg font-medium text-green-600 dark:text-green-400 hover:underline mb-3 inline-block"
            >
              {website.url}
            </a>
            {website.summaryIsLoaded ? (
              <p className="text-sm text-slate-700 dark:text-slate-300 mt-2">{website.summary}</p>
            ) : (
              <div className="mt-2 space-y-2">
                <Skeleton className="h-3 w-full" />
                <Skeleton className="h-3 w-full" />
                <Skeleton className="h-3 w-4/5" />
              </div>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  )
}

export default function App() {

  const [ query, setQuery ] = useState("");
  const [ websites, setWebsites ] = useState<Website[]>([]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const results = fakeData;
    summarizeWebsites(results);
  }

  const summarizeWebsites = async (results: Website[]) => {
    
    if (!query) {
      setWebsites([]);
      return;
    }

    // Instantly load URLs with summaries marked as not loaded
    const initialWebsites = results.map((website: Website) => ({
      ...website,
      summaryIsLoaded: false,
    }));
    setWebsites(initialWebsites);

    // Load summaries one by one with 1 second delay between each
    initialWebsites.forEach((_, index) => {
      setTimeout(
        () => {
          setWebsites((prevWebsites: Website[]) => {
            const newWebsites: Website[] = [...prevWebsites];
            if (newWebsites[index]) {
              newWebsites[index].summaryIsLoaded = true;
            }
            return newWebsites;
          })
        },
        (index + 1) * 1000,
      ) // 1 second delay for each summary
    });

  }

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-950">
      <div className="container mx-auto px-4 py-16 flex flex-col items-center">
        <h1 className="text-4xl font-bold mb-8 text-center">Search Engine</h1>
        <div className="w-full max-w-2xl">
          <SearchBar query={query} setQuery={setQuery} handleSubmit={handleSubmit} />
          <SearchResults query={query} websites={websites} />
        </div>
      </div>
    </main>
  )
}
