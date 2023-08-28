const path = require("path");
const fs = require("fs");
const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "public");
const wasm = path.resolve(__dirname, "pkg/bevy_demo_bg.wasm");

module.exports = {
  mode: "development",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devtool: "source-map",
  devServer: {
    static: {
      directory: dist,
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
    new CopyPlugin({
      patterns: [
        { from: wasm, to: dist },
      ]
    })
  ]
};