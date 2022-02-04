/* eslint-disable */
const fs = require("fs")
const path = require("path")

const entryPoint = require("./env/entry_point")

generateSecureHtmlFiles()

function generateSecureHtmlFiles() {
    const template = path.join(__dirname, "../public/template/site.html")
    const dist = path.join(__dirname, "../public/dist")

    cleanup()
    copy()

    function cleanup() {
        entryPoint.secureHtmlFiles().forEach((file) => {
            const tips = file.split("/")
            if (tips.length > 1) {
                fs.rmSync(path.join(dist, file.split("/")[0]), { force: true, recursive: true })
            }
        })
    }
    function copy() {
        entryPoint.secureHtmlFiles().forEach((file) => {
            fs.mkdirSync(path.join(dist, path.dirname(file)), { recursive: true })
            fs.copyFile(template, path.join(dist, file), (err) => {
                if (err) {
                    throw err
                }
            })
        })
    }
}
