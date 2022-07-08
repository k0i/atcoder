fun twenty() {
    var s = readLine()!!.toInt()
    val res = mutableListOf(s)
    do {
        s = if (s % 2 == 0) {
            even(s)
        } else {
            odd(s)
        }
        if (res.contains(s)) {
            break
        }
        res.add(s)
    } while (true)
    println(res.size + 1)
}

private inline fun odd(arg: Int): Int = arg * 3 + 1
private inline fun even(arg: Int): Int = arg / 2

