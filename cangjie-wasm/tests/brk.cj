func firstOver(limit: Int64): Int64 {
    var i = 0
    while (true) {
        i = i + 1
        if (i > limit) {
            break
        }
    }
    return i
}

main(): Int64 {
    return firstOver(5)
}
