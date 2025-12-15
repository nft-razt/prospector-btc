import "@testing-library/jest-dom";

// Polyfill para TextEncoder (necesario para algunas versiones de JSDOM/Node)
if (typeof global.TextEncoder === "undefined") {
  const { TextEncoder, TextDecoder } = require("util");
  global.TextEncoder = TextEncoder;
  global.TextDecoder = TextDecoder;
}
