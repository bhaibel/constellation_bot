require 'tmpdir'

require "constellation_bot/version"
require "constellation_bot/constellation"

module ConstellationBot
  def self.constellation
    Constellation.new.to_svg
  end

  def self.with_tempfile
    Dir.mktmpdir do |tempdir|
      Dir.chdir(tempdir) do
        write_png('constellation')
        File.open('constellation.png') do |f|
          yield(f)
        end
      end 
    end
  end

  def self.write_svg(name)
    File.open("#{name}.svg", 'w') do |f|
      f.write(constellation)
    end
  end

  def self.write_png(name)
    write_svg(name)
    system("inkscape --export-png #{name}.png #{name}.svg")
  end
end
