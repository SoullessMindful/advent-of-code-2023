import io
from functools import reduce

lines = io.open("../input", "r").readlines()


def processLine(line: str) -> int | None:
  digits = list(filter(lambda c: c.isdigit(), line))
  if len(digits) == 0:
    return None
  else:
    return int(digits[0] + digits[-1])


processedLines = map(processLine, lines)
filteredLines = filter(lambda i: i is not None, processedLines)
sum = reduce(lambda x, y: x+y, filteredLines, 0)

print(sum)
