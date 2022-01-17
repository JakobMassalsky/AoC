inpt = readlines("AoC_20\\day17_input.txt")
l = length(inpt)
s = 6
A = Array{Bool,4}(undef, 2*s+l, 2*s+l, 2*s+1, 2*s+1)

for i in CartesianIndices(A)
    A[i] = (i[4] == i[3] == 7 && i[2] in 1+s:s+l && i[1] in 1+s:s+l) && inpt[i[2]-s][i[1]-s] == '#'
end

function getNearby(A::AbstractArray{Bool}, i::CartesianIndex)
    R, nA = CartesianIndices(A), 0::Int
    ifirst, ilast = first(R), last(R)
    i1 = oneunit(ifirst)
    for j in max(ifirst, i-i1):min(ilast, i+i1)
        nA += A[j] ? 1 : 0
    end
    return nA::Int
end

function takeSteps(A::AbstractArray{Bool}, s::Int)
    for i in 1:s
        tmp = copy(A)
        for i in CartesianIndices(A)
            nA = getNearby(A, i)
            tmp[i] = A[i] ? (nA in 3:4) : nA == 3
        end
        A = tmp
    end
    return A::AbstractArray{Bool}
end

@time takeSteps(A, s)

value = sum([x ? 1 : 0 for x in takeSteps(A, s)])
