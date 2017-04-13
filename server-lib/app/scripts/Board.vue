<template>
  <div>
    <img src="/dist/sprite.png" id="sprite" hidden>
    <canvas width="600" height="640" v-show-board="data">
      Your brower is not supported.
    </canvas>
  </div>
</template>

<script>
// https://stackoverflow.com/questions/40177493/drawing-onto-a-canvas-with-vue-js
function get_image_name(cp) {
    const piece_to_num = {
        Pawn: 8,
        Lance: 7,
        Knight: 6,
        Silver: 5,
        Gold: 4,
        Bishop: 3,
        Rook: 2,
        King: 1,
        PPawn: 18,
        PLance: 17,
        PKnight: 16,
        PSilver: 15,
        Horse: 13,
        Dragon: 12
    };


    const [color, piece] = cp;

    let num;
    if (color === 'Black') {
      num = piece_to_num[piece];
    } else if (color === 'White') {
      num = piece_to_num[piece] + 30;
    }

    const znum = `0${num}`.slice(-2);
    return `sgl${znum}`;
}

module.exports = {
  directives: {
    showBoard: function(canvas, data) {
      const sprite = require('../spritesmith-generated/sprite.json').frames;
      const ctx = canvas.getContext('2d');
      const stripe_img = document.getElementById('sprite');

      const board = sprite.board.frame;
      ctx.drawImage(stripe_img, board.x, board.y, board.w, board.h, 0, 0, board.w, board.h);

      for (let j = 0; j < 9; j++) {
        for (let i = 0; i < 9; i++) {
          const cp = data.value[j][8 - i];
          if (cp) {
            const name = get_image_name(cp);
            const frame = sprite[name].frame;
            ctx.drawImage(stripe_img, frame.x, frame.y, frame.w, frame.h, 30 + 60 * i, 30 + 64 * j, frame.w, frame.h);
          }
        }
      }
    }
  },
  props: {
    data: {
      type: Array,
      default: function() { return [
      [["White","Lance"],["White","Knight"],["White","Silver"],["White","Gold"],["White","King"],["White","Gold"],["White","Silver"],["White","Knight"],["White","Lance"]],
      [null,["White","Bishop"],null,null,null,null,null,["White","Rook"],null],
      [["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"],["White","Pawn"]],
      [null,null,null,null,null,null,null,null,null],
      [null,null,null,null,null,null,null,null,null],
      [null,null,null,null,null,null,null,null,null],
      [["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"],["Black","Pawn"]],
      [null,["Black","Rook"],null,null,null,null,null,["Black","Bishop"],null],
      [["Black","Lance"],["Black","Knight"],["Black","Silver"],["Black","Gold"],["Black","King"],["Black","Gold"],["Black","Silver"],["Black","Knight"],["Black","Lance"]]
      ];
    }
    }
  }
}
</script>

<style lang="stylus" scoped>
canvas
  width:100%
</style>
