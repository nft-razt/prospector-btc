import '@testing-library/jest-dom';

// Mock global de TextEncoder para entornos JSDOM que no lo tienen nativo en algunas versiones de Node
if (typeof global.TextEncoder === 'undefined') {
  const { TextEncoder, TextDecoder } = require('util');
  global.TextEncoder = TextEncoder;
  global.TextDecoder = TextDecoder;
}
