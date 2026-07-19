# frozen_string_literal: true

require "test_helper"
require "mkmf"
require "rb_sys/mkmf"
require "tmpdir"

class MkmfEchoTest < Minitest::Test
  def test_generated_makefile_uses_absolute_echo_command
    skip "requires a POSIX echo executable" if win_target?

    Dir.mktmpdir do |dir|
      Dir.chdir(dir) do
        create_rust_makefile("example")

        makefile = File.read("Makefile")

        assert_includes makefile, "ECHO = $(ECHO1:0=@ /bin/echo)"
        refute_includes makefile, "ECHO = $(ECHO1:0=@ echo)"
      end
    end
  end
end
