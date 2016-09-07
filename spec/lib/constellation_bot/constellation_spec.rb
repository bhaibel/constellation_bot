require 'spec_helper'

module ConstellationBot
  describe Constellation do
    describe '#to_svg' do
      let(:parsed) { Nokogiri::XML(subject.to_svg) }

      it 'is a svg' do
        expect(parsed).to have_node('svg')
      end

      it 'has dimensions' do
        expect(parsed.at('svg')).to have_attribute('width').eq(480)
        expect(parsed.at('svg')).to have_attribute('height').eq(320)
        expect(parsed.at('svg')).to have_attribute('viewBox').eq("0 0 480 320")
      end

      it 'has stars' do
        expect(parsed).to have_node('svg/circle')
      end

      it 'has a colored background' do
        # someday
        # expect(parsed.at('svg')).to have_attribute('viewport-fill').eq("#143166")
        expect(parsed).to have_node('svg/rect')
        expect(parsed.at('svg/rect')).to have_attribute('fill').eq("#143166")
        expect(parsed.at('svg/rect')).to have_attribute('x').eq("0")
        expect(parsed.at('svg/rect')).to have_attribute('y').eq("0")
        expect(parsed.at('svg/rect')).to have_attribute('width').eq(480)
        expect(parsed.at('svg/rect')).to have_attribute('height').eq(480)
      end
    end
  end
end
