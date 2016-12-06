import string

def decrypt(char, sector_id):
    index = (ord(char) - ord('a') + sector_id) \
            % len(string.ascii_lowercase)
    return string.ascii_lowercase[index]

for line in open("input-04.txt"):
    splitted = line.rstrip("\n]").split("-")
    encrypted_name = " ".join(splitted[0:-1])
    sector_id = int(splitted[-1].split("[")[0])

    clear_name = "".join(map(lambda x: decrypt(x, sector_id),
                             encrypted_name))

    if "north" in clear_name:
        print clear_name, sector_id
        break
