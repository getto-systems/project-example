const headers = {
  "strict-transport-security": "max-age=31536000",
  "content-security-policy": [
    "default-src 'none'",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'self'",
    "frame-src 'none'",
    "frame-ancestors 'none'",
    "img-src 'self'",
    "font-src " + [
      "'self'",
      "https://fonts.googleapis.com/",
    ].join(" "),
    "manifest-src 'self'",
    "connect-src " + [
      "'self'",
      "https://api.example.getto.systems/",
      "https://secure.example.getto.systems/",
    ].join(" "),
    "script-src " + [
      "'self'",
      "https://secure.example.getto.systems/",
    ].join(" "),
    "child-src blob:",
    "worker-src blob:",
    "style-src " + [
      "'self'",
      "https://trellis.getto.systems/css/",
      "https://fonts.googleapis.com/",
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
