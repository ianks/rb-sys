# frozen_string_literal: true

require "test_helper"
require "fileutils"
require "rb_sys/extensiontask"
require "tmpdir"

class TestExtensionTask < Minitest::Test
  def setup
    @original_rake_application = Rake.application
    @original_env = ENV.to_h
    Rake.application = Rake::Application.new
  end

  def teardown
    Rake.application = @original_rake_application
    ENV.replace(@original_env)
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

  def test_force_install_defines_compile_tasks_without_cargo
    in_new_extension do |dir, spec|
      ENV["CARGO"] = File.join(dir, "missing-cargo")
      ENV["RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN"] = "true"

      extension_task = RbSys::ExtensionTask.new("example-extension", spec)

      assert_equal File.join(dir, "ext/example_extension"), extension_task.ext_dir
      assert_equal File.join(dir, "target"), extension_task.target_directory
      assert Rake::Task.task_defined?("compile")

      Rake::Task["rb_sys:env:default"].invoke
      assert_equal File.join(dir, "ext/example_extension"), ENV["RB_SYS_CARGO_MANIFEST_DIR"]
      assert_equal File.join(dir, "target"), ENV["RB_SYS_CARGO_TARGET_DIR"]
    end
  end

  def test_missing_cargo_still_raises_without_force_install
    in_new_extension do |dir, spec|
      ENV["CARGO"] = File.join(dir, "missing-cargo")
      ENV.delete("RB_SYS_FORCE_INSTALL_RUST_TOOLCHAIN")

      assert_raises(RbSys::CargoMetadataError) do
        RbSys::ExtensionTask.new("example-extension", spec)
      end
    end
  end

  private

  def in_new_extension
    Dir.mktmpdir do |dir|
      Dir.chdir(dir) do
        extension_dir = "ext/example_extension"
        FileUtils.mkdir_p(extension_dir)
        File.write("Cargo.toml", "[workspace]\nmembers = [\"#{extension_dir}\"]\n")
        File.write(File.join(extension_dir, "Cargo.toml"), "[package]\nname = \"example-extension\"\nversion = \"0.1.0\"\n")
        File.write(File.join(extension_dir, "extconf.rb"), "")

        spec = Gem::Specification.new do |gem_spec|
          gem_spec.name = "example-extension"
          gem_spec.version = "0.1.0"
          gem_spec.files = Dir["ext/**/*"]
          gem_spec.extensions = [File.join(extension_dir, "extconf.rb")]
        end

        yield Dir.pwd, spec
      end
    end
  end
end
