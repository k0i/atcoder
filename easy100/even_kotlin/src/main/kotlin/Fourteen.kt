import kotlin.math.abs

fun fourteen(){
 val (n,k) =readLine()!!.split(" ").map{it.toLong()}
    var rest =n%k
 if(rest > k-rest){
  println(k-rest)
 }else(print(rest))
}