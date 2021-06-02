/* eslint-disable */
const path = require("path")

const WorkerPlugin = require("worker-plugin")
const TerserPlugin = require("terser-webpack-plugin")

const environment = require("../env/environment")
const entryPoint = require("../env/entry_point")

module.exports = {
    entry: entryPoint.findPublicEntries(),
    output: {
        path: path.join(__dirname, "./dist"),
        filename: "[name].js",
        globalObject: "self",
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
        port: process.env.PUBLIC_APP_PORT,

        hot: true,
        sockPort: "443",
        disableHostCheck: true,
    },
}
