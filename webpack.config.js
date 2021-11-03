const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const toml = require('toml');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const AutoprefixerPlugin = require('autoprefixer');

function requireToml(path) {
  return toml.parse(fs.readFileSync(path, { encoding: 'utf-8' }));
}

const pages = require('./webpack/pages.json');
const standardFields = requireToml('./src/fields.toml');
const cargoLockfile = requireToml('./Cargo.lock');
const distPath = path.resolve(__dirname, 'dist');

const buildInfo = createBuildInfo();

function getDependencyInfo(dependencyName) {
  const packageInfo = cargoLockfile.package.find(({ name }) => name === dependencyName);
  return {
    version: packageInfo.version
  };
}

// TODO: get this info in CD builds
function getGitInfo() {
  const gitOutput = execSync('git status --porcelain=v2 --branch', {
    encoding: 'utf8'
  });
  const gitOutputLines = gitOutput.split('\n');
  const commitLine = gitOutputLines.find((line) => line.startsWith('# branch.oid'));
  const commitHash = commitLine.match(/\b(?<hash>[0-9a-f]{40})$/).groups.hash;
  const isDirty = gitOutputLines.some((line) => line.startsWith('1 ') || line.startsWith('2 '));
  return { commitHash, isDirty };
}

function createBuildInfo() {
  return {
    deps: {
      'jwt-compact': getDependencyInfo('jwt-compact'),
      yew: getDependencyInfo('yew')
    },
    git: getGitInfo()
  };
}

const entry = {
  index: './webpack/index.js',
  verify: './webpack/verify.js',
  claims: './webpack/claims.js',
  about: './webpack/about.js',
};

const htmlPlugins = Object.keys(entry).map((entry) => {
  return new HtmlWebpackPlugin({
    filename: entry === 'index' ? 'index.html' : `${entry}/index.html`,
    chunks: [entry, 'commons'],
    template: `webpack/templates/${entry}.pug`,
    templateParameters: {
      $pages: pages,
      $standardFields: standardFields,
      $buildInfo: buildInfo,
    },
  });
});

module.exports = {
  entry,
  output: {
    path: distPath,
    filename: '[name].js',
    chunkFilename: '[name].[chunkhash:8].js'
  },
  experiments: {
    asyncWebAssembly: true
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: [MiniCssExtractPlugin.loader, 'css-loader']
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
                plugins: [AutoprefixerPlugin]
              }
            }
          },
          'sass-loader'
        ]
      },
      {
        test: /\.pug$/i,
        loader: 'pug-loader'
      }
    ]
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendors: false, // disable splitting the main chunk into 3rd-party and built-in parts
        commons: {
          name: 'commons',
          chunks: 'initial',
          minChunks: 2
        }
      }
    }
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
    }),
    ...htmlPlugins
  ]
};
