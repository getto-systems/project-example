/* eslint-disable */
const fs = require("fs")
const path = require("path")

fs.writeFileSync(path.join(__dirname, "../../sqlx-data.json"), JSON.stringify({
    ...JSON.parse(readData("auth")),
    ...JSON.parse(readData("example")),
}))

function readData(target) {
    return fs.readFileSync(path.join(__dirname, `../../sqlx-data.${target}.json`), {encoding: "utf-8"})
}
