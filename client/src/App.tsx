import { useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
export default function App() {
  const [query, setQuery] = useState("");

  const handleSearch = async () => {
    try {
      const response = await fetch("http://127.0.0.1:3000/search", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          query: query,
          search_type: "name",
        }),
      });
      const data = await response.json();
      const results = data.results;
      console.log(results);
    } catch (error) {
      console.error("Error searching:", error);
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
