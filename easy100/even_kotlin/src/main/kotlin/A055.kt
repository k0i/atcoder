fun a055() {
    val answer: MutableMap<String, Long> = mutableMapOf(
        "a" to 0,
        "b" to 0,
        "c" to 0,
        "d" to 0,
        "e" to 0,
        "f" to 0,
        "g" to 0,
        "h" to 0,
        "i" to 0,
        "j" to 0,
        "k" to 0,
        "l" to 0,
        "m" to 0,
        "n" to 0,
        "o" to 0,
        "p" to 0,
        "q" to 0,
        "r" to 0,
        "s" to 0,
        "t" to 0,
        "u" to 0,
        "v" to 0,
        "w" to 0,
        "x" to 0,
        "y" to 0,
        "z" to 0
    )
    val s = readLine()!!
    val desect = mutableListOf<String>()
    val onlyNums = mutableListOf<Long>()

    for (i in s.indices) {
        if (s[i].toString() == 0.toString()) {
            continue
        }
        if (s[i].isDigit()) {
            val arr = s.slice(i until s.length)
            val num = arr.takeWhile { it.isDigit() }.toLong()
            if (i == 0 || !s[i - 1].isDigit()) {
                onlyNums.add(num)
                desect.add(num.toString())
            }
        } else {
            desect.add(s[i].toString())
        }
    }
    println(desect)
    println(onlyNums)
    var parentheses = 0
    for (i in desect.indices) {

        if (desect[i].toLongOrNull() != null) {
            if (desect[i + 1] != "(") {
                parentheses += 1
                answer[desect[i + 1]] =
                    onlyNums.slice(0 until parentheses).reduce { res, element -> res * element } * desect[i].toLong()

                desect[i + 1] = "NONE"
                parentheses -= 1
                continue
            }
        }
        if (desect[i] == "(") {
            parentheses += 1
            continue
        } else if (desect[i] == ")") {
            onlyNums.removeAt(parentheses - 1)
            parentheses -= 1
            continue
        }
        if (parentheses == 0) {
            println(desect[i])
            continue
        }

        println(onlyNums.slice(0 until parentheses))
        answer[desect[i]] = onlyNums.slice(0 until parentheses).reduce { res, element -> res * element }

    }

    println(s)

    println(answer)
}