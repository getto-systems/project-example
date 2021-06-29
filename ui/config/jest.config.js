/* eslint-disable */
module.exports = {
    preset: "ts-jest",
    testEnvironment: "node",
    rootDir: "../..",
    collectCoverage: true,
    collectCoverageFrom: [
        "**/*.ts",
        "!ui/**",
        "!**/x_*/**",
        "!**/y_*/**",
        "!**/z_*/**",
        "!**/test.ts",
        "!**/test_helper.ts",
        "!**/init/**",
        "!**/init.ts",
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
