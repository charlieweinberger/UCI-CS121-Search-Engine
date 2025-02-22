# getting the json file and just downloading it and redturning it as a class or something so it is easy to use
import json
import bs4 as BeautifulSoup

def download_json(file_path: str):
    with open(file_path, 'r', encoding='utf-8') as file:
        return json.load(file)


class JSONData:
    def __init__(self, file_path: str):
        self.data = download_json(file_path)
        self.file_path = file_path
        self.url = self.data['url']
        self.content = self.data['content']


    # * In important Note, verify the dom structure validity
    def verify_content_HTML(self):
        try:
            parser = BeautifulSoup(self.content, 'html.parser')
            return True
        except:
            return False