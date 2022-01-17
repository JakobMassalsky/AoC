pk1, pk2 = 8987316, 14681524
ls1, ls2 = 2541700, 4208732
# ls1, ls2 = 8, 11
function trans(sn, ls)
    acc = 1
    for i in 1:ls
        acc *= sn
        acc %= 20201227
    end
    return acc
end
n = 1
for i in 1:10000000
    global n *= 7
    global n %= 20201227
    if n == pk1
        println("", "Loop size of pk1 = ", i)
    end
    if n == pk2
        println("Loop size of pk2 = ", i)
    end
end

println(trans(pk1, ls2))
