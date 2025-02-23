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
        self.content = self.get_only_text(self.data['content'])

    # * In important Note, verify the dom structure validity
    def verify_content_HTML(self):
        try:
            BeautifulSoup.BeautifulSoup(self.data['content'], 'html.parser')
            return True
        except:
            return False

    def get_only_text(self, content: str) -> str:
        parser = BeautifulSoup.BeautifulSoup(content, 'html.parser')
        return parser.get_text()
