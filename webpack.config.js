const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const toml = require('toml');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const AutoprefixerPlugin = require('autoprefixer');

const pages = require('./webpack/pages.json');

function requireToml(pathToToml) {
  return toml.parse(fs.readFileSync(pathToToml, { encoding: 'utf-8' }));
}

const standardFields = requireToml('./src/fields.toml');
const cargoLockfile = requireToml('./Cargo.lock');
const distPath = path.resolve(__dirname, 'dist');

function getDependencyInfo(dependencyName) {
  const packageInfo = cargoLockfile.package.find(({ name }) => name === dependencyName);
  return {
    version: packageInfo.version,
  };
}

// TODO: get this info in CD builds
function getGitInfo() {
  const gitOutput = execSync('git status --porcelain=v2 --branch', {
    encoding: 'utf8',
  });
  const gitOutputLines = gitOutput.split('\n');
  const commitLine = gitOutputLines.find((line) => line.startsWith('# branch.oid'));
  const commitHash = commitLine.match(/\b(?<hash>[0-9a-f]{40})$/).groups.hash;
  const isDirty = gitOutputLines.some((line) => line.startsWith('1 ') || line.startsWith('2 '));
  return { commitHash, isDirty };
}

const buildInfo = {
  deps: {
    'jwt-compact': getDependencyInfo('jwt-compact'),
    yew: getDependencyInfo('yew'),
  },
  git: getGitInfo(),
};

const entries = {
  index: './webpack/index.js',
  verify: './webpack/verify.js',
  claims: './webpack/claims.js',
  about: './webpack/about.js',
};

const htmlPlugins = Object.keys(entries).map((entry) => new HtmlWebpackPlugin({
  filename: entry === 'index' ? 'index.html' : `${entry}/index.html`,
  chunks: [entry, 'commons'],
  template: `webpack/templates/${entry}.pug`,
  templateParameters: {
    $pages: pages,
    $standardFields: standardFields,
    $buildInfo: buildInfo,
  },
}));

module.exports = {
  entry: entries,
  output: {
    path: distPath,
    filename: '[name].js',
    chunkFilename: '[name].[chunkhash:8].js',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader'],
      },
      {
        test: /\.scss$/i,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                plugins: [AutoprefixerPlugin],
              },
            },
          },
          'sass-loader',
        ],
      },
      {
        test: /\.pug$/i,
        loader: 'pug-loader',
      },
    ],
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendors: false, // disable splitting the main chunk into 3rd-party and built-in parts
        commons: {
          name: 'commons',
          chunks: 'initial',
          minChunks: 2,
        },
      },
    },
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new WasmPackPlugin({
      crateDirectory: '.',
      extraArgs: '--no-typescript',
    }),
    ...htmlPlugins,
  ],
};
