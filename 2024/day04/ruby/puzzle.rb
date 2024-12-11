#! /usr/bin/env ruby

input = $stdin.readlines(chomp: true).map {|row| row.split('') }

def peek(pos, vec, input)
  r, c = pos
  rd, cd = vec
  row = r + rd
  col = c + cd
  return nil if row < 0 || col < 0
  return nil if row >= input.length || col >= input[row].length

  input[row][col]
end

vecs = {
  n:  [-1,0],  s: [1,0],  e: [0,1],  w: [0,-1],
  nw: [-1,-1], ne:[-1,1], sw:[1,-1], se:[1,1]
}
match = "XMAS".split('')
wsmatches = 0
xmas_matches = 0
input.each_with_index do |row, rdx|
  row.each_with_index do |col, cdx|
    # part1
    if input[rdx][cdx] == match[0]
      vecs.each do |k,dir|
        path = (0..(match.length-1))
          .map { |c| dir.map {|e| e*c} }
        wsmatches += 1 if path.each_with_index.all? do |vec, vdx|
          mrow = vec[0] + rdx
          mcol = vec[1] + cdx

          next if mrow < 0 || mcol < 0
          next if mrow >= input.length || mcol >= row.length

          true if input[mrow][mcol] == match[vdx]
        end
      end
    end
    # part2
    if input[rdx][cdx] == "A"
      nwse = [["M", "S"],["S", "M"]].any? do |pair|
        pair[0] == peek(vecs[:nw], [rdx,cdx], input) &&
        pair[1] == peek(vecs[:se], [rdx,cdx], input)
      end
      nesw = [["M", "S"],["S", "M"]].any? do |pair|
        pair[0] == peek(vecs[:ne], [rdx,cdx], input) &&
        pair[1] == peek(vecs[:sw], [rdx,cdx], input)
      end

      xmas_matches += 1 if nwse && nesw
    end

  end
end


part_one = wsmatches
part_two = xmas_matches

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
