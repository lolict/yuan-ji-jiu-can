func sumRange(n: Int64): Int64 {
    var s = 0
    for (i in 1..=n) {
        s = s + i
    }
    return s
}

main(): Int64 {
    return sumRange(10)
}
