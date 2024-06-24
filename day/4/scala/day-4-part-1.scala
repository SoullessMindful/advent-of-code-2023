import scala.io.Source.fromFile
import scala.math.pow

@main
def partOne (filePath: String) =
  val src = fromFile(filePath)
  val lines = src.getLines.toArray
  src.close

  val result = processLines (lines)
  println(result)


def processLines (lines: Array[String]): Int =
  lines
    .map (processLine)
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

  pow (2, wins.length - 1)
    .toInt

