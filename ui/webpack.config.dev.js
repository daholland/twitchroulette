const { merge } = require('webpack-merge');
const common = require('./webpack.config.common.js');

module.exports = merge(common, {
  mode: 'development',
  devtool: 'inline-source-map',
  devServer: {
    watchFiles: ['src/*', 'src/**/*', 'dist/*', 'dist/**/*'],
    historyApiFallback: true,
  },
});
