<template>
  <div>
  <board></board>
  <table style="font-size:80%;" class="table-bordered">
    <tbody>
      <tr v-for="i in [-2, -1, 0, 1, 2]">
        <td :text-content.prop="move(i)"></td>
        <td :text-content.prop="score(i)"></td>
        <td :text-content.prop="pv(i)"></td>
      </tr>
    </tbody>
    <tfoot>
      <th>指手</th>
      <th>評価値</th>
      <th>読み筋</th>
    </tfoot>
  </table>
  <p><span :text-content.prop="n"></span>手目の<span :text-content.prop="k">0</span>手先</p>
  <button class="btn" v-on:click="up()"><i class="fa fa-angle-down" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="down()"><i class="fa fa-angle-up" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="next()"><i class="fa fa-angle-left" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="prev()"><i class="fa fa-angle-right" aria-hidden="true"></i></button>
  </div>
</template>

<script>
module.exports = {
  data: function() {

      const handler = e  => {
          e = e || window.event;
          let code = e.keyCode;
          if (e.charCode && code === 0)
              code = e.charCode;

          if (code === 37) {
              prev();
          } else if (code === 38) {
              up();
          } else if (code === 39) {
              next();
          } else if (code === 40) {
              down();
          } else {
              return;
          }
          e.preventDefault();
      };
      $(document).keydown(handler);
      return {
          n: 0,
          k: 0,
      }
  },
  computed: {
  },
  props: {
      kifu: {
          default: () => []
      }
  },
  components: {
      board: require('./Board.vue')
  },
  methods: {
      move: function(i) {
        const ki = this.kifu[this.n + i];

        if (ki && ki.movement) {
          return ki.movestr;
        } else {
          return '-----';
        }
      },
      score: function(i) {
        const ki = this.kifu[this.n + i];

        if (ki && ki.score) {
          return `${ki.type} ${ki.value}`;
        } else {
          return '-----';
        }
      },
      pv: function(i) {
        const ki = this.kifu[this.n + i];

        if (ki && ki.pv) {
            const make_templ = k => {
                let template = '';

                const init = Math.max(k - 1, 1);
                for (let j = init; j < Math.min(init + 4, ki.pv.length); j++) {
                    if (ki.pv[j]) {
                        if (i === 0 && j === k) {
                            template = `${template}<span class="move-selected">${ki.pv[j].movestr}</span>`;
                        } else {
                            template = `${template}${ki.pv[j].movestr}`;
                        }
                    }
                }

                return template;
            };
            if (i === 0) {
                return make_templ(k);
            } else {
                return make_templ(0);
            }
        } else {
            return '-----';
        }
      },
      next: function() {
          if (!this.kifu[this.n].pv || !this.kifu[this.n].pv[this.k + 1]) {
              return;
          }
          this.k += 1;
      },
      prev: function() {
          if (this.k < 1) {
              return;
          }
          this.k -= 1;
      },
      down: function() {
          if (!this.kifu[this.n + 1]) {
              return;
          }
          this.n += 1;
          this.k = 0;
      },
      up: function() {
          if (this.n < 1) {
              return;
          }
          this.n -= 1;
          this.k = 0;
      }
  }
}
</script>
