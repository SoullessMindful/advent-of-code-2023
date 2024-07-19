const fs = require('node:fs')

const filePath = process.argv[2]

const input = fs.readFileSync(filePath, 'utf-8')

const [timeStr, distStr] = input.split('\n')

const time = Number.parseInt(timeStr
  .split(' ')
  .filter((s) => s !== '')
  .slice(1)
  .join(''))

const distance = Number.parseInt(distStr
  .split(' ')
  .filter((s) => s !== '')
  .slice(1)
  .join(''))

const possibleDistances = range(1, time - 1)
  .map((T) => T*(time-T))

const possibleWinCount = possibleDistances
  .filter((pd) => pd > distance)
  .length

console.log(possibleWinCount)

function range(start, length) {
  return Array.from({ length }).map((_, i) => start + i)
}
