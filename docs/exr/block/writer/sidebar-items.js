initSidebarItems({"fn":[["write_chunks_with","Write an exr file by writing one chunk after another in a closure. In the closure, you are provided a chunk writer, which should be used to write all the chunks. Assumes the your write destination is buffered."]],"struct":[["ChunkWriter","Can consume compressed pixel chunks, writing them a file. Use `sequential_blocks_compressor` or `parallel_blocks_compressor` to compress your data, or use `compress_all_blocks_sequential` or `compress_all_blocks_parallel`. Use `on_progress` to obtain a new writer that triggers a callback for each block."],["OnProgressChunkWriter","A new writer that triggers a callback for each block written to the inner writer."],["ParallelBlocksCompressor","Compress blocks to a chunk writer with multiple threads."],["SequentialBlocksCompressor","Compress blocks to a chunk writer in this thread."],["SortedBlocksWriter","Write blocks that appear in any order and reorder them before writing."]],"trait":[["ChunksWriter","Write chunks to a byte destination. Then write each chunk with `writer.write_chunk(chunk)`."]]});