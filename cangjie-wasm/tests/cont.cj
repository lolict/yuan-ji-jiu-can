func sumOdd(n: Int64): Int64 {
    var s = 0
    for (i in 1..=n) {
        if (i % 2 == 0) {
            continue
        }
        s = s + i
    }
    return s
}

main(): Int64 {
    return sumOdd(10)
}
