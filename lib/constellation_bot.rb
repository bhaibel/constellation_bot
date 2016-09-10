require 'tempfile'
require 'rmagick'
require 'byebug'

require "constellation_bot/version"
require "constellation_bot/constellation"

module ConstellationBot
  def self.constellation
    Constellation.new.to_svg
  end

  def self.with_tempfile
    Tempfile.open(['constellation','.png']) do |tempfile|
      ilist = Magick::ImageList.new
      image = ilist.from_blob("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>#{constellation}")
      image.write(tempfile.path)
      File.open(tempfile.path) do |file|
        yield(file)
      end
    end
  end
end
