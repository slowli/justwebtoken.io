const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const pug = require('pug');
const toml = require('toml');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const pages = require('./templates/pages.json');
const cargoLockfile = toml.parse(fs.readFileSync('./Cargo.lock', { encoding: 'utf-8' }));
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

module.exports = {
  entry: {
    index: './webpack/index.js',
    verify: './webpack/verify.js',
    about: './webpack/about.js'
  },
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
    new CopyWebpackPlugin({
      patterns: [
        {
          from: './templates/*.pug',
          globOptions: { ignore: ['**/_*.pug'] },

          to({ absoluteFilename }) {
            const isIndex = absoluteFilename.endsWith('index.pug');
            return isIndex ? 'index.html' : '[name]/index.html';
          },

          toType: 'template',

          transform(content, path) {
            const render = pug.compile(content, {
              filename: path
            });
            return render({ $pages: pages, $buildInfo: buildInfo });
          }
        }
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
    })
  ]
};
