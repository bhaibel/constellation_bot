require 'ffi'

module ConstellationBot
  class Constellation

    def to_svg(origin_x: 0, origin_y: 0, width: 480, height: 320)
      Builder::Binding.generate(origin_x, origin_y, width, height).to_s
    end

    class Builder < FFI::AutoPointer
      def self.release(ptr)
        Binding.free(ptr)
      end

      def to_s
        @str ||= self.read_string.force_encoding('UTF-8')
      end

      module Binding
        extend FFI::Library
        FFI_EXT = RUBY_PLATFORM.include?('darwin') ? 'dylib' : 'so'

        ffi_lib 'target/release/libconstellation.' + FFI_EXT

        attach_function :generate, :extern_constellation_svg,
                        [:uint32, :uint32, :uint32, :uint32], Builder
        attach_function :free, :extern_constellation_free,
                        [Builder], :void
      end
    end

    private_constant :Builder
  end

  private_constant :Constellation
end