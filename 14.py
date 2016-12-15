import md5
import itertools

def contains_five_same_chars(string, char):
    for group in itertools.groupby(string):
        if group[0] == char and sum(1 for _ in group[1]) >= 5:
            return True

    return False

def find_three_same_char(string):
    for group in itertools.groupby(string):
        if sum(1 for _ in group[1]) >= 3:
            return group[0]

    return None

def search(salt, key_func):
    potential_keys, key_indices = [], []

    for index in itertools.count():
        p_key = key_func(salt + str(index))

        for key in potential_keys[:]:
            if index - key[1] < 1000:
                if contains_five_same_chars(p_key, key[0]):
                    potential_keys.remove(key)
                    key_indices.append(key[1])
            else:
                potential_keys.remove(key)

        if len(key_indices) >= 64:
            return sorted(key_indices)

        key_char = find_three_same_char(p_key)
        if key_char is not None:
            potential_keys.append((key_char, index, p_key))

def a_key(string):
    return md5.new(string).hexdigest()

def b_key(string):
    h = md5.new(string)
    for i in range(2016):
        h = md5.new(h.hexdigest())

    return h.hexdigest()

salt = "ihaygndm"
print "A: ", search(salt, a_key)[63]
print "B: ", search(salt, b_key)[63]
