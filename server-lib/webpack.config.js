const path = require('path');

const SpritesmithPlugin = require('webpack-spritesmith');
const ExtractTextPlugin = require('extract-text-webpack-plugin');
const webpack = require('webpack');

module.exports = [{
    entry: './app/main.js',
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'dist')
    },
    module: {
        loaders: [
            { test: /\.vue$/,
                loader: 'vue-loader' },
            { test: /\.styl$/, loaders: [
                'style-loader',
                'css-loader',
                'stylus-loader'
            ] },
            { test: /\.png$/, loaders: [
                'file-loader?name=i/[hash].[ext]'
            ] }
        ]
    },
    resolve: {
        modules: ['node_modules', 'spritesmith-generated'],
        alias: {
            vue$: 'vue/dist/vue.esm.js'
        }
    },
    plugins: [
        new SpritesmithPlugin({
            src: {
                cwd: path.resolve(__dirname, 'app/images/'),
                glob: '**/*.png'
            },
            target: {
                image: path.resolve(__dirname, 'dist/sprite.png'),
                css: [[path.resolve(__dirname, 'app/spritesmith-generated/sprite.json'), {
                    format: 'json_texture'
                }]
                ]
            },
            apiOptions: {
                cssImageRef: '~sprite.png'
            }
        }),
        new webpack.DefinePlugin({
            WEBSOCKET_URL: '"ws://192.168.1.40:3001"'
        })
    ]
}, {
    entry: {
        main: './app/styles/main.js'
    },
    output: {
        path: path.join(__dirname, 'dist/'),
        filename: '[name].css'
    },
    module: {
        rules: [{
            test: /\.css$/,
            use: ExtractTextPlugin.extract({ use: 'css-loader!autoprefixer-loader' })
        }, {
            test: /\.scss$/,
            use: ExtractTextPlugin.extract({ use: 'css-loader!sass-loader!autoprefixer-loader' })
        }]
    },
    plugins: [
        new ExtractTextPlugin('[name].css')
    ]
}];
