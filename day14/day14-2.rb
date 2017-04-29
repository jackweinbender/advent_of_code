require 'digest'

salt = "ihaygndm"
test_salt = "abc"

num_keys = 64

triples = Hash.new
keys = []

i = 0

def stretch_hash(str, iter)
    iter.times do 
        str = Digest::MD5.hexdigest(str)
    end
    str
end
while keys.length < num_keys
    md5 = stretch_hash("#{salt}#{i.to_s}", 2017)
    fiple = md5.downcase.scan(/(.)\1\1\1\1/).flatten
    if not fiple.empty?
        fip = fiple[0]
        if triples.key?(fip)
            keys += triples[fip]
            .select{|key, hash| key + 1000 > i}
            .map{|m| puts "#{m[0]}-#{fip}, #{m[1]}, (#{i}) #{md5}"
                    m}
        end
    end
    triple = md5.scan(/(.)\1\1/).flatten
    if not triple.empty?
        trip = triple[0]
        if triples.key?(trip)
            triples[trip] << [i, md5]
        else
            triples[trip] = [[i, md5]]
        end
    end
    
    i += 1
end
puts "#{keys.map{|k,v| k}.sort[num_keys-1]}"
    