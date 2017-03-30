$(document).ready(function() {
    function populate_board(kifu_id) {

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
        const color_to_num = {
            Black: 0,
            White: 1
        };
        function cp_to_img(color, piece) {
            if (color === 'Black') {
                return piece_to_num[piece];
            } else if (color === 'White') {
                return piece_to_num[piece] + 30;
            }
        }

        const images = { White: {}, Black: {} };
        for (const color in color_to_num) {
            for (const piece in piece_to_num) {
                const img = new Image();
                img.src = `/app/images/piece/sgl${`0${cp_to_img(color, piece)}`.slice(-2)}.png`;
                images[color][piece] = img;
            }
        }

        function get_image(cp) {
            const [color, piece] = cp;
            return images[color][piece];
        }
        const board_img = new Image();
        board_img.src = '/app/images/board.jpg';
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
        $.get(`/kifu/show_moves/${kifu_id}`, function(d) {
            kifu = d;
            $('#moves').empty();
            kifu.forEach(function(m, n) {
                $('#moves').append($('<option>').val(n).text(`${n} ${m.movestr}`));
            });
        });
        $('#moves').change(function() {
            update_board(kifu[parseInt($('#moves').val())].position.board.inner);
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
            let value = data.score.fields[0];
            const type = data.score.variant;
            if (n % 2) {
                value = -value;
            }
            kifu[n].score = data.score;
            $('#moves').children(`[value="${n}"]`).text(`${n} ${kifu[n].movestr} ${type} ${value}`);
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
