import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

import Groq from "groq-sdk";

const groq = new Groq({ apiKey: import.meta.env.VITE_GROQ_API_KEY });

async function getGroqChatCompletion() {
  return groq.chat.completions.create({
    messages: [
      {
        role: "system",
        content: "you are a helpful assistant.",
      },
      {
        role: "user",
        content: "Explain the importance of fast language models",
      },
    ],
    model: "llama-3.3-70b-versatile",
  });
}

async function getAISummary(website: Website): Promise<Website> {
  const chatCompletion = await getGroqChatCompletion();
  website.summary = chatCompletion.choices[0]?.message?.content || "AI summary failed.";
  return website;
}

export default function App() {

  const [ query, setQuery ] = useState("");
  const [ websites, setWebsites ] = useState<Website[]>([]);

  const handleSearch = async () => {
    try {
      // const response = await fetch("http://127.0.0.1:3000/search", {
      //   method: "POST",
      //   headers: {
      //     "Content-Type": "application/json",
      //   },
      //   body: JSON.stringify({
      //     query: query,
      //     search_type: "name",
      //   }),
      // });
      // const data = await response.json();
      // const results: Website[] = data.results;
      const results: Website[] = fakeData;
      console.log(results);
      return results;
    } catch (error) {
      console.error(`Error searching: ${error}`);
    }
  };

  return (
    <div className="bg-blue-100 py-44 h-screen flex items-center flex-col gap-12">
      <div className="text-8xl text-center font-bold flex flex-row">
        <p>CS 121 A3 Search Engine</p>
      </div>
      <div className="flex flex-row gap-2">
        <Input
          id="search-query"
          type="text"
          placeholder="Input search query here"
          onChange={(e) => setQuery(e.target.value)}
          className="w-96 bg-white"
          required
        />
        <Button type="submit" onClick={handleSearch}>
          Search
        </Button>
      </div>
    </div>
  );
}
