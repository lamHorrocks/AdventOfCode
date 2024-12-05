import re

def find_sum(str):
  sum = 0
  matches = re.findall(r'mul\((\d{1,3},\d{1,3})\)', str)

  for match in matches:
    x, y = match.split(',')
    sum += int(x) * int(y)

  return sum


data = ''
with open('input.txt', 'r', encoding='utf-8') as file:
  data = file.read().rstrip()

# Part 1
print("Part 1: ", find_sum(data))

# Part 2
total = 0
do = True
while True:
  if do:
    idx = data.find("don't()")

    if idx == -1:
      total += find_sum(data)
      break

    total += find_sum(data[:idx])
    data = data[idx:]
    do = False
  else:
    idx = data.find("do()")

    if idx == -1:
      break

    data = data[idx:]
    do = True


print("Part 2: ", total)