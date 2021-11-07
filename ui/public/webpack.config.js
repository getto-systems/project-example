/* eslint-disable */
const path = require("path")

const WorkerPlugin = require("worker-plugin")
const TerserPlugin = require("terser-webpack-plugin")

const environment = require("../env/environment")
const entryPoint = require("../env/entry_point")

module.exports = {
    entry: entryPoint.publicEntries(),
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
        static: {
            directory: path.join(__dirname, "dist"),
            publicPath: "/dist/",
        },

        host: "0.0.0.0",
        port: process.env.PUBLIC_APP_PORT,

        proxy: [
            {
                context: ["/dist/auth", "/dist/avail"],
                target: `http://localhost:${process.env.PUBLIC_APP_PORT}`,
                pathRewrite: { '^/dist': '' },
            },
        ],

        hot: true,
        client: {
            webSocketURL: `wss://${new URL(process.env.PUBLIC_SERVER_URL).host}/ws`,
        },
        allowedHosts: "all",
    },
}
