$(document).ready(function() {
    function populate_board(kifu_id) {
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
            function cp_to_img(color, piece) {
                if (color === 'Black') {
                    return piece_to_num[piece];
                } else if (color === 'White') {
                    return piece_to_num[piece] + 30;
                }
            }

            const [color, piece] = cp;
            return `sgl${`0${cp_to_img(color, piece)}`.slice(-2)}`;
        }

        let board_init = false;
        let sprite = null;
        $.get('/dist/sprite.json', function(d) {
            sprite = d;
            if (!board_init && kifu) {
                update_board(kifu[0].position.board.inner);
                board_init = true;
            }
        });

        const stripe_img = $('#sprite');
        function update_board(board) {
            const ctx = $('#board')[0].getContext('2d');
            ctx.drawImage(stripe_img[0], sprite.board.x, sprite.board.y, sprite.board.width, sprite.board.height, 0, 0, sprite.board.width, sprite.board.height);

            for (let j = 0; j < 9; j++) {
                for (let i = 0; i < 9; i++) {
                    const cp = board[j][8 - i];
                    if (cp) {
                        const name = get_image_name(cp);
                        ctx.drawImage(stripe_img[0], sprite[name].x, sprite[name].y, sprite[name].width, sprite[name].height, 30 + 60 * i, 30 + 64 * j, sprite[name].width, sprite[name].height);
                    }
                }
            }
        }

        let kifu = null;
        $.get(`/kifu/show_moves/${kifu_id}`, function(d) {
            kifu = d;
            $('#moves').empty();
            kifu.forEach(function(m, n) {
                $('#moves').append($('<option>').val(n).text(`${n} ${m.movestr}`));
            });
            $('#moves').val('0');
            if (!board_init && sprite) {
                update_board(kifu[0].position.board.inner);
                board_init = true;
            }
        });
        $('#moves').change(function() {
            update_board(kifu[parseInt($('#moves').val())].position.board.inner);
        });

        const websocket_url = $('#websocket_url').text();
        const connection = new WebSocket(websocket_url);
        connection.onopen = function() {
            console.log('connection opened');
            connection.send(kifu_id);
        };
        connection.onerror = function(error) {
            console.log(`WebSocket Error ${error}`);
        };
        connection.onmessage = function(e) {
            const data = JSON.parse(e.data);
            const n = data[0];

            let value = data[1].score.fields[0];
            const type = data[1].score.variant;
            if (n % 2) {
                value = -value;
            }

            kifu[n].score = data[1].score;
            kifu[n].pv_str = '';
            data[1].moves.forEach(function(m, i) {
                if (i > 0) {
                    kifu[n].pv_str = `${kifu[n].pv_str}${m.movestr}`;
                }
            });
            $('#moves').children(`[value="${n}"]`).text(`${n} ${kifu[n].movestr} ${kifu[n].pv_str} ${type} ${value}`);
        };
    }

    const url = window.location.href;

    if (new RegExp('https?://[^/]*/kifu/?$').test(url)) {
        $('#shougiwars_search').on('submit', event => {
            window.location.href = `/kifu/shougiwars/history/${$('#shougiwars_usename').val()}`;
            event.preventDefault();
        });
    } else if (new RegExp('https?://[^/]*/kifu/([^/]*)/?$').test(url)) {
        populate_board($('#kifu_id').text());
    } else if (new RegExp('https?://[^/]*/kifu/shougiwars/game/([^/]*)/?$').test(url)) {
        populate_board($('#kifu_id').text());
    }
});
