fun ten() {
    val card = readLine()!!.toInt()
    val arr = readLine()!!.split(" ").map { it.toInt() }.sortedDescending()
    println(arr.sum())
    println(arr.filterIndexed { index, _ -> index % 2 == 0 }.sum() - arr.filterIndexed { index, _ -> index % 2 != 0 }
        .sum())

}