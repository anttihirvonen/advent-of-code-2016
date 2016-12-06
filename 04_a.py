from collections import defaultdict

sector_id_sum = 0

for line in open("input-04.txt"):
    splitted = line.rstrip("\n]").split("-")
    encrypted_name = "".join(splitted[0:-1])
    sector_id, checksum = splitted[-1].split("[")

    letter_map = defaultdict(int)
    for letter in encrypted_name:
        letter_map[letter] += 1

    letters = sorted(letter_map.items(), key=lambda x: (-x[1], x[0]))[:5]
    is_valid = "".join(l[0] for l in letters) == checksum

    if is_valid:
        sector_id_sum += int(sector_id)

print sector_id_sum
