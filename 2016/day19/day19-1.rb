
num_elfs = 3005290
queue = Queue.new

for i in 1..num_elfs
    queue.push([i, 1])
end

while queue.length > 1
    a = queue.pop()
    b = queue.pop()

    queue.push([a[0], a[1] + b[1]])
end

winner = queue.pop()

puts "Elf #{winner[0]} won with #{winner[1]} gifts."