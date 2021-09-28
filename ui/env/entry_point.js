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
        { name: "auth/sign", background: true },
    ],
    secure: [
        { name: "index", /* TODO background: true */ },

        { name: "auth/ticket/logout", /* TODO background: true */ },
        { name: "auth/profile", /* TODO background: true */ },

        { name: "docs/index" },
        { name: "docs/privacy-policy" },

        { name: "docs/auth/index" },
        { name: "docs/auth/logout" },
        { name: "docs/auth/auth-ticket/check" },
        { name: "docs/auth/password/authenticate" },
        { name: "docs/auth/password/reset" },

        { name: "docs/avail" },
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
        "storybook/index.html",
        "coverage/api/index.html",
        "coverage/ui/lcov-report/index.html",
        ...secureHtmlFiles(),
    ]
}

function toEntry(root, entry) {
    if (entry.background) {
        return {
            ...foregroundEntry(),
            ...backgroundEntry(),
        }
    } else {
        return foregroundEntry()
    }

    function foregroundEntry() {
        return buildEntry(entry.name, toPath("foreground"))
    }
    function backgroundEntry() {
        return buildEntry(`${entry.name}.worker`, toPath("background"))
    }
    function buildEntry(name, path) {
        const entry = {}
        entry[name] = path
        return entry
    }
    function toPath(type) {
        return path.join(__dirname, "../../main/ui", root, entryPath(), `${type}.ts`)
    }
    function entryPath() {
        return entry.name.replaceAll("-", "_")
    }
}
