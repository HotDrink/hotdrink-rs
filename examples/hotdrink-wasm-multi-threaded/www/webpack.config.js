const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: {
    bundle: "./index.js",
  },
  // output: {
  //   path: path.resolve(__dirname, "dist"),
  //   filename: "index.js",
  // },
  mode: "development",
  plugins: [new CopyWebpackPlugin(["index.html"])],
  module: {
    rules: [
      {
        test: /\.js$/,
        enforce: "pre",
        use: ["source-map-loader"],
      },
    ],
  },
  // Set headers for Firefox to work
  devServer: {
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin'
    }
  }
};
