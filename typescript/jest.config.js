module.exports = {
  preset: "ts-jest",
  globals: {
    "ts-jest": {
      isolatedModules: true,
      diagnostics: false
    }
  },
  testEnvironment: "jsdom",
  collectCoverage: true,
  testMatch: ["<rootDir>/src/**/__tests__/**/*.{ts,tsx}"],
  testPathIgnorePatterns: ["/lib/"],
  moduleNameMapper: {
    "\\.(css|less|sass|scss)$": "<rootDir>/jest/style-mock.js",
    "typeface-.*$": "<rootDir>/jest/style-mock.js"
  }
};
