<template>
  <table class="table">
    <thead>
      <tr>
        <th>id</th>
        <th>black</th>
        <th>white</th>
        <th>winner</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="item in kifu">
        <td><a v-on:click.prevent="item.on_click" href>{{item.name}}</a></td>
        <td>{{item.black}}</td>
        <td>{{item.white}}</td>
        <td>{{item.winner}}</td>
      </tr>
    </tbody>
  </table>
</template>

<script>
module.exports = {
  props: {
    kifu: {
      type: Array,
      default: () => []
    }
  }
}
</script>
