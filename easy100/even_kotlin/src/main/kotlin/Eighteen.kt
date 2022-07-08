fun eighteen() {
    val input = readLine()!!

    var res = 0
    val chars = listOf("A", "C", "G", "T")
    var temp = 0
    for (element in input) {
        if (chars.contains(element.toString())) {
            temp++
            if (res < temp) {
                res = temp
            }
        } else {
            temp = 0
        }
    }

    println(res)
}
