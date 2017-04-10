const gulp = require('gulp');
const sass = require('gulp-sass');
const gulpif = require('gulp-if');
const autoprefixer = require('gulp-autoprefixer');
const spritesmith = require('gulp.spritesmith');
const cleanCSS = require('gulp-clean-css');
const browserify = require('gulp-browserify');
const uglify = require('gulp-uglify');
const babel = require('gulp-babel');
const yargs = require('yargs').argv;
const pump = require('pump');

gulp.task('sprite', () => {
    const spriteData = gulp.src(['app/images/*.jpg', 'app/images/**/*.png']).pipe(spritesmith({
        imgName: 'sprite.png',
        cssName: 'sprite.json',
        cssFormat: 'json',
        imgPath: '/dist/sprite.png'
    }));
    spriteData.img.pipe(gulp.dest('dist/'));
    spriteData.css.pipe(gulp.dest('app/spritesmith-generated/'));
});

gulp.task('scss', () => {
    gulp.src(['app/styles/main.scss'])
        .pipe(sass({
            includePaths:
            ['bower_components/bootstrap/scss/', 'bower_components/font-awesome/scss/']
        }))
        .pipe(autoprefixer())
        .pipe(gulpif(yargs.production, cleanCSS()))
        .pipe(gulp.dest('dist/'));
});

gulp.task('js', cb => {
    pump([gulp.src(['app/scripts/*.js']),
        babel(),
        browserify({
            inserteGlobals: true,
            debug: !yargs.production
        }),
        gulpif(yargs.production, uglify()),
        gulp.dest('dist/')], cb);
});

gulp.task('default', ['sprite', 'scss', 'js']);
