initSidebarItems({"fn":[["enumerate_ordered_header_block_indices","This iterator tells you the block indices of all blocks that must be in the image. The order of the blocks depends on the `LineOrder` attribute (unspecified line order is treated the same as increasing line order). The blocks written to the file must be exactly in this order, except for when the `LineOrder` is unspecified. The index represents the block index, in increasing line order, within the header."],["read","Immediately reads the meta data from the file. Then, returns a reader that can be used to read all pixel blocks. From the reader, you can pull each compressed chunk from the file. Alternatively, you can create a decompressor, and pull the uncompressed data from it. The reader is assumed to be buffered."],["write","Immediately writes the meta data to the file. Then, calls a closure with a writer that can be used to write all pixel blocks. In the closure, you can push compressed chunks directly into the writer. Alternatively, you can create a compressor, wrapping the writer, and push the uncompressed data to it. The writer is assumed to be buffered."]],"mod":[["chunk","Read and write already compressed pixel data blocks. Does not include the process of compression and decompression."],["lines","Extract lines from a block of pixel bytes."],["reader","Composable structures to handle reading an image."],["samples","Extract pixel samples from a block of pixel bytes."],["writer","Composable structures to handle writing an image."]],"struct":[["BlockIndex","Specifies where a block of pixel data should be placed in the actual image. This is a globally unique identifier which includes the layer, level index, and pixel location."],["UncompressedBlock","Contains a block of pixel data and where that data should be placed in the actual image."]]});