# frozen_string_literal: true

require "test_helper"
require "rb_sys/extensiontask"

class TestExtensionTask < Minitest::Test
  def setup
    Rake::Task.clear
    @original_manifest_dir = ENV.delete("RB_SYS_CARGO_MANIFEST_DIR")
    @original_target_dir = ENV.delete("RB_SYS_CARGO_TARGET_DIR")
    @original_profile = ENV.delete("RB_SYS_CARGO_PROFILE")
  end

  def teardown
    ENV["RB_SYS_CARGO_MANIFEST_DIR"] = @original_manifest_dir
    ENV["RB_SYS_CARGO_TARGET_DIR"] = @original_target_dir
    ENV["RB_SYS_CARGO_PROFILE"] = @original_profile
    Rake::Task.clear
  end

  def test_custom_ext_dir_is_used_for_extconf_and_cargo_manifest
    metadata = Struct.new(:manifest_directory, :target_directory)
      .new("ext/my_gem", "target")

    RbSys::Cargo::Metadata.stub(:new_or_inferred, metadata) do
      extension = RbSys::ExtensionTask.new("my_gem") do |ext|
        ext.ext_dir = "bindings/ruby"
      end

      assert_equal "bindings/ruby/extconf.rb", extension.extconf

      Rake::Task["rb_sys:env:default"].invoke
      assert_equal File.expand_path("bindings/ruby"), ENV["RB_SYS_CARGO_MANIFEST_DIR"]
    end
  end
end
