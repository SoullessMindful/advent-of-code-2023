import io

lines = io.open("../input", "r").readlines()


def processLine(line: str) -> int | None:
  digits = list(filter(lambda c: c.isdigit(), line))
  if len(digits) == 0:
    return None
  else:
    return int(digits[0] + digits[-1])


processedLines = map(processLine, lines)
filteredLines = filter(lambda i: i is not None, processedLines)
answers = map(lambda i: str(i), filteredLines)

print("\n".join(answers))
