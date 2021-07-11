/* eslint-disable */
const fs = require("fs")
const path = require("path")

module.exports = {
    findPublicEntries,
    findHtmlFiles,
    findSecureEntries,
}

function findPublicEntries() {
    return [
        "avail/move-to-latest-version",
        "avail/move-to-next-version",
        "avail/not-found",
        "auth/sign",
    ].reduce(toEntry("public"), {})
}

function findHtmlFiles() {
    const root = path.join(__dirname, "../public/dist")
    return gatherFiles(root).map((file) => file.replace(root, ""))

    function gatherFiles(dir) {
        const files = []
        fs.readdirSync(dir, { withFileTypes: true }).forEach((file) => {
            if (file.isDirectory()) {
                if (isGatherDirectory(file.name)) {
                    gatherFiles(path.join(dir, file.name)).forEach((file) => {
                        files.push(file)
                    })
                }
            }
            if (file.isFile()) {
                if (file.name.endsWith(".html")) {
                    files.push(path.join(dir, file.name))
                }
            }
        })
        return files

        function isGatherDirectory(name) {
            const target = path.join(dir, name).replace(root, "")
            switch (target) {
                case "/coverage":
                case "/storybook":
                case "/css":
                case "/fonts":
                    return false

                default:
                    return true
            }
        }
    }
}
function findSecureEntries() {
    return findHtmlFiles()
        .map((file) => file.replace("/", "").replace(/\.html$/, ""))
        .reduce(toEntry("secure"), {})
}

function toEntry(root) {
    return (acc, name) => {
        acc[name] = toForegroundPath(name)

        const worker = toBackgroundPath(name)
        if (exists(worker)) {
            acc[`${name}.worker`] = worker
        }
        return acc
    }

    function toForegroundPath(file) {
        return toPath("foreground", file)
    }
    function toBackgroundPath(file) {
        return toPath("background", file)
    }
    function toPath(type, file) {
        return path.join(__dirname, "../../main/_ui", root, toEntryPath(file), `${type}.ts`)
    }
    function toEntryPath(file) {
        return file.replaceAll("-", "_")
    }

    function exists(file) {
        try {
            fs.accessSync(file)
            return true
        } catch (err) {
            return false
        }
    }
}
