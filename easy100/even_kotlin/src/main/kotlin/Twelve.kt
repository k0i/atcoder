import java.util.*

fun twelve() {
    val (len, house) = readLine()!!.split(" ").map { it.toInt() }
    val arr = readLine()!!.split(" ").map { it.toInt() }.sorted()


    val dif = mutableListOf<Int>()

    for (item in 0 until house - 1) {
        dif.add(arr[item + 1] - arr[item])
    }
    val a = if (arr[arr.lastIndex] - arr[0] > 10) {
        len - arr[arr.lastIndex]
    } else {
        arr[arr.lastIndex] - arr[0]
    }
    dif.add(a)
    dif.remove(Collections.max(dif))
    println(dif.sum())
}