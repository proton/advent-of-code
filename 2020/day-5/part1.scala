import scala.io.Source
import scala.io.Source

object Part1 extends App {
  val filepath = "input.txt"
  val maxSeat = Source
                  .fromFile(filepath)
                  .getLines()
                  .map(seatId(_))
                  .reduceLeft(_ max _)

  println(maxSeat)

  def seatId(line: String): Int = {
    var row = 0
    var rowCnt = 128
    var col = 0
    var colCnt = 8
    for (char <- line) {
      char match {
        case 'F' => rowCnt /= 2
        case 'B' => rowCnt /= 2; row += rowCnt
        case 'L' => colCnt /= 2
        case 'R' => colCnt /= 2; col += colCnt
      }
    }
    return row * 8 + col
  }
}