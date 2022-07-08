fun twentyTwo() {
    var (_, x) = readLine()!!.split(" ").map { it.toInt() }
    val arr = readLine()!!.split(" ").map { it.toInt() }.sorted()
    var res = 0
    loop@ for (i in arr.indices) {
        if (i == arr.size - 1) {
            if (x == arr[i]) {
                res++
            }
            break@loop
        }
        if (x >= arr[i]) {
            res++
            x -= arr[i]
        }else{break@loop}

    }
    println(res)
}