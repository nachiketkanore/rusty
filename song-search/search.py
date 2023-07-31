import meilisearch

client = meilisearch.Client('http://localhost:7700', 'aSampleMasterKey')

all = client.index('movies').search('avengers', {
                                            'limit': 10
                                        })

results = client.index('movies').search('avengers', {
                                            'limit': 2
                                        })

print("all results:\n")
for (id, result) in enumerate(all["hits"]):
    print(id, result["title"])

# import json
# output = json.dumps(results, sort_keys = True, indent = 4)
# print(output)

print()
LIMIT = 2
for offset in range(0, 10, LIMIT):
    print(f"results from offset = {offset}")
    for (id, result) in enumerate(
            client.index('movies').search('avengers', { 'limit': LIMIT, 'offset': offset })["hits"]):
        print(offset + id, result["title"])
    print()

