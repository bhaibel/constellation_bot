require 'spec_helper'

module ConstellationBot
  describe Constellation do
    describe '#to_svg' do
      let(:parsed) { Nokogiri::XML(subject.to_svg) }

      it 'is a svg' do
        expect(parsed).to have_node('svg')
      end

      it 'has dimensions' do
        skip "fix attribute tests"
        expect(parsed.root).to have_attribute('width').eq(640)
        expect(parsed.root).to have_attribute('height').eq(480)
        expect(parsed.root).to have_attribute('viewBox').eq("0 0 480 320")
      end

      it 'has stars' do
        expect(parsed).to have_node('svg/circle')
      end

      it 'has a colored background' do
        skip "fix attribute tests"
        # someday
        # expect(parsed.at('svg')).to have_attribute('viewport-fill').eq("#143166")
        expect(parsed).to have_node('svg/rect')
        expect(parsed.at('svg/rect')).to have_attribute('fill').eq("#143166")
        expect(parsed.at('svg/rect')).to have_attribute('x').eq("0")
        expect(parsed.at('svg/rect')).to have_attribute('y').eq("0")
        expect(parsed.at('svg/rect')).to have_attribute('width').eq(640)
        expect(parsed.at('svg/rect')).to have_attribute('height').eq(480)
      end

      it 'insets stars in a second svg' do
        inner = parsed.at('svg/svg')
        expect(inner).to have_node('circle')
        expect(inner).to have_attribute('viewBox')
      end
    end
  end
end
