/* eslint-disable */
const fs = require("fs")
const path = require("path")

const entryPoint = require("../env/entry_point")

generateSecureHtmlFiles()

function generateSecureHtmlFiles() {
    const template = path.join(__dirname, "../public/template/site.html")
    const dist = path.join(__dirname, "../public/dist")

    entryPoint.secureHtmlFiles().forEach((file) => {
        fs.copyFile(template, path.join(dist, file), (err) => {
            if (err) {
                throw err
            }
        })
    })
}
