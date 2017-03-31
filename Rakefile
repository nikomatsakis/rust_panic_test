require 'bundler/setup'
require 'helix_runtime/build_task'
require 'rspec/core/rake_task'

# For Windows
$stdout.sync = true

HelixRuntime::BuildTask.new("panic_test") do |t|
  t.helix_lib_dir = __dir__
end

RSpec::Core::RakeTask.new(:spec) do |t|
  t.verbose = false
end

task :spec => :build
task :default => :spec
