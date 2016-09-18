begin
  require "rspec/core/rake_task"
  require "dotenv-heroku/tasks"

  RSpec::Core::RakeTask.new(:spec)

  task :default => :spec

rescue LoadError
end
