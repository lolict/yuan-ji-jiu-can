func grid(n: Int64): Int64 {
    var s = 0
    for (i in 1..=n) {
        for (j in 1..=n) {
            s = s + 1
        }
    }
    return s
}

main(): Int64 {
    return grid(3)
}
