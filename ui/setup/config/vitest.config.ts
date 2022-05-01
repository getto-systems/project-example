import { defineConfig } from "vitest/config"

export default defineConfig({
    test: {
        include: ["**/test.ts", "**/test.*.ts"],
        root: "../../../src",
        coverage: {
            enabled: true,
            reporter: ["text", "html"],
            reportsDirectory: "ui/public/dist/coverage/ui",
            clean: true,
            functions: 100,
            include: [
                "**/*.ts",
            ],
            exclude: [
                "**/x_*/**",
                "**/y_*/**",
                "z_vendor/base64/**",
                "z_vendor/getto-application/action/worker/**",
                "z_vendor/getto-application/board/kernel/convert.ts",
                "z_vendor/getto-application/docs/helper.ts",
                "z_vendor/getto-css/**",
                "z_vendor/getto-table/**",
                "z_vendor/protobuf/**",
                "**/test.ts",
                "**/test_helper.ts",
                "**/menu/**",
                "**/init/**",
                "**/init.ts",
                "**/view.ts",
                "**/infra.ts",
                "**/mock.ts",
                "**/docs.ts",
                "**/site.ts",
            ],
        },
    },
})
