function copy_audio_buffer(dest, src, channel) {
    // Turn the array view into owned memory.
    var standalone = [...src];
    // Make it a Float32Array.
    var buffer = new Float32Array(standalone);

    // Copy the data.
    dest.copyToChannel(buffer, channel);
}

if (typeof exports === 'object' && typeof module === 'object')
    module.exports = copy_audio_buffer;
else if (typeof define === 'function' && define['amd'])
    define([], function() { return copy_audio_buffer; });
else if (typeof exports === 'object')
    exports["copy_audio_buffer"] = copy_audio_buffer;
