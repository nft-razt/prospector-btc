/* eslint-disable */
export default {
  displayName: "core-client-vault",
  preset: "../../../jest.preset.js",
  // CRÍTICO: jsdom simula el navegador para Web Crypto API
  testEnvironment: "jsdom",
  transform: {
    "^.+\\.[tj]s$": ["ts-jest", { tsconfig: "<rootDir>/tsconfig.spec.json" }],
  },
  moduleFileExtensions: ["ts", "js", "html"],
  coverageDirectory: "../../../coverage/libs/core/client-vault",
  setupFilesAfterEnv: ["<rootDir>/src/test-setup.ts"], // Opcional si necesitamos polyfills extra
};
