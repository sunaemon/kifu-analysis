$(document).ready(function() {
    function populate_board(kifu_id) {
        function cp_to_img(cp) {
            const piece_to_num = [8, 7, 6, 5, 4, 3, 2, 1, 18, 17, 16, 15, 13, 12];
            if (!cp.color) {
                return piece_to_num[cp.piece];
            } else {
                return piece_to_num[cp.piece] + 30;
            }
        }

        const images = [[], []];
        for (let c = 0; c < 2; c++) {
            for (let p = 0; p < 14; p++) {
                const piece = new Image();
                piece.src = `/images/piece/sgl${`0${cp_to_img({ color: c, piece: p })}`.slice(-2)}.png`;
                images[c].push(piece);
            }
        }
        function get_image(cp) {
            return images[cp.color][cp.piece];
        }
        const board_img = new Image();
        board_img.src = '/images/board.jpg';
        function update_board(board) {
            const ctx = $('#board')[0].getContext('2d');
            ctx.drawImage(board_img, 0, 0, 600, 640);

            for (let j = 0; j < 9; j++) {
                for (let i = 0; i < 9; i++) {
                    const cp = board[j][8 - i];
                    if (cp)
                        ctx.drawImage(get_image(cp), 30 + 60 * i, 30 + 64 * j, 60, 64);
                }
            }
        }

        let kifu = {};
        $.get('/kifu/show_moves', function(d) {
            kifu = d;
            $('#moves').empty();
            kifu.forEach(function(m, n) {
                $('#moves').append($('<option>').val(n).text(`${n} ${m.move_str}`));
            });
        });
        $('#moves').change(function() {
            update_board(kifu[parseInt($('#moves').val())].position.board);
        });
        const connection = new WebSocket('ws://192.168.1.40:3001');
        connection.onopen = function() {
            console.log('connection opened');
            connection.send(kifu_id);
        };
        connection.onerror = function(error) {
            console.log(`WebSocket Error ${error}`);
        };
        connection.onmessage = function(e) {
            const data = JSON.parse(e.data);
            const n = data.n;
            if (kifu[n].position.color) {
                data.score.value = -data.score.value;
            }
            kifu[n].score = data.score;
            $('#moves').children(`[value="${n}"]`).text(`${n} ${kifu[n].move_str} ${data.score.type} ${data.score.value}`);
        };
    }

    const url = window.location.href;

    if (new RegExp('https?://[^/]*/kifu/?').exec(url)) {
        $('#shougiwars_search').on('submit', event => {
            window.location.href = `/kifu/shougiwars/history/${$('#shougiwars_usename').val()}`;
            event.preventDefault();
        });
    } else if (new RegExp('https?://[^/]*/kifu/([^/]*)/?').exec(url)) {
        populate_board($('#kifu_id').text());
    } else if (new RegExp('https?://[^/]*/kifu/shougiwars/game/([^/]*)/?').exec(url)) {
        populate_board($('#kifu_id').text());
    }
});
