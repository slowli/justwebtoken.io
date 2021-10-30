const fs = require('fs');
const path = require('path');
const pug = require('pug');
const toml = require('toml');
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

function createBuildInfo() {
  return {
    deps: {
      'jwt-compact': getDependencyInfo('jwt-compact'),
      yew: getDependencyInfo('yew')
    },
    commit: '??????' // FIXME: use real commit
  };
}

module.exports = {
  entry: { index: './index.js' },
  output: {
    path: distPath,
    filename: '[name].js',
    chunkFilename: '[name].[chunkhash:8].js'
  },
  experiments: {
    asyncWebAssembly: true
  },
  optimization: {
    splitChunks: {
      chunks: 'async',
      cacheGroups: {
        vendors: false // disable splitting the main chunk into 3rd-party and built-in parts
      }
    }
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: './static', to: distPath },
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
