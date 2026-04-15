# Code Coverage Report

This document tracks the code coverage statistics for the `qubit-mixin` crate.

[中文说明](COVERAGE.zh_CN.md)

## Latest Coverage

Last updated: 2025-01-XX

| Metric | Value |
|--------|-------|
| Lines Covered | TBD |
| Total Lines | TBD |
| Coverage Percentage | TBD% |

## Generating Coverage Report

To generate a coverage report locally, run:

```bash
./coverage.sh
```

This will:
1. Install `cargo-llvm-cov` if not already installed
2. Clean previous coverage data
3. Generate an LCOV report (`lcov.info`)
4. Generate an HTML report in `target/llvm-cov/html/`

Open `target/llvm-cov/html/index.html` in your browser to view the detailed coverage report.

## Coverage Goals

- **Target**: 90%+ line coverage
- **Minimum**: 80% line coverage

## Notes

- All public APIs should be covered by tests
- Focus on testing trait implementations and their interactions
- Edge cases and error conditions should be tested

