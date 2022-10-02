initSidebarItems({"enum":[["XYEmission",""]],"struct":[["HomopolyPairHMM","A pair Hidden Markov Model for comparing sequences x and y as described by Durbin, R., Eddy, S., Krogh, A., & Mitchison, G. (1998). Biological Sequence Analysis. Current Topics in Genome Analysis 2008. http://doi.org/10.1017/CBO9780511790492. The default model has been extended to consider homopolymer errors, at the cost of more states and transitions."],["PairHMM","A pair Hidden Markov Model for comparing sequences x and y as described by Durbin, R., Eddy, S., Krogh, A., & Mitchison, G. (1998). Biological Sequence Analysis. Current Topics in Genome Analysis 2008. http://doi.org/10.1017/CBO9780511790492."]],"trait":[["BaseSpecificHopParameters","Trait for parametrization of `PairHMM` hop behavior."],["Emission","Trait needed for the `HomopolyPairHMM`, because its implementation details depend on the actual bases to distinguish between Match states."],["EmissionParameters","Trait for parametrization of `PairHMM` emission behavior."],["GapParameters","Trait for parametrization of `PairHMM` gap behavior."],["HopParameters","Trait for parametrization of `PairHMM` hop behavior."],["StartEndGapParameters","Trait for parametrization of `PairHMM` start and end gap behavior. This trait can be used to implement global and semiglobal alignments."]]});