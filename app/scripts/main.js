$(document).ready(function(){
  function cp_to_img(cp) {
    const piece_to_num = [8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 13, 12];
    if(!cp.color) {
      return piece_to_num[cp.piece];
    } else {
      return piece_to_num[cp.piece] + 30;
    }
  }

  let images = [[],[]];
  for(let c=0; c<2; c++) {
    for(let p=0; p<13; p++) {
      let piece = new Image();
      piece.src = '/images/piece/sgl' + ('0'+cp_to_img({color: c, piece: p})).slice(-2) + '.png';
      images[c].push(piece);
    }
  }
  function get_image(cp) {
    return images[cp.color][cp.piece];
  }
  var board_img = new Image();
  board_img.src = '/images/board.jpg';
  function update_board(board) {
    var ctx = $('#board')[0].getContext('2d');
    ctx.drawImage(board_img, 0, 0, 600, 640);

    for(let j=0; j<9; j++) {
      for(let i=0; i<9; i++) {
        let cp = board[j][8-i];
        if(cp)
          ctx.drawImage(get_image(cp), 30+60*i, 30+64*j, 60, 64);
      }
    }
  }

  let kifu={};
  $.get('/get_moves', function(d) {
    kifu = d;
    $('#moves').empty();
    kifu.forEach(function(m,n) {
      $('#moves').append($('<option>').val(n).text(n+ ' ' + m.move_str));
    });
  });
  $('#moves').change(function() {
    update_board(kifu[parseInt($('#moves').val())].position.board);
  });
  var connection = new WebSocket('wss://ws.kifu-analysis.com');
  connection.onopen = function() {
    console.log('connection opened');
  };
  connection.onerror = function (error) {
    console.log('WebSocket Error ' + error);
  };
  connection.onmessage = function(e) {
    let data = JSON.parse(e.data);
    let n = data.n;
    if (kifu[n].position.color) {
      data.score.value = -data.score.value
    }
    kifu[n].score = data.score;
    $('#moves').children('[value="' + n +'"]').text(n + ' ' + kifu[n].move_str + ' ' + data.score.type + ' ' + data.score.value)
  }
});
