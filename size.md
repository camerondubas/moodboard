// Note: lto true for these
Dev: 
Baseline: 25.49MB
lto true & opt-level 's': 18.76MB
lto true & opt-level 'z': 16.28MB

lto 'thin' & opt-level 's':
lto 'thin' & opt-level 'z': 21.36MB

lto true & opt-level 'z' & wasm-opt 's': 11.23MB
lto true & opt-level 'z' & wasm-opt 'z': 10.30MB
lto true & opt-level 'z' & wasm-opt 'z' & strip & panic 'abort': 10.31MB
lto 'thin' & opt-level 's' & wasm-opt 'z' & strip : 13.53MB
