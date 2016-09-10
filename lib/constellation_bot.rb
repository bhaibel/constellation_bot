require 'tempfile'

require "constellation_bot/version"
require "constellation_bot/constellation"

module ConstellationBot
  def self.constellation
    Constellation.new.to_svg
  end

  def self.with_tempfile
    Tempfile.open('constellation.svg') do |tempfile|
      tempfile.write(constellation)
      File.open(tempfile.path) do |file|
        yield(file)
      end
    end
  end
end
