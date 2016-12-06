require 'digest'

md5 = Digest::MD5.new
door_id = File.open("input-05.txt").read[0..-2]
index = 0
password_len = 0
password = "gggggggg"

while password_len < 8
    md5.reset() << door_id << index.to_s

    if md5.to_s[0,5] == "00000"
        position = Integer(md5.to_s[5]) rescue nil

        if !position.nil? and password[position] == "g"
            password[position] = md5.to_s[6]
            password_len += 1
        end
    end

    index += 1
end

puts password
