/* eslint-disable */
const path = require("path")
const fs = require("fs")

module.exports = {
    publicEntries,
    secureEntries,
    linkableHtmlFiles,
    secureHtmlFiles,
}

const main = "../../../src/x_main/";
const entries = {
    public: JSON.parse(fs.readFileSync(path.join(__dirname, main, "public/entry_point.json"))),
    secure: JSON.parse(fs.readFileSync(path.join(__dirname, main, "secure/entry_point.json"))),
}

function publicEntries() {
    return entries.public.reduce((acc, entry) => ({ ...acc, ...toEntry("public", entry) }), {})
}
function secureEntries() {
    return entries.secure.reduce((acc, entry) => ({ ...acc, ...toEntry("secure", entry) }), {})
}
function secureHtmlFiles() {
    return entries.secure.map((entry) => `${entry.name}.html`)
}
function linkableHtmlFiles() {
    return [
        "coverage/api/index.html",
        "coverage/ui/index.html",
        ...secureHtmlFiles(),
    ]
}

function toEntry(root, entry) {
    if (entry.worker) {
        return workerEntry()
    } else {
        return simpleEntry()
    }

    function simpleEntry() {
        const map = {}
        map[entry.name] = toPath("entry")
        return map
    }
    function workerEntry() {
        const map = {}
        map[entry.name] = toPath("worker/foreground")
        map[`${entry.name}.worker`] = toPath("worker/background")
        return map
    }
    function toPath(type) {
        return path.join(__dirname, "../../../src/x_main", root, entryPath(), `${type}.ts`)
    }
    function entryPath() {
        return entry.name.replaceAll("-", "_")
    }
}
