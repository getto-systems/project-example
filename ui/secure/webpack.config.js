/* eslint-disable */
const path = require("path")

const WorkerPlugin = require("worker-plugin")
const TerserPlugin = require("terser-webpack-plugin")

const environment = require("../setup/env/environment")
const entryPoint = require("../setup/env/entry_point")

module.exports = {
    entry: entryPoint.secureEntries(),
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
        static: {
            directory: path.join(__dirname, "dist"),
            publicPath: "/dist/",
        },

        host: "0.0.0.0",
        port: process.env.SECURE_APP_PORT,

        proxy: [
            {
                context: ["/dist"],
                target: `http://localhost:${process.env.SECURE_APP_PORT}`,
                pathRewrite: { '^/dist': '' },
            },
        ],

        hot: true,
        client: {
            webSocketURL: `wss://${webSocketHost()}/ws`,
        },
        allowedHosts: "all",

        headers: {
            "Access-Control-Allow-Origin": "*",
            "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, PATCH, OPTIONS",
            "Access-Control-Allow-Headers": "X-Requested-With, content-type, Authorization",
        },
    },
}

function webSocketHost() {
    if (!process.env.SECURE_SERVER_URL) {
        return "localhost"
    }
    return new URL(process.env.SECURE_SERVER_URL).host
}
