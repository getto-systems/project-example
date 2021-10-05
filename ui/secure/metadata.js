const headers = {
  "access-control-allow-origin": "https://example.getto.systems/",
  "strict-transport-security": "max-age=31536000",
  "content-security-policy": [
    "default-src 'none'",
    "object-src 'none'",
    "base-uri 'none'",
    "form-action 'self'",
    "connect-src 'self'",
    "frame-src 'none'",
    "frame-ancestors 'none'",
    "img-src 'self'",
    "font-src 'self'",
    "manifest-src 'self'",
    "script-src 'self'",
    "style-src 'self'",
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
