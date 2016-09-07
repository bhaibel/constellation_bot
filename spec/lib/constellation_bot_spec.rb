require 'spec_helper'

describe ConstellationBot do
  describe '.constellation' do
    it 'is a svg' do
      expect(Nokogiri::XML(ConstellationBot.constellation)).to have_node('svg')
    end
  end
end
