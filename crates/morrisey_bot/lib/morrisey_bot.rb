require "helix_runtime"

begin
  require "morrisey_bot/native"
rescue LoadError
  warn "Unable to load morrisey_bot/native. Please run `rake build`"
end
