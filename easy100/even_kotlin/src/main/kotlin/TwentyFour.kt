fun twentyFour() {
    val (_, _, m) = readLine()!!.split(" ").map { it.toInt() }
    val aAr = readLine()!!.split(" ").map { it.toInt() }
    val bAr = readLine()!!.split(" ").map { it.toInt() }
    var res = aAr.minOrNull()!!.plus(bAr.minOrNull()!!)

    for (i in 0 until m) {
        val coupon = readLine()!!.split(" ").map { it.toInt() }
        res =
            if (aAr[coupon[0] - 1] + bAr[coupon[1] - 1] - coupon[2] < res) aAr[coupon[0] - 1] + bAr[coupon[1] - 1] - coupon[2] else res
    }
    println(res)
}
