require 'spec_helper'

module ConstellationBot
  describe Constellation do
    describe '#to_svg' do
      it 'is a svg' do
        expect(subject.to_svg).to have_node('/svg')
      end
    end
  end
end
