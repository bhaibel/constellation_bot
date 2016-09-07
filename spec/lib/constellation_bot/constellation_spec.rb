require 'spec_helper'

module ConstellationBot
  describe Constellation do
    describe '#to_svg' do
      let(:parsed) { Nokogiri::XML(subject.to_svg) }

      it 'is a svg' do
        expect(parsed).to have_node('svg')
      end

      it 'has dimensions' do
        expect(parsed.at('svg')).to have_attribute('width').eq(100)
        expect(parsed.at('svg')).to have_attribute('height').eq(100)
        expect(parsed.at('svg')).to have_attribute('viewBox').eq("0 0 100 100")
      end

      it 'has stars' do
        expect(parsed).to have_node('circle')
      end
    end
  end
end
