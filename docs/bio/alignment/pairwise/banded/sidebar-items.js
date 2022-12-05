initSidebarItems({"struct":[["Aligner","A banded implementation of Smith-Waterman aligner (SWA). Unlike the full SWA, this implementation computes the alignment between a pair of sequences only inside a ‘band’ withing the dynamic programming matrix. The band is constructed using the Sparse DP routine (see sparse::sdpkpp), which uses kmer matches to build the best common subsequence (including gap penalties) between the two strings. The band is constructed around this subsequence (using the window length ‘w’), filling in the gaps."]]});