range = 193651:649729
counter = 0

function getDigit(n, i)
    return i > 0 ? (n % 10^i - n % 10^(i-1)) / 10^(i-1) : 0
end

for r = range
    falses1 = 0
    hasTruePair = 0
    for i = 1:5
        falses1 += getDigit(r, i) < getDigit(r, i+1)
    end
    falses1 == 0 || continue
    for i = 1:5
        hasTruePair += getDigit(r, i) == getDigit(r, i+1) && getDigit(r, i) != getDigit(r, i+2) && getDigit(r, i) != getDigit(r, i-1)
    end
    hasTruePair > 0 || continue
    global counter += 1
end
print("Amount of possible keys: ")
print(counter)
