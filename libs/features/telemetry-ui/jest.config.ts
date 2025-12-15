/* eslint-disable */
export default {
  displayName: "feat-telemetry-ui",
  preset: "../../../jest.preset.js",
  transform: {
    "^.+\\.[tj]sx?$": ["ts-jest", { tsconfig: "<rootDir>/tsconfig.spec.json" }],
  },
  moduleFileExtensions: ["ts", "tsx", "js", "jsx"],
  coverageDirectory: "../../../coverage/libs/features/telemetry-ui",
};
