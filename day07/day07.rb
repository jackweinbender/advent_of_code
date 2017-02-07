def contains_abba( str )
    for i in 0...str.length - 3
        puts "#{str[i]} and #{str[i+3]} || #{str[i+1]} #{str[i+2]}"
    end
end


contains_abba("Test")