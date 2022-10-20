const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const toml = require('toml');
const CopyWebpackPlugin = require('copy-webpack-plugin');
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

  const gitRegex = /^git.*github\.com\/(?<repo>.*)\?.*#(?<rev>[\da-f]{40})$/;
  const gitMatch = packageInfo.source?.match(gitRegex);
  const rev = gitMatch?.groups?.rev;
  const rawGithubRepo = gitMatch?.groups?.repo;
  const githubRepo = (rawGithubRepo?.endsWith('.git') ?? false)
    ? rawGithubRepo.substring(0, rawGithubRepo.length - 4)
    : rawGithubRepo;

  return {
    version: packageInfo.version,
    rev,
    githubRepo,
  };
}

function getGitInfo() {
  const gitOutput = execSync('git status --porcelain=v2 --branch', {
    encoding: 'utf8',
  });
  const gitOutputLines = gitOutput.split('\n');
  const commitLine = gitOutputLines.find((line) => line.startsWith('# branch.oid'));
  const commitHash = commitLine.match(/\b(?<hash>[\da-f]{40})$/).groups.hash;
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
    filename: '_assets/js/[name].js',
    chunkFilename: '_assets/js/[name].[chunkhash:8].js',
    webassemblyModuleFilename: '_assets/js/[hash].module.wasm',
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
      {
        test: /\.(woff|woff2)$/i,
        type: 'asset',
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
    new CopyWebpackPlugin({
      patterns: [{
        from: './webpack/favicon',
        to: '_assets/css/[name][ext]',
      }],
    }),
    new MiniCssExtractPlugin({
      filename: '_assets/css/[name].css',
    }),
    new WasmPackPlugin({
      crateDirectory: '.',
      extraArgs: '--no-typescript',
    }),
    ...htmlPlugins,
  ],
};
