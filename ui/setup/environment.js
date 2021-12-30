/* eslint-disable */
const fs = require("fs")
const path = require("path")

const environment = require("../env/environment")
const entryPoint = require("../env/entry_point")

const environmentRoot = path.join(__dirname, "../../src/y_environment/ui")
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
        apiServerNonceHeader: "GETTO-EXAMPLE-NONCE",

        database: {
            authProfile: "GETTO-EXAMPLE-AUTH-PROFILE",
            season: "GETTO-EXAMPLE-SEASON",
            menuExpand: "GETTO-EXAMPLE-MENU-EXPAND",
            searchColumns: "GETTO-EXAMPLE-SEARCH-COLUMNS",
        },
    }

    return "export const env = " + JSON.stringify(env, null, "    ")
}

function pathContent() {
    const files = entryPoint.linkableHtmlFiles()
    return [
        "export const staticMenuPath = " + JSON.stringify(files) + " as const",
        "export type StaticMenuPath = typeof staticMenuPath[number]",
    ].join("\n")
}

function dump(file, content) {
    console.log(file)
    console.log(content)
    fs.writeFileSync(file, content + "\n")
}
