p, d = 0, 1im
locations = []

for cmd in split(readline(), ", ")
    d = d*(cmd[1] == 'L' ? -1im : 1im)

    for i = 1:parse(Int, cmd[2:end])
        p += d

        for loc in locations
            if loc == p
                println(abs(real(p)) + abs(imag(p)))
                return
            end
        end

        push!(locations, p)
    end
end

