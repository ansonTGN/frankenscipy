# SciPy Parity Coverage Report

Generated: 2026-05-25

## Overall Coverage: 69.1%

880 of 1274 SciPy callable symbols have FrankenSciPy equivalents.

## Module-Level Coverage

| Module | scipy | covered | Coverage |
|--------|-------|---------|----------|
| ndimage | 75 | 75 | 100.0% |
| linalg | 98 | 77 | 78.6% |
| fft | 41 | 31 | 75.6% |
| special | 357 | 259 | 72.5% |
| sparse | 53 | 37 | 69.8% |
| spatial | 18 | 12 | 66.7% |
| stats | 301 | 194 | 64.5% |
| signal | 156 | 99 | 63.5% |
| interpolate | 57 | 33 | 57.9% |
| optimize | 71 | 39 | 54.9% |
| integrate | 33 | 18 | 54.5% |
| io | 14 | 6 | 42.9% |

## Out-of-Scope Items

The following scipy features are intentionally out-of-scope for V1:

1. **BLAS/LAPACK internals**: `get_blas_funcs`, `get_lapack_funcs`, `find_best_blas_type` - FrankenSciPy uses pure Rust implementations
2. **Plotting utilities**: `convex_hull_plot_2d`, `voronoi_plot_2d`, `delaunay_plot_2d` - visualization out of scope
3. **Deprecated functions**: Functions marked deprecated in scipy
4. **Test utilities**: `test` functions, internal testing infrastructure
5. **Legacy formats**: Harwell-Boeing (`hb_read`/`hb_write`) - superseded by Matrix Market
6. **ODE class wrappers**: `ode`, `complex_ode` - functionality exists via `solve_ivp` with `SolverKind` enum
7. **Warning classes**: `IntegrationWarning`, `OdeintWarning` - Rust uses `Result` types

## Implemented But Different API

### sparse (69.8% coverage) ✓
- Matrix formats: `CsrMatrix`, `CscMatrix`, `CooMatrix`, `BsrMatrix`, `DiaMatrix`, `DokMatrix`, `LilMatrix`
- Type aliases: `csr_matrix`, `csr_array`, etc. all point to the corresponding structs
- Type checking: `issparse`, `isspmatrix`, `isspmatrix_csr`, etc.

### integrate (54.5% coverage)
- ODE solvers exist as `SolverKind::Rk45`, `SolverKind::Bdf`, `SolverKind::Lsoda`, etc.
- Quadrature: `quad`, `dblquad`, `tplquad`, `nquad`, `romberg`, `simpson`, `trapezoid`

### io (42.9% coverage)
- `loadmat`/`savemat` - implemented for MATLAB v4/v5
- `mmread`/`mmwrite` - Matrix Market format
- `wav_read`/`wav_write` - WAV audio
- `readsav` - IDL save files

## Notes

- Coverage counts include classes, functions, and type aliases
- Some functions have different names in FrankenSciPy (e.g., `solve_with_casp` vs `solve`)
- Many "missing" items are aliases or thin wrappers around implemented functionality
- All 0 open beads - remaining gaps are documented as out-of-scope or different API style
