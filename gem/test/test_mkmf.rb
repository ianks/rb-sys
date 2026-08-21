# frozen_string_literal: true

require "test_helper"
require "rb_sys/mkmf"

class TestMkmf < Minitest::Test
  class Harness
    include RbSys::Mkmf

    def initialize(executable)
      @executable = executable
    end

    private

    def find_executable(name)
      name == @executable
    end
  end

  def test_fixup_libnames_uses_dylib_basename_with_native_tool
    command = Harness.new("install_name_tool").send(:fixup_libnames)

    assert_equal '$(Q) install_name_tool -id "$(notdir $(DLLIB))" $(DLLIB)', command
  end

  def test_fixup_libnames_uses_dylib_basename_with_cross_tool
    command = Harness.new("$(CARGO_BUILD_TARGET)-install_name_tool").send(:fixup_libnames)

    assert_equal '$(Q) $(CARGO_BUILD_TARGET)-install_name_tool -id "$(notdir $(DLLIB))" $(DLLIB)', command
  end
end
