inpt = ".#..#..#..#...#..#...###....##.#....
.#.........#.#....#...........####.#
#..##.##.#....#...#.#....#..........
......###..#.#...............#.....#
......#......#....#..##....##.......
....................#..............#
..#....##...#.....#..#..........#..#
..#.#.....#..#..#..#.#....#.###.##.#
.........##.#..#.......#.........#..
.##..#..##....#.#...#.#.####.....#..
.##....#.#....#.......#......##....#
..#...#.#...##......#####..#......#.
##..#...#.....#...###..#..........#.
......##..#.##..#.....#.......##..#.
#..##..#..#.....#.#.####........#.#.
#......#..........###...#..#....##..
.......#...#....#.##.#..##......#...
.............##.......#.#.#..#...##.
..#..##...#...............#..#......
##....#...#.#....#..#.....##..##....
.#...##...........#..#..............
.............#....###...#.##....#.#.
#..#.#..#...#....#.....#............
....#.###....##....##...............
....#..........#..#..#.......#.#....
#..#....##.....#............#..#....
...##.............#...#.....#..###..
...#.......#........###.##..#..##.##
.#.##.#...##..#.#........#.....#....
#......#....#......#....###.#.....#.
......#.##......#...#.#.##.##...#...
..#...#.#........#....#...........#.
......#.##..#..#.....#......##..#...
..##.........#......#..##.#.#.......
.#....#..#....###..#....##..........
..............#....##...#.####...##.
"

# println(inpt)

struct Asteroid
    x
    y
end

struct emptySpace
    x
    y
end

struct Angle
    x
    y
    dir
end

function getAngle(ang)
    return ang.x // ang.y
end

function getDistance(a1, a2)
    return sqrt((a2.x-a1.x)^2 + (a2.y-a1.y)^2)
end

function giveSights(currentAsteroid, asMap)
    cenx = currentAsteroid.x
    ceny = currentAsteroid.y
    visibleAsteroids = 0
    closestAsteroidsList = []

    for r = asMap
        if typeof(r) == Asteroid && r != currentAsteroid
            relAngleQuadr = 4
            relAngleQuadr = r.x - cenx >= 0 && r.y - ceny < 0 ? 1 : relAngleQuadr
            relAngleQuadr = r.x - cenx > 0 && r.y - ceny >= 0 ? 2 : relAngleQuadr
            relAngleQuadr = r.x - cenx <= 0 && r.y - ceny > 0 ? 3 : relAngleQuadr
            tempAngle = Angle(r.x - cenx, r.y - ceny, relAngleQuadr)
            pushAsteroid = true
            d = getDistance(currentAsteroid, r)

            for a = 1:length(closestAsteroidsList)
                if getAngle(tempAngle) == getAngle(closestAsteroidsList[a][2]) && tempAngle.dir == closestAsteroidsList[a][2].dir
                    pushAsteroid = false
                    closestAsteroidsList[a] = closestAsteroidsList[a][3] >= d ? closestAsteroidsList[a] : [r, tempAngle, d]
                    break
                end
            end
            pushAsteroid && push!(closestAsteroidsList, [r, tempAngle, d])
            visibleAsteroids += pushAsteroid ? 1 : 0
        end
    end
    return [currentAsteroid, closestAsteroidsList]
end

function sortByAngle(listOfAsteroids)
    quadr1 = []
    quadr2 = []
    quadr3 = []
    quadr4 = []
    sortedList = Array{Any, 1}(undef, 276)
    for e = listOfAsteroids
        e[2].dir == 1 && push!(quadr1, e)
        e[2].dir == 2 && push!(quadr2, e)
        e[2].dir == 3 && push!(quadr3, e)
        e[2].dir == 4 && push!(quadr4, e)
    end

    for e1 = quadr1
        a1 = getAngle(e1[2])
        smallerAngles = 0
        for e2 = quadr1
            a2 = getAngle(e2[2])
            smallerAngles += a1 < a2 ? 1 : 0
        end
        sortedList[smallerAngles+1] = e1
    end

    occupiedIndices = length(quadr1)

    for e1 = quadr2
        a1 = getAngle(e1[2])
        smallerAngles = 0
        for e2 = quadr2
            a2 = getAngle(e2[2])
            smallerAngles += a1 < a2 ? 1 : 0
        end
        sortedList[occupiedIndices + smallerAngles+1] = e1
    end

    occupiedIndices += length(quadr2)

    for e1 = quadr3
        a1 = getAngle(e1[2])
        smallerAngles = 0
        for e2 = quadr3
            a2 = getAngle(e2[2])
            smallerAngles += a1 < a2 ? 1 : 0
        end
        sortedList[occupiedIndices + smallerAngles+1] = e1
    end

    occupiedIndices += length(quadr3)

    for e1 = quadr4
        a1 = getAngle(e1[2])
        smallerAngles = 0
        for e2 = quadr4
            a2 = getAngle(e2[2])
            smallerAngles += a1 < a2 ? 1 : 0
        end
        sortedList[occupiedIndices + smallerAngles+1] = e1
    end

    return sortedList
end


lines = split(inpt, "\n")

asteroidStringMap = split.(lines, "")
asteroidMap = Matrix{}(undef, 36, 36)
#asteroidMap = Matrix{}(undef, 10, 10)

for x = 1:36
    for y = 1:36
        asteroidMap[x, y] = asteroidStringMap[y][x] == "." ? emptySpace(x - 1, y - 1) : Asteroid(x - 1, y - 1)
    end
end

asteroidValues = []

push!(asteroidValues, giveSights(asteroidMap[18, 23], asteroidMap))

visAsteroids = asteroidValues[1]

# println(visAsteroids)

for x = 1:length(sortByAngle(visAsteroids[2]))
    print(x)
    print(": ")
    println(sortByAngle(visAsteroids[2])[x])
end

# minn = -Inf

# for i = asteroidValues
#     if i[2] > minn
#         global minn = i[2]
#         println(i)
#     end
# end

# println(asteroidMap)
