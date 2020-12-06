import scala.io.Source

object Part1 extends App {
  val ROW_COUNT = 128
  val COL_COUNT = 8

  val filepath = "input.txt"
  val seats = Source
                  .fromFile(filepath)
                  .getLines()
                  .map(seatId(_))
                  .toList
                  .sorted

  seats.reduce((a, b) => {
    if (b - a > 1) println(a + 1)
    b
  })

  def seatId(line: String): Int = {
    var row = 0
    var rowCnt = ROW_COUNT
    var col = 0
    var colCnt = COL_COUNT
    for (char <- line) {
      char match {
        case 'F' => rowCnt /= 2
        case 'B' => rowCnt /= 2; row += rowCnt
        case 'L' => colCnt /= 2
        case 'R' => colCnt /= 2; col += colCnt
      }
    }
    row * COL_COUNT + col
  }
}