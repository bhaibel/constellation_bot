require 'ffi'

module ConstellationBot
  class Constellation

    def to_svg
      Builder::Binding.generate(1)
    end

    class Builder < FFI::AutoPointer
      def self.release(ptr)
        Binding.free(ptr)
      end

      private

      def to_s
        @str ||= self.read_string.force_encoding('UTF-8')
      end

      module Binding
        extend FFI::Library
        FFI_EXT = RUBY_PLATFORM.include?('darwin') ? 'dylib' : 'so'

        ffi_lib 'target/release/libconstellation.' + FFI_EXT

        attach_function :generate, :theme_song_generate,
                        [:uint8], Builder
        attach_function :free, :theme_song_free,
                        [Builder], :void
      end
    end

    private_constant :Builder
  end

  private_constant :Constellation
end