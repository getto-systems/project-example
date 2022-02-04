/* eslint-disable */
const path = require("path")

module.exports = {
    publicEntries,
    secureEntries,
    linkableHtmlFiles,
    secureHtmlFiles,
}

const entries = {
    public: [
        { name: "avail/version/move-to-latest" },
        { name: "avail/version/move-to-next" },
        { name: "avail/not-found" },
        { name: "auth/sign", worker: true },
    ],
    secure: [
        { name: "index", /* TODO worker: true */ },

        { name: "auth/ticket/logout" },
        { name: "auth/profile", worker: true },
        { name: "auth/user/account", /* TODO worker: true */ },

        { name: "core/season/setup", /* TODO worker: true */ },

        { name: "docs/index" },
        { name: "docs/privacy-policy" },

        { name: "docs/auth/index" },
        { name: "docs/auth/ticket/logout" },
        { name: "docs/auth/ticket/check" },
        { name: "docs/auth/password/authenticate" },
        { name: "docs/auth/password/reset" },
        { name: "docs/auth/password/change" },

        { name: "docs/avail" },
        { name: "docs/docs" },
    ],
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
        "coverage/ui/lcov-report/index.html",
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
