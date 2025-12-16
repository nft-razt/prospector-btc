export default {
  displayName: "infra-supabase",
  preset: "../../../jest.preset.js",
  testEnvironment: "node", // Supabase JS corre bien en Node
  transform: {
    "^.+\\.[tj]s$": ["ts-jest", { tsconfig: "<rootDir>/tsconfig.spec.json" }],
  },
  moduleFileExtensions: ["ts", "js", "html"],
  coverageDirectory: "../../../coverage/libs/infra/supabase",
};
