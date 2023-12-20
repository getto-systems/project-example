import { defineConfig } from "vitest/config"

export default defineConfig({
    test: {
        include: ["**/test.ts", "**/test.*.ts"],
        root: ".",
        watch: false,
        coverage: {
            enabled: true,
            provider: "v8",
            reporter: ["text", "html"],
            reportsDirectory: "ui/public/dist/coverage/ui",
            clean: true,
            thresholds: { functions: 100 },
            include: ["**/*.ts"],
            exclude: [
                "**/x_*/**",
                "**/y_*/**",
                "src/docs/**",
                "src/common/util/protobuf/**",
                "src/common/util/worker/**",
                "src/z_vendor/getto-css/**",
                "src/z_vendor/getto-table/**",
                "**/test.ts",
                "**/test_helper.ts",
                "**/menu/**",
                "**/detail/**",
                "**/detail.ts",
                "**/data.ts",
                "**/infra.ts",
                "**/feature.ts",
                "**/mock.ts",
                "**/docs.ts",
                "**/site.ts",
            ],
        },
    },
})
