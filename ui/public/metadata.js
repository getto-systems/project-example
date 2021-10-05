const headers = {
  "strict-transport-security": "max-age=31536000",
  "content-security-policy": [
    "default-src 'none'",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'self'",
    "connect-src " + [
      "'self'",
      "https://api.example.getto.systems/",
    ].join(" "),
    "frame-src 'none'",
    "frame-ancestors 'none'",
    "img-src 'self'",
    "font-src 'self'",
    "manifest-src 'self'",
    "script-src " + [
      "'self'",
      "https://example.getto.systems/",
    ].join(" "),
    "worker-src blob:",
    "style-src " + [
      "'self'",
      "https://trellis.getto.systems/css/",
    ].join(" "),
  ].join(";"),
  "x-content-type-options": "nosniff",
  "x-frame-options": "DENY",
  "x-xss-protection": "1; mode=block",
  "referrer-policy": "same-origin",
};

console.log(JSON.stringify(Object.keys(headers).reduce((acc,key) => {
  acc["header-" + key] = headers[key];
  return acc;
}, {})));
