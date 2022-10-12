class String
  # colorization
  def colorize(color_code)
    "\e[#{color_code}m#{self}\e[0m"
  end

  def red
    colorize(31)
  end

  def green
    colorize(32)
  end

  def yellow
    colorize(33)
  end

  def blue
    colorize(34)
  end

  def pink
    colorize(35)
  end

  def light_blue
    colorize(36)
  end
end

def assert(boolean)
  message = boolean ? 'PASSED'.green : 'FAILED'.red
  puts message
end

def quiet_assert(boolean)
  puts 'FAILED'.red unless boolean
end

def tests(title)
  puts title.upcase.center(20, '-').yellow
  yield
  puts '-'.center(20, '-').yellow
  puts "\n"
end

def results
  puts 'Results'.upcase.center(20, '-').blue
  yield
  puts ''.center(20, '-').blue
end
