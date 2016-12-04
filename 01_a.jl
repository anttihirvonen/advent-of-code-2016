p, d = 0, 1im

for cmd in split(readline(), ", ")
    # rotate complex
    d = d*(cmd[1] == 'L' ? -1im : 1im)
    p += d*parse(Int, cmd[2:end])
end

println(abs(real(p)) + abs(imag(p)))
