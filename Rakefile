require "rspec/core/rake_task"

begin
  require "dotenv-heroku/tasks"
rescue LoadError
end

RSpec::Core::RakeTask.new(:spec)

task :default => :spec
