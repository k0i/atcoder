fun sixteen(){
    val len =readLine()!!.toInt()
    val arr =readLine()!!.split(" ").map{it.toInt()}

    val res = mutableListOf<Int>()
    for (i in 0 until len){
      res.add(0)
    }
     for (i in 0 until len){
      res[i]=arr.indexOfFirst { it==i+1 }+1
    }

    println(res)
}