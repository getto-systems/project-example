/* eslint-disable */
const path = require("path")

const WorkerPlugin = require("worker-plugin")
const TerserPlugin = require("terser-webpack-plugin")

const environment = require("../env/environment")
const entryPoint = require("../env/entry_point")

module.exports = {
    entry: entryPoint.findSecureEntries(),
    output: {
        path: path.join(__dirname, "./dist"),
        filename: "[name].js",
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                loader: "ts-loader",
                resolve: {
                    extensions: [".ts"],
                },
            },
        ],
    },
    optimization: {
        minimize: environment.isProduction(),
        minimizer: [new TerserPlugin()],
    },
    plugins: [new WorkerPlugin()],
    watchOptions: {
        ignored: ["**/.git", "**/node_modules", "**/storybook"],
    },
    devServer: {
        contentBase: path.join(__dirname, "."),
        publicPath: "/dist/",

        host: "0.0.0.0",
        port: process.env.SECURE_APP_PORT,

        hot: true,
        sockPort: "443",
        sockHost: new URL(process.env.SECURE_SERVER_URL).host,

        disableHostCheck: true,
        headers: {
            "Access-Control-Allow-Origin": "*",
            "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
            "Access-Control-Allow-Headers": "X-Requested-With, content-type, Authorization",
        },
    },
}
