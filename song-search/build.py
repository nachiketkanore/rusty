import meilisearch
import json

client = meilisearch.Client('http://localhost:7700', 'aSampleMasterKey')

# json_file = open('movies.json', encoding='utf-8')
# movies = json.load(json_file)
# client.index('movies').add_documents(movies)
print(client.get_task(0))
# print(client.is_healthy())
