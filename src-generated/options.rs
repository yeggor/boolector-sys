/* automatically generated by rust-bindgen 0.59.1 */

pub const BTOR_OPT_MODEL_GEN: BtorOption = 0;
pub const BTOR_OPT_INCREMENTAL: BtorOption = 1;
pub const BTOR_OPT_INCREMENTAL_SMT1: BtorOption = 2;
pub const BTOR_OPT_INPUT_FORMAT: BtorOption = 3;
pub const BTOR_OPT_OUTPUT_NUMBER_FORMAT: BtorOption = 4;
pub const BTOR_OPT_OUTPUT_FORMAT: BtorOption = 5;
pub const BTOR_OPT_ENGINE: BtorOption = 6;
pub const BTOR_OPT_SAT_ENGINE: BtorOption = 7;
pub const BTOR_OPT_AUTO_CLEANUP: BtorOption = 8;
pub const BTOR_OPT_PRETTY_PRINT: BtorOption = 9;
pub const BTOR_OPT_EXIT_CODES: BtorOption = 10;
pub const BTOR_OPT_SEED: BtorOption = 11;
pub const BTOR_OPT_VERBOSITY: BtorOption = 12;
pub const BTOR_OPT_LOGLEVEL: BtorOption = 13;
pub const BTOR_OPT_REWRITE_LEVEL: BtorOption = 14;
pub const BTOR_OPT_SKELETON_PREPROC: BtorOption = 15;
pub const BTOR_OPT_ACKERMANN: BtorOption = 16;
pub const BTOR_OPT_BETA_REDUCE: BtorOption = 17;
pub const BTOR_OPT_ELIMINATE_SLICES: BtorOption = 18;
pub const BTOR_OPT_VAR_SUBST: BtorOption = 19;
pub const BTOR_OPT_UCOPT: BtorOption = 20;
pub const BTOR_OPT_MERGE_LAMBDAS: BtorOption = 21;
pub const BTOR_OPT_EXTRACT_LAMBDAS: BtorOption = 22;
pub const BTOR_OPT_NORMALIZE: BtorOption = 23;
pub const BTOR_OPT_NORMALIZE_ADD: BtorOption = 24;
pub const BTOR_OPT_FUN_PREPROP: BtorOption = 25;
pub const BTOR_OPT_FUN_PRESLS: BtorOption = 26;
pub const BTOR_OPT_FUN_DUAL_PROP: BtorOption = 27;
pub const BTOR_OPT_FUN_DUAL_PROP_QSORT: BtorOption = 28;
pub const BTOR_OPT_FUN_JUST: BtorOption = 29;
pub const BTOR_OPT_FUN_JUST_HEURISTIC: BtorOption = 30;
pub const BTOR_OPT_FUN_LAZY_SYNTHESIZE: BtorOption = 31;
pub const BTOR_OPT_FUN_EAGER_LEMMAS: BtorOption = 32;
pub const BTOR_OPT_FUN_STORE_LAMBDAS: BtorOption = 33;
pub const BTOR_OPT_PRINT_DIMACS: BtorOption = 34;
pub const BTOR_OPT_SLS_NFLIPS: BtorOption = 35;
pub const BTOR_OPT_SLS_STRATEGY: BtorOption = 36;
pub const BTOR_OPT_SLS_JUST: BtorOption = 37;
pub const BTOR_OPT_SLS_MOVE_GW: BtorOption = 38;
pub const BTOR_OPT_SLS_MOVE_RANGE: BtorOption = 39;
pub const BTOR_OPT_SLS_MOVE_SEGMENT: BtorOption = 40;
pub const BTOR_OPT_SLS_MOVE_RAND_WALK: BtorOption = 41;
pub const BTOR_OPT_SLS_PROB_MOVE_RAND_WALK: BtorOption = 42;
pub const BTOR_OPT_SLS_MOVE_RAND_ALL: BtorOption = 43;
pub const BTOR_OPT_SLS_MOVE_RAND_RANGE: BtorOption = 44;
pub const BTOR_OPT_SLS_MOVE_PROP: BtorOption = 45;
pub const BTOR_OPT_SLS_MOVE_PROP_N_PROP: BtorOption = 46;
pub const BTOR_OPT_SLS_MOVE_PROP_N_SLS: BtorOption = 47;
pub const BTOR_OPT_SLS_MOVE_PROP_FORCE_RW: BtorOption = 48;
pub const BTOR_OPT_SLS_MOVE_INC_MOVE_TEST: BtorOption = 49;
pub const BTOR_OPT_SLS_USE_RESTARTS: BtorOption = 50;
pub const BTOR_OPT_SLS_USE_BANDIT: BtorOption = 51;
pub const BTOR_OPT_PROP_NPROPS: BtorOption = 52;
pub const BTOR_OPT_PROP_USE_RESTARTS: BtorOption = 53;
pub const BTOR_OPT_PROP_USE_BANDIT: BtorOption = 54;
pub const BTOR_OPT_PROP_PATH_SEL: BtorOption = 55;
pub const BTOR_OPT_PROP_PROB_USE_INV_VALUE: BtorOption = 56;
pub const BTOR_OPT_PROP_PROB_FLIP_COND: BtorOption = 57;
pub const BTOR_OPT_PROP_PROB_FLIP_COND_CONST: BtorOption = 58;
pub const BTOR_OPT_PROP_FLIP_COND_CONST_DELTA: BtorOption = 59;
pub const BTOR_OPT_PROP_FLIP_COND_CONST_NPATHSEL: BtorOption = 60;
pub const BTOR_OPT_PROP_PROB_SLICE_KEEP_DC: BtorOption = 61;
pub const BTOR_OPT_PROP_PROB_CONC_FLIP: BtorOption = 62;
pub const BTOR_OPT_PROP_PROB_SLICE_FLIP: BtorOption = 63;
pub const BTOR_OPT_PROP_PROB_EQ_FLIP: BtorOption = 64;
pub const BTOR_OPT_PROP_PROB_AND_FLIP: BtorOption = 65;
pub const BTOR_OPT_PROP_NO_MOVE_ON_CONFLICT: BtorOption = 66;
pub const BTOR_OPT_AIGPROP_USE_RESTARTS: BtorOption = 67;
pub const BTOR_OPT_AIGPROP_USE_BANDIT: BtorOption = 68;
pub const BTOR_OPT_QUANT_SYNTH: BtorOption = 69;
pub const BTOR_OPT_QUANT_DUAL_SOLVER: BtorOption = 70;
pub const BTOR_OPT_QUANT_SYNTH_LIMIT: BtorOption = 71;
pub const BTOR_OPT_QUANT_SYNTH_QI: BtorOption = 72;
pub const BTOR_OPT_QUANT_DER: BtorOption = 73;
pub const BTOR_OPT_QUANT_CER: BtorOption = 74;
pub const BTOR_OPT_QUANT_MINISCOPE: BtorOption = 75;
pub const BTOR_OPT_SORT_EXP: BtorOption = 76;
pub const BTOR_OPT_SORT_AIG: BtorOption = 77;
pub const BTOR_OPT_SORT_AIGVEC: BtorOption = 78;
pub const BTOR_OPT_AUTO_CLEANUP_INTERNAL: BtorOption = 79;
pub const BTOR_OPT_SIMPLIFY_CONSTRAINTS: BtorOption = 80;
pub const BTOR_OPT_CHK_FAILED_ASSUMPTIONS: BtorOption = 81;
pub const BTOR_OPT_CHK_MODEL: BtorOption = 82;
pub const BTOR_OPT_CHK_UNCONSTRAINED: BtorOption = 83;
pub const BTOR_OPT_PARSE_INTERACTIVE: BtorOption = 84;
pub const BTOR_OPT_SAT_ENGINE_LGL_FORK: BtorOption = 85;
pub const BTOR_OPT_SAT_ENGINE_CADICAL_FREEZE: BtorOption = 86;
pub const BTOR_OPT_SAT_ENGINE_N_THREADS: BtorOption = 87;
pub const BTOR_OPT_SIMP_NORMAMLIZE_ADDERS: BtorOption = 88;
pub const BTOR_OPT_DECLSORT_BV_WIDTH: BtorOption = 89;
pub const BTOR_OPT_QUANT_SYNTH_ITE_COMPLETE: BtorOption = 90;
pub const BTOR_OPT_QUANT_FIXSYNTH: BtorOption = 91;
pub const BTOR_OPT_RW_ZERO_LOWER_SLICE: BtorOption = 92;
pub const BTOR_OPT_NONDESTR_SUBST: BtorOption = 93;
pub const BTOR_OPT_NUM_OPTS: BtorOption = 94;
pub type BtorOption = ::std::os::raw::c_uint;
pub const BTOR_SAT_ENGINE_LINGELING: BtorOptSatEngine = 0;
pub const BTOR_SAT_ENGINE_PICOSAT: BtorOptSatEngine = 1;
pub const BTOR_SAT_ENGINE_MINISAT: BtorOptSatEngine = 2;
pub const BTOR_SAT_ENGINE_CADICAL: BtorOptSatEngine = 3;
pub const BTOR_SAT_ENGINE_CMS: BtorOptSatEngine = 4;
pub type BtorOptSatEngine = ::std::os::raw::c_uint;
pub const BTOR_ENGINE_FUN: BtorOptEngine = 1;
pub const BTOR_ENGINE_SLS: BtorOptEngine = 2;
pub const BTOR_ENGINE_PROP: BtorOptEngine = 3;
pub const BTOR_ENGINE_AIGPROP: BtorOptEngine = 4;
pub const BTOR_ENGINE_QUANT: BtorOptEngine = 5;
pub type BtorOptEngine = ::std::os::raw::c_uint;
pub const BTOR_INPUT_FORMAT_NONE: BtorOptInputFormat = 0;
pub const BTOR_INPUT_FORMAT_BTOR: BtorOptInputFormat = 1;
pub const BTOR_INPUT_FORMAT_BTOR2: BtorOptInputFormat = 2;
pub const BTOR_INPUT_FORMAT_SMT1: BtorOptInputFormat = 3;
pub const BTOR_INPUT_FORMAT_SMT2: BtorOptInputFormat = 4;
pub type BtorOptInputFormat = ::std::os::raw::c_uint;
pub const BTOR_OUTPUT_BASE_BIN: BtorOptOutputBase = 1;
pub const BTOR_OUTPUT_BASE_HEX: BtorOptOutputBase = 2;
pub const BTOR_OUTPUT_BASE_DEC: BtorOptOutputBase = 3;
pub type BtorOptOutputBase = ::std::os::raw::c_uint;
pub const BTOR_OUTPUT_FORMAT_NONE: BtorOptOutputFormat = 0;
pub const BTOR_OUTPUT_FORMAT_BTOR: BtorOptOutputFormat = 1;
pub const BTOR_OUTPUT_FORMAT_SMT2: BtorOptOutputFormat = 2;
pub const BTOR_OUTPUT_FORMAT_AIGER_ASCII: BtorOptOutputFormat = 3;
pub const BTOR_OUTPUT_FORMAT_AIGER_BINARY: BtorOptOutputFormat = 4;
pub type BtorOptOutputFormat = ::std::os::raw::c_uint;
pub const BTOR_DP_QSORT_JUST: BtorOptDPQsort = 1;
pub const BTOR_DP_QSORT_ASC: BtorOptDPQsort = 2;
pub const BTOR_DP_QSORT_DESC: BtorOptDPQsort = 3;
pub type BtorOptDPQsort = ::std::os::raw::c_uint;
pub const BTOR_JUST_HEUR_BRANCH_LEFT: BtorOptJustHeur = 1;
pub const BTOR_JUST_HEUR_BRANCH_MIN_APP: BtorOptJustHeur = 2;
pub const BTOR_JUST_HEUR_BRANCH_MIN_DEP: BtorOptJustHeur = 3;
pub type BtorOptJustHeur = ::std::os::raw::c_uint;
pub const BTOR_SLS_STRAT_BEST_MOVE: BtorOptSLSStrat = 1;
pub const BTOR_SLS_STRAT_RAND_WALK: BtorOptSLSStrat = 2;
pub const BTOR_SLS_STRAT_FIRST_BEST_MOVE: BtorOptSLSStrat = 3;
pub const BTOR_SLS_STRAT_BEST_SAME_MOVE: BtorOptSLSStrat = 4;
pub const BTOR_SLS_STRAT_ALWAYS_PROP: BtorOptSLSStrat = 5;
pub type BtorOptSLSStrat = ::std::os::raw::c_uint;
pub const BTOR_PROP_PATH_SEL_CONTROLLING: BtorOptPropPathSel = 1;
pub const BTOR_PROP_PATH_SEL_ESSENTIAL: BtorOptPropPathSel = 2;
pub const BTOR_PROP_PATH_SEL_RANDOM: BtorOptPropPathSel = 3;
pub type BtorOptPropPathSel = ::std::os::raw::c_uint;
pub const BTOR_QUANT_SYNTH_NONE: BtorOptQuantSynth = 0;
pub const BTOR_QUANT_SYNTH_EL: BtorOptQuantSynth = 1;
pub const BTOR_QUANT_SYNTH_ELMC: BtorOptQuantSynth = 2;
pub const BTOR_QUANT_SYNTH_EL_ELMC: BtorOptQuantSynth = 3;
pub const BTOR_QUANT_SYNTH_ELMR: BtorOptQuantSynth = 4;
pub type BtorOptQuantSynth = ::std::os::raw::c_uint;
pub use self::BtorOptQuantSynth as BtorOptQuantSynt;
pub const BTOR_FUN_EAGER_LEMMAS_NONE: BtorOptFunEagerLemmas = 0;
pub const BTOR_FUN_EAGER_LEMMAS_CONF: BtorOptFunEagerLemmas = 1;
pub const BTOR_FUN_EAGER_LEMMAS_ALL: BtorOptFunEagerLemmas = 2;
pub type BtorOptFunEagerLemmas = ::std::os::raw::c_uint;
pub const BTOR_INCREMENTAL_SMT1_BASIC: BtorOptIncrementalSMT1 = 1;
pub const BTOR_INCREMENTAL_SMT1_CONTINUE: BtorOptIncrementalSMT1 = 2;
pub type BtorOptIncrementalSMT1 = ::std::os::raw::c_uint;
pub const BTOR_BETA_REDUCE_NONE: BtorOptBetaReduceMode = 0;
pub const BTOR_BETA_REDUCE_FUN: BtorOptBetaReduceMode = 1;
pub const BTOR_BETA_REDUCE_ALL: BtorOptBetaReduceMode = 2;
pub type BtorOptBetaReduceMode = ::std::os::raw::c_uint;
