require 'spec_helper'

describe ConstellationBot do
  it 'does not crash' do
    expect { ConstellationBot.constellation }.to_not raise_error
  end
end
