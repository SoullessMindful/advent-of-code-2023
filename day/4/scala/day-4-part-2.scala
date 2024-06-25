import scala.io.Source.fromFile
import scala.math.pow

@main
def partTwo (filePath: String) =
  val src = fromFile(filePath)
  val lines = src.getLines.toArray
  src.close

  val result = processLines (lines)
  println(result)


def processLines (lines: Array[String]): Int =
  val processedLines = lines
    .map (processLine)

  totalLines (
    0,
    processedLines.map((l) => (l,1))
  )
    .map ((_,count) => count)
    .foldLeft (0) (_ + _)


def processLine (line: String): Int =
  val Array(left, right) = line.split ("\\|")

  val Array(_, leftContent) = left.split(":")

  val leftNumbers = leftContent
    .split(" ")
    .filter(_ != "")
    .map(s => s.toInt)

  val rightNumbers = right
    .split(" ")
    .filter(_ != "")
    .map(s => s.toInt)

  val wins = rightNumbers.filter(n =>
    leftNumbers.contains (n)
  )

  wins.length


def totalLines (i: Int, lines: Array[(Int,Int)]): Array[(Int, Int)] =
  if (i >= lines.length)
    lines
  else
    val (winCount, scratchCount) = lines(i)

    totalLines(
      i + 1,
      lines
        .zipWithIndex
        .map((el, j) =>
          if (j > i && j <= i + winCount)
            (el(0), el(1) + scratchCount)
          else
            el
        )
    )
