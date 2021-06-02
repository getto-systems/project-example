/* eslint-disable */
module.exports = {
    isProduction,
}

function isProduction() {
    return process.env.BUILD_ENV === "production"
}
