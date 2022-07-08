fun six() {
    val (a, b) = readLine()!!.split(" ").map { x -> x.toInt() }
    if (a == 1 || b == 1) {
        println("1")
    } else if (a * b % 2 == 1) {
        println(a * b / 2 + 1)
    } else {
        println(a * b / 2)
    }
}