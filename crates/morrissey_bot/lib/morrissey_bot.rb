require "helix_runtime"

begin
  require "morrissey_bot/native"
rescue LoadError
  warn "Unable to load morrissey_bot/native. Please run `rake build`"
end
