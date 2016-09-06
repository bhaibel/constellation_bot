require 'spec_helper'

describe ConstellationBot do
  describe '.constellation' do
    subject { ConstellationBot.constellation }

    it 'is a svg' do
      parsed_constellation = Nokogiri::XML(subject).children.first
      expect(parsed_constellation.name).to eq('svg')
    end
  end
end
