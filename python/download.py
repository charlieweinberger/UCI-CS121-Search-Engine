# getting the json file and just downloading it and redturning it as a class or something so it is easy to use
import json
import bs4 as BeautifulSoup
import os


def download_json(file_path: str):
    with open(file_path, 'r', encoding='utf-8') as file:
        return json.load(file)


def generator_files(path: str):
    for root, _, files in os.walk(path):
        for file in files:
            yield os.path.join(root, file)


class Document:
    def __init__(self, file_path: str):
        self.data = download_json(file_path)
        self.file_path = file_path
        self.url = self.data['url']
        self.encoding = self.data['encoding']
        self.content = self.get_only_text(self.data['content'])

    def get_only_text(self, content: str) -> str:
        parser = BeautifulSoup.BeautifulSoup(
            content, 'html.parser', from_encoding=self.encoding)
        text = parser.get_text()

        # Get important text from relevant tags
        important_tags = ['b', 'strong', 'h1', 'h2', 'h3', 'title']
        important_text = []

        for tag in important_tags:
            elements = parser.find_all(tag)
            for element in elements:
                important_text.append(element.get_text().strip())
        # Add important text 5 times at the end, separated by spaces
        if important_text:
            important_string = ' '.join(important_text)
            text = text + ' ' + ' '.join([important_string] * 5)

        return text
