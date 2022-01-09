/* eslint-disable */
module.exports = {
    preset: "ts-jest",
    testEnvironment: "node",
    testRegex: "/test(\\.[^/]*)?\\.ts$",
    rootDir: "../..",
    collectCoverage: true,
    collectCoverageFrom: [
        "**/*.ts",
        "!ui/**",
        "!main/**",
        "!**/x_*/**",
        "!**/y_*/**",
        "!**/z_*/**",
        "!**/test.ts",
        "!**/test_helper.ts",
        "!**/menu/**",
        "!**/init/**",
        "!**/init.ts",
        "!**/view.ts",
        "!**/infra.ts",
        "!**/mock.ts",
        "!**/docs.ts",
        "!**/site.ts",
    ],
    coverageThreshold: {
        global: {
            functions: 100,
        },
    },
    coverageDirectory: "ui/public/dist/coverage/ui",
}
