func countTo(n: Int64): Int64 {
    var i = 0
    while (i < n) {
        i = i + 1
    }
    return i
}

main(): Int64 {
    return countTo(10)
}
