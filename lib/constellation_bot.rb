require "constellation_bot/version"
require "constellation_bot/constellation"

module ConstellationBot
  def self.constellation
    Constellation.new.to_svg
  end
end
