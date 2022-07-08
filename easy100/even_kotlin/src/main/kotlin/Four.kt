import kotlin.math.floor

fun four() {
    val input = readLine()!!.toDouble()
    for (i in 1..50000) {
        if (floor(i * 1.08) == input) {
            println(i)
            break
        }
        if (i == 50000) {
            println(":(")
        }
    }
}