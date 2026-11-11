func sumTo(n: Int64): Int64 {
    var s = 0
    var i = 0
    while (i < n) {
        s = s + i
        i = i + 1
    }
    return s
}

main(): Int64 {
    return sumTo(10)
}
