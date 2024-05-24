import sys
import io
from functools import reduce

substitutionDict = {
  "one": "1",
  "two": "2",
  "three": "3",
  "four": "4",
  "five": "5",
  "six": "6",
  "seven": "7",
  "eight": "8",
  "nine": "9",
}

digits = [
  "0",
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
  "9",
]


def processLine(line: str, dict: dict[str, str]) -> int | None:
  matchesAlphabetical = [(v, line.find(k)) for (k, v) in dict.items()]
  matchesAlphabeticalR = [(v, line.rfind(k)) for (k, v) in dict.items()]
  matchesDigital = [(v, line.find(v)) for v in digits]
  matchesDigitalR = [(v, line.rfind(v)) for v in digits]
  matches = matchesAlphabetical + matchesAlphabeticalR + matchesDigital + matchesDigitalR
  filteredMatches = list(filter(lambda x: x[1] != -1, matches))
  sortedMatches = sorted(filteredMatches, key=lambda x: x[1])
  if len(sortedMatches) == 0:
    return None
  else:
    result = sortedMatches[0][0] + sortedMatches[-1][0]
    return int(result)


lines = io.open(sys.argv[1], "r").readlines()
processedLines = map(lambda line: processLine(line, substitutionDict), lines)
filteredLines = filter(lambda i: i is not None, processedLines)
sum = reduce(lambda x, y: x+y, filteredLines, 0)

print(sum)
