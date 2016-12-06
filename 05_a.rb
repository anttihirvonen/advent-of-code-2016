require 'digest'

door_id = File.open("input-05.txt").read[0..-2]
md5 = Digest::MD5.new
index = 0
password = ""

while password.length < 8
    md5.reset() << door_id << index.to_s

    if md5.to_s[0,5] == "00000"
        password << md5.to_s[5]
    end

    index += 1
end

puts password
