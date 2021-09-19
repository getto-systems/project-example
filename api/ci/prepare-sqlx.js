/* eslint-disable */
const fs = require("fs")
const path = require("path")

const api = JSON.parse(readData("api"))
const auth = JSON.parse(readData("auth"))
const example = JSON.parse(readData("example"))

fs.writeFileSync(path.join(__dirname, "../../sqlx-data.json"), JSON.stringify({
    ...api,
    ...auth,
    ...example,
}))

function readData(target) {
    return fs.readFileSync(path.join(__dirname, `../../sqlx-data.${target}.json`), {encoding: "utf-8"})
}
