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

const firstWinningTime = range(1, time - 1)
  .find((T) => T*(time-T) > distance)

const lastWinningTime = time - firstWinningTime
const result = lastWinningTime - firstWinningTime + 1

console.log(result)

function range(start, length) {
  return Array.from({ length }).map((_, i) => start + i)
}
