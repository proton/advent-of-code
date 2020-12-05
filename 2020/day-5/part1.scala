import scala.io.Source
import scala.io.Source

object Part1 extends App {
  val filepath = "input.txt"
  // val lines = Source.fromFile(filepath).getLines()
  // for (line <- ) {
  //   println(line + " " + Array(seatId(line)).mkString("-"))
  // }
  val maxSeat = Source.fromFile(filepath).getLines().map(seatId(_)).reduceLeft(_ max _)
  println(maxSeat)

  def seatId(line: String): Int = {
    var minRow = 0
    var rowCnt = 128
    var minCol = 0
    var colCnt = 8
    for (char <- line) {
      if (char == 'F') {
        rowCnt /= 2
      } else if (char == 'B') {
        rowCnt /= 2
        minRow += rowCnt
      } else if (char == 'L') {
        colCnt /= 2
      } else if (char == 'R') {
        colCnt /= 2
        minCol += colCnt
      }
    }
    return minRow * 8 + minCol
  }
}

// BFFFBBFRRR: row 70, column 7, seat ID 567.
// FFFBBBFRRR: row 14, column 7, seat ID 119.
// BBFFBBFRLL: row 102, column 4, seat ID 820