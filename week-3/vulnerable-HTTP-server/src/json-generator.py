depth = 100  # Start low and increase until crash
obj = '"boom"'
for _ in range(depth):
    obj = '{"a":' + obj + '}'

with open("nested.json", "w") as f:
    f.write(obj)
