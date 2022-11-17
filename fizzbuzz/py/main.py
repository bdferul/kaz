def fizzbuzz(a: int):
    sneed = [
        ["Fizz", 3],
        ["Buzz", 5],
    ]

    for i in range(1,a):
        p = ""
        for [s,x] in sneed:
            if i % x == 0:
                p += s
        print(i if p == "" else p)

fizzbuzz(100)