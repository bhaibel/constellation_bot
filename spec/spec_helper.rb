$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'constellation_bot'

require 'byebug'

require 'nokogiri'
require 'active_support'
require 'active_support/core_ext/object/blank'

RSpec::Matchers.define :have_node do |expected_path|
  match do |actual_node|
    parsed_node = Nokogiri::XML(actual_node)
    parsed_node.at(expected_path).present?
  end
end
