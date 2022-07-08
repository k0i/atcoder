import kotlin.math.roundToInt

fun two() {
    val menbers = readLine()!!.toDouble()
    val input = readLine()!!.split(" ").map { it -> it.toInt() }
    val avg = (input.sum().toDouble() / input.size).roundToInt()
    println(input.sumOf { it -> (it - avg) * (it - avg) })
}