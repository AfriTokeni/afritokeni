module.exports = {
  default: {
    require: ['tests/e2e/steps/**/*.ts'],
    requireModule: ['ts-node/register'],
    format: ['progress', 'html:test-results/cucumber-report.html'],
    formatOptions: { snippetInterface: 'async-await' },
    paths: ['tests/e2e/features/**/*.feature'],
    publishQuiet: true,
  }
};
