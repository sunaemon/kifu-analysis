<template>
  <div>
    <div class="col-md-6">
      <board :data="currentBoard"></board>
    </div>
    <div class="col-md-6">
      <table style="font-size:80%;" class="table-bordered">
        <tbody>
          <tr v-for="i in [-2, -1, 0, 1, 2]">
            <td :class="i == 0 && k == 0 ? 'move-selected': ''" :text-content.prop="move(i)"></td>
            <td :text-content.prop="score(i)"></td>
            <td v-html="pv(i)"></td>
          </tr>
        </tbody>
        <tfoot>
          <th>指手</th>
          <th>評価値</th>
          <th>読み筋</th>
        </tfoot>
      </table>
      <p><span :text-content.prop="n"></span>手目の<span :text-content.prop="k">0</span>手先</p>
      <button class="btn" @click="up()"><i class="fa fa-angle-down" aria-hidden="true"></i></button>
      <button class="btn" @click="down()"><i class="fa fa-angle-up" aria-hidden="true"></i></button>
      <button class="btn" @click="next()"><i class="fa fa-angle-left" aria-hidden="true"></i></button>
      <button class="btn" @click="prev()"><i class="fa fa-angle-right" aria-hidden="true"></i></button>
    </div>
  </div>
</template>

<script>
module.exports = {
  data: function () {
    window.AudioContext = window.AudioContext || window.webkitAudioContext
    return {
      n: 0,
      k: 0,
      clickSoundLoaded: false,
      xStart: 0,
      yStart: 0,
      nStart: 0,
      kStart: 0,
      kPos: new Date(),
      audioContext: new AudioContext(),
      clickSound: null,
      lastTouchEnd: new Date()
    }
  },
  created: function () {
    window.addEventListener('keypress', this.onKeypress)
    window.addEventListener('touchstart', this.onTouchstart)
    window.addEventListener('touchmove', this.onTouchmove)
    window.addEventListener('touchend', this.onTouchend)

    const req = new XMLHttpRequest()
    req.responseType = 'arraybuffer'

    req.onload = () => {
      this.audioContext.decodeAudioData(req.response, buffer => {
        console.log(buffer);
        this.clickSound = buffer
      }, e => console.log(e))
    }

    req.open('GET', '/app/click.mp3', true)
    // req.open('GET', '/app/click.ogg', true)
    req.send()
  },
  beforeDestroy: function () {
    window.removeEventListener('keypress', this.onKeypress)
    window.removeEventListener('touchstart', this.onTouchstart)
    window.removeEventListener('touchmove', this.onTouchmove)
    window.removeEventListener('touchend', this.onTouchend)
  },
  props: {
    kifu: {
      default: () => []
    }
  },
  components: {
    board: require('./Board.vue')
  },
  computed: {
    currentBoard: function () {
      if (!this.kifu[this.n]) {
        return [
      [['White', 'Lance'], ['White', 'Knight'], ['White', 'Silver'], ['White', 'Gold'], ['White', 'King'], ['White', 'Gold'], ['White', 'Silver'], ['White', 'Knight'], ['White', 'Lance']],
      [null, ['White', 'Bishop'], null, null, null, null, null, ['White', 'Rook'], null],
      [['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn'], ['White', 'Pawn']],
      [null, null, null, null, null, null, null, null, null],
      [null, null, null, null, null, null, null, null, null],
      [null, null, null, null, null, null, null, null, null],
      [['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn'], ['Black', 'Pawn']],
      [null, ['Black', 'Rook'], null, null, null, null, null, ['Black', 'Bishop'], null],
      [['Black', 'Lance'], ['Black', 'Knight'], ['Black', 'Silver'], ['Black', 'Gold'], ['Black', 'King'], ['Black', 'Gold'], ['Black', 'Silver'], ['Black', 'Knight'], ['Black', 'Lance']]
        ]
      } else if (this.k === 0) {
        return this.kifu[this.n].position.board.inner
      } else {
        return this.kifu[this.n].pv[this.k].position.board.inner
      }
    }
  },
  methods: {
    move: function (i) {
      const ki = this.kifu[this.n + i]

      if (ki && ki.movement) {
        return ki.movestr
      } else {
        return '-----'
      }
    },
    score: function (i) {
      const ki = this.kifu[this.n + i]

      if (ki && ki.value) {
        return `${ki.type} ${ki.value}`
      } else {
        return '-----'
      }
    },
    pv: function (i) {
      const ki = this.kifu[this.n + i]

      if (ki && ki.pv) {
        const makeTempl = k => {
          let template = ''

          const init = Math.max(k - 1, 1)
          for (let j = init; j < Math.min(init + 4, ki.pv.length); j++) {
            if (ki.pv[j]) {
              if (i === 0 && j === k) {
                template = `${template}<span class="move-selected">${ki.pv[j].movestr}</span>`
              } else {
                template = `${template}${ki.pv[j].movestr}`
              }
            }
          }

          return template
        }
        if (i === 0) {
          return makeTempl(this.k)
        } else {
          return makeTempl(0)
        }
      } else {
        return '-----'
      }
    },
    onTouchstart: function (e) {
      this.xStart = e.changedTouches[0].pageX
      this.yStart = e.changedTouches[0].pageY
      this.nStart = this.n
      this.kStart = this.k
      if (!this.clickSoundLoaded) {
        this.playSound(this.clickSound, 0.001)
        this.clickSoundLoaded = true
      }

      if (e.touches.length > 1) {
        e.preventDefault()
      }
    },
    onTouchmove: function (e) {
      e.preventDefault()

      if (this.k === 0 && new Date() - this.kPos > 20) {
        const oldn = this.n
        const tmp = this.nStart + Math.floor((this.yStart - e.changedTouches[0].pageY) / 10)
        this.n = Math.min(Math.max(tmp, 0), this.kifu.length - 1)

        if (oldn !== this.n) {
          this.playSound(this.clickSound)
          this.xStart = e.changedTouches[0].pageX
        }
      } else {
        this.yStart = e.changedTouches[0].pageY
        this.kPos = new Date()
      }

      const oldk = this.k
      if (this.kifu[this.n].pv) {
        const tmp = this.kStart + Math.floor((this.xStart - e.changedTouches[0].pageX) / 10)
        this.k = Math.min(Math.max(tmp, 0), this.kifu[this.n].pv.length - 1)
      }

      if (oldk !== this.k) {
        this.playSound(this.clickSound)
      }
    },
    onTouchend: function (event) {
      event.preventDefault()
      /*
      const now = (new Date()).getTime()
      if (now - this.lastTouchEnd <= 300) {
        event.preventDefault()
      }
      this.lastTouchEnd = now
      */
    },
    onKeypress: function (event) {
      event = event || window.event
      let code = event.keyCode
      if (event.charCode && code === 0) { code = event.charCode }

      if (code === 37) {
        this.prev()
      } else if (code === 38) {
        this.up()
      } else if (code === 39) {
        this.next()
      } else if (code === 40) {
        this.down()
      } else {
        return
      }
      event.preventDefault()
    },
    next: function () {
      if (!this.kifu[this.n].pv || !this.kifu[this.n].pv[this.k + 1]) {
        return
      }
      this.k += 1
      this.playSound(this.clickSound)
    },
    prev: function () {
      if (this.k < 1) {
        return
      }
      this.k -= 1
      this.playSound(this.clickSound)
    },
    down: function () {
      if (!this.kifu[this.n + 1]) {
        return
      }
      this.n += 1
      this.k = 0
      this.playSound(this.clickSound)
    },
    up: function () {
      if (this.n < 1) {
        return
      }
      this.n -= 1
      this.k = 0
      this.playSound(this.clickSound)
    },
    playSound: function (buffer, gain) {
      gain = gain || 0.8
      const source = this.audioContext.createBufferSource()
      source.buffer = buffer
      const gainNode = this.audioContext.createGain()
      gainNode.gain.value = gain
      source.connect(gainNode)
      gainNode.connect(this.audioContext.destination)
      source.start(0)
    }
  }
}
</script>
