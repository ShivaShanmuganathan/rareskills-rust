depth = 20
repeat = 500  # how many entries to add
entry = '"boom"'
for _ in range(depth):
    entry = '{"a":' + entry + '}'

with open("recursive_map.json", "w") as f:
    f.write('{')
    for i in range(repeat):
        f.write(f'"entry_{i}": {entry},')
    f.write('"done": "ok"}')  # end properly
