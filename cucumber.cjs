module.exports = {
  default: {
    import: ['tests/e2e/steps/**/*.ts'],
    loader: ['ts-node/esm'],
    format: ['progress', 'html:test-results/cucumber-report.html'],
    formatOptions: { snippetInterface: 'async-await' },
    paths: ['tests/e2e/features/**/*.feature'],
    publishQuiet: true,
  }
};
