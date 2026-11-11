func add(a: Int64, b: Int64): Int64 {
    return a + b
}

main(): Int64 {
    let x = add(1, 2)
    if (x > 5) {
        return x
    }
    return 0
}
