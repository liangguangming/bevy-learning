const path = require("path");
const fs = require("fs");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const public = path.resolve(__dirname, "public");

const isDev = process.env.NODE_ENV === "develop";

const config = {
  mode: isDev? "development" : "production",
  entry: {
    index: "./web/index.js"
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js"
  },
  devServer: {
    static: {
      directory: public,
    },
  },
  experiments: {
    asyncWebAssembly: true,
  },
  plugins: [
    new HtmlWebpackPlugin(
      {
        templateContent: fs.readFileSync(path.resolve(__dirname, "public", "index.html")).toString()
      }
    ),
    // new WasmPackPlugin({
    //   crateDirectory: path.resolve(__dirname, "."),
    //               // Check https://rustwasm.github.io/wasm-pack/book/commands/build.html for
    //         // the available set of arguments.
    //         //
    //         // Optional space delimited arguments to appear before the wasm-pack
    //         // command. Default arguments are `--verbose`.
    //         args: '--log-level warn',
    //         // Default arguments are `--typescript --target browser --mode normal`.
    //         extraArgs: '--target web',

    //         // Optional array of absolute paths to directories, changes to which
    //         // will trigger the build.
    //         // watchDirectories: [
    //         //   path.resolve(__dirname, "another-crate/src")
    //         // ],

    //         // The same as the `--out-dir` option for `wasm-pack`
    //         // outDir: "pkg",

    //         // The same as the `--out-name` option for `wasm-pack`
    //         outName: "bevy_demo",

    //         // If defined, `forceWatch` will force activate/deactivate watch mode for
    //         // `.rs` files.
    //         //
    //         // The default (not set) aligns watch mode for `.rs` files to Webpack's
    //         // watch mode.
    //         // forceWatch: true,

    //         // If defined, `forceMode` will force the compilation mode for `wasm-pack`
    //         //
    //         // Possible values are `development` and `production`.
    //         //
    //         // the mode `development` makes `wasm-pack` build in `debug` mode.
    //         // the mode `production` makes `wasm-pack` build in `release` mode.
    //         forceMode: isDev ? "development" : "production",

    //         // Controls plugin output verbosity, either 'info' or 'error'.
    //         // Defaults to 'info'.
    //         // pluginLogLevel: 'info'
    // })
  ]
};

if (isDev) {
  config.devtool = "source-map";
}

module.exports = config;