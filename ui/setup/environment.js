/* eslint-disable */
const fs = require("fs")
const path = require("path")

const environment = require("../env/environment")
const entryPoint = require("../env/entry_point")

const environmentRoot = path.join(__dirname, "../../src/y_environment/_ui")
dump(path.join(environmentRoot, "env.ts"), envContent())
dump(path.join(environmentRoot, "path.ts"), pathContent())

function envContent() {
    const isProduction = environment.isProduction()
    const version = (() => {
        if (isProduction) {
            return fs.readFileSync(path.join(__dirname, "../VERSION"), "utf8").trim()
        } else {
            return "dist"
        }
    })()

    const env = {
        isProduction,

        version,
        versionSuffix: "-ui",

        secureServerURL: process.env.SECURE_SERVER_URL,
        apiServerURL: process.env.API_SERVER_URL,

        database: {
            authn: "GETTO-EXAMPLE-AUTHN",
            authz: "GETTO-EXAMPLE-AUTHZ",
            season: "GETTO-EXAMPLE-SEASON",
            menuExpand: "GETTO-EXAMPLE-MENU-EXPAND",
        },
    }

    return "export const env = " + JSON.stringify(env, null, "    ")
}

function pathContent() {
    const files = ["/storybook/index.html", "/coverage/api/index.html", "/coverage/ui/lcov-report/index.html"].concat(
        entryPoint.findHtmlFiles(),
    )
    return [
        "export const staticMenuPath = " + JSON.stringify(files) + " as const",
        "export type StaticMenuPath = typeof staticMenuPath[number]",
    ].join("\n")
    return ["export type StaticMenuPath =" + toTypeVariant(files)].join("\n")

    function toTypeVariant(files) {
        if (files.length === 0) {
            return " never"
        }
        const padding = "\n    | "
        return padding + files.map(JSON.stringify).join(padding)
    }
}

function dump(file, content) {
    console.log(file)
    console.log(content)
    fs.writeFileSync(file, content + "\n")
}
