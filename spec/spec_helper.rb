$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'constellation_bot'

require 'byebug'

require 'nokogiri'
require 'active_support'
require 'active_support/core_ext/object/blank'

RSpec::Matchers.define :have_node do |expected_path|
  match do |actual_node|
    actual_node.at(expected_path).present?
  end
end

RSpec::Matchers.define :have_attribute do |expected_attribute|
  match do |actual_node|
    @attribute = actual_node[expected_attribute]
  end

  chain :eq do |expected_value|
    @attribute.present? && attribute.value == expected_value
  end
end
