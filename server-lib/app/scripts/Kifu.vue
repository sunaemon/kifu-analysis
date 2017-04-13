<template>
  <div>
  <board></board>
  <table style="font-size:80%;" class="table-bordered">
    <tbody>
      <tr v-for="i in [-2, -1, 0, 1, 2]">
        <td :text-content.prop="move(i)"></td>
      </tr>
    </tbody>
    <tfoot>
      <th>指手</th>
      <th>評価値</th>
      <th>読み筋</th>
    </tfoot>
  </table>
  <p><span :text-content.prop="n"></span>手目の<span :text-content.prop="k">0</span>手先</p>
  <button class="btn" v-on:click="n++"><i class="fa fa-angle-down" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="n--"><i class="fa fa-angle-up" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="k++"><i class="fa fa-angle-left" aria-hidden="true"></i></button>
  <button class="btn" v-on:click="k--"><i class="fa fa-angle-right" aria-hidden="true"></i></button>
  </div>
</template>

<script>
module.exports = {
  data: function() {
    return {
       n: 0,
       k: 0,
    }
  },
  computed: {
    move: function(i) {
      return () => {
      const ki = this.kifu[this.n + i];

      if (ki && ki.movement) {
        return ki.move_str;
      } else {
        return '-----';
      }
      }
    }
  },
  props: {
    kifu: {
      type: Array,
      default: () => []
    }
  },
  components: {
    board: require('./Board.vue')
  }
}
</script>
