# getting the json file and just downloading it and redturning it as a class or something so it is easy to use
import json
import bs4 as BeautifulSoup
import os
import filter

#! Return the json object stored in the inputted file location
def download_json(file_path: str):
    with open(file_path, 'r', encoding='utf-8') as file:
        return json.load(file)

class Document:

    def __init__(self, file_path: str):
        #! Get the json data from a file in /developer/DEV 
        self.data = download_json(file_path)
        #! Read the data within the json
        self.url = self.data['url']
        self.encoding = self.data['encoding']
        #! Parse the html content for text, and highlight important text
        self.content = self.get_only_text(self.data['content'])

    def get_only_text(self, content: str) -> str | None:

        #! If the filter is not valid, return None
        if not filter.is_valid_page(self.url, content):
            return None

        #! Parse the html content, for the specified encoding
        parser = BeautifulSoup.BeautifulSoup(
            content, 'html.parser', from_encoding=self.encoding)
        text = parser.get_text()
        
        #! From the project assignment specification:
        # Important text: text in bold (b, strong), in headings (h1, h2, h3), and
        # in titles should be treated as more important than the in other places.
        # Verify which are the relevant HTML tags to select the important words.

        #! For all instances of all important tags, add the text to `important_text`
        important_text = []
        for tag in ['b', 'strong', 'h1', 'h2', 'h3', 'title']:
            elements = parser.find_all(tag)
            for element in elements:
                important_text.append(element.get_text().strip())
        #! Add important text 5 times at the end, separated by spaces
        if important_text:
            important_string = ' '.join(important_text)
            text += ' ' + ' '.join([important_string] * 5)

        #! Return the parsed html text content
        return text
