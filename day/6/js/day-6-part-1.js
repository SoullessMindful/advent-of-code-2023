const fs = require('node:fs')

const filePath = process.argv[2]

const input = fs.readFileSync(filePath, 'utf-8')

const [timeStr, distStr] = input.split('\n')

const times = timeStr
  .split(' ')
  .filter((s) => s !== '')
  .slice(1)
  .map((s) => Number.parseInt(s))

const distances = distStr
  .split(' ')
  .filter((s) => s !== '')
  .slice(1)
  .map((s) => Number.parseInt(s))

const possibleDistances = times
  .map((t) => range(1, t - 1)
    .map((T) => T*(t-T))
  )

const possibleWinCounts = possibleDistances
  .map((pds, i) => [pds, distances[i]])
  .map(([pds, d]) => pds
    .filter((pd) => pd > d)
    .length
  )

const result = possibleWinCounts.reduce((prod, factor) => prod * factor, 1)

console.log(result)

function range(start, length) {
  return Array.from({ length }).map((_, i) => start + i)
}
