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
                update_board();
                board_init = true;
            }
        });

        let n = 0;
        let k = 0;
        const stripe_img = $('#sprite');
        function update_board() {
            $('#n').text(n);
            $('#k').text(k);

            let board;
            if (k === 0) {
                board = kifu[n].position.board.inner;
            } else {
                board = kifu[n].pv[k].position.board.inner;
            }

            for (let i = -2; i < 3; i++) {
                const ki = kifu[n + i];
                const ki_n = kifu[n + i + 1];

                if (i === 0 && k === 0) {
                    $(`#move${i}`).addClass('move-selected');
                } else {
                    $(`#move${i}`).removeClass('move-selected');
                }

                if (ki && ki.movement) {
                    $(`#move${i}`).text(ki.movestr);
                } else {
                    $(`#move${i}`).text('-----');
                }

                if (ki && ki.value) {
                    $(`#score${i}`).text(`${ki.type} ${ki.value}`);
                } else {
                    $(`#score${i}`).text('-----');
                }

                if (ki && ki.value && ki_n && ki_n.value) {
                    if (ki.type === 'Cp' && ki_n.type === 'Cp') {
                        let value_diff = ki_n.value - ki.value;
                        if ((n + i) % 2) {
                            value_diff = -value_diff;
                        }
                        $(`#diff${i}`).text(value_diff);
                    } else if (ki.type === 'Cp' && ki_n.type === 'Mate') {
                        $(`#diff${i}`).text(`cp ${ki.value} -> mate ${ki_n.value}`);
                    } else if (ki.type === 'Mate' && ki_n.type === 'Cp') {
                        $(`#diff${i}`).text(`mate ${ki.value} -> cp ${ki_n.value}`);
                    } else if (ki.type === 'Mate' && ki_n.type === 'Mate') {
                        const value_diff = ki_n.value - ki.value;
                        $(`#diff${i}`).text(`mate ${value_diff}`);
                    } else {
                        console.log(`error: ${ki.type}, ${ki_n.type}`);
                    }
                } else {
                    $(`#diff${i}`).text('-----');
                }

                if (ki && ki.pv) {
                    const make_templ = function(k) {
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
                        $(`#pv${i}`).html(make_templ(k));
                    } else {
                        $(`#pv${i}`).html(make_templ(0));
                    }
                } else {
                    $(`#pv${i}`).text('-----');
                }
            }

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


        let x_start;
        let y_start;
        let n_start;
        let k_start;
        let k_pos = new Date();
        let click_sound_loaded = false;
        $(window).on('touchstart', e => {
            x_start = e.changedTouches[0].pageX;
            y_start = e.changedTouches[0].pageY;
            n_start = n;
            k_start = k;
            if (!click_sound_loaded) {
                playSound(click_sound, 0.001);
                click_sound_loaded = true;
            }

            if (e.touches.length > 1) {
                e.preventDefault();
            }
        });
        $(window).on('touchmove', function(e) {
            let updated = false;

            if (k === 0 && new Date() - k_pos > 20) {
                const old_n = n;
                const new_n = n_start + Math.floor((y_start - e.changedTouches[0].pageY) / 10);
                n = Math.min(Math.max(new_n, 0), kifu.length - 1);

                if (old_n !== n) {
                    updated = true;
                    playSound(click_sound);
                    x_start = e.changedTouches[0].pageX;
                }
            } else {
                y_start = e.changedTouches[0].pageY;
                k_pos = new Date();
            }

            const old_k = k;
            const new_k = k_start + Math.floor((x_start - e.changedTouches[0].pageX) / 10);
            if (kifu[n].pv) {
                k = Math.min(Math.max(new_k, 0), kifu[n].pv.length - 1);
            }

            if (old_k !== k) {
                playSound(click_sound);
                updated = true;
            }

            e.preventDefault();

            if (updated) {
                update_board();
            }
        });

        let lastTouchEnd = 0;
        $(window).on('touchend', event => {
            const now = (new Date()).getTime();
            if (now - lastTouchEnd <= 300) {
                event.preventDefault();
            }
            lastTouchEnd = now;
        }, false);


        function next() {
            if (!(kifu[n].pv && kifu[n].pv[k + 1])) {
                return;
            }
            k += 1;

            playSound(click_sound);
            update_board();
        }
        function prev() {
            if (k < 1) {
                return;
            }
            k -= 1;
            playSound(click_sound);
            update_board();
        }
        function down() {
            if (!kifu[n + 1]) {
                return;
            }
            n += 1;
            k = 0;
            playSound(click_sound);
            update_board();
        }
        function up() {
            if (n < 1) {
                return;
            }
            n -= 1;
            k = 0;
            playSound(click_sound);
            update_board();
        }
        $('#next_button').click(next);
        $('#prev_button').click(prev);
        $('#down_button').click(down);
        $('#up_button').click(up);
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

        let kifu = null;
        $.get(`/kifu/show_moves/${kifu_id}`, function(d) {
            kifu = d;
            if (!board_init && sprite) {
                update_board();
                board_init = true;
            }
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
            const nn = data[0];

            let value = data[1].score.fields[0];
            const type = data[1].score.variant;
            if (nn % 2) {
                value = -value;
            }

            kifu[nn].value = value;
            kifu[nn].type = type;
            kifu[nn].pv = data[1].moves;

            update_board();
        };
    }

    const url = window.location.href;

    if (new RegExp('https?://[^/]*/kifu/?$').test(url)) {
        $('#shougiwars_search').on('submit', event => {
            window.location.href = `/kifu/shougiwars/history/${$('#shougiwars_usename').val()}`;
            event.preventDefault();
        });
    } else if (new RegExp('https?://[^/]*/kifu/([^/]*)/?$').test(url)) {
        $('#tab a[href="#local"]').click(e => {
            e.preventDefault();
            $(this).tab('add');
        });
        $('#tab a[href="#add"]').click(e => {
            e.preventDefault();
            $(this).tab('add');
        });

        populate_board($('#kifu_id').text());
    } else if (new RegExp('https?://[^/]*/kifu/shougiwars/game/([^/]*)/?$').test(url)) {
        populate_board($('#kifu_id').text());
    }

    window.AudioContext = window.AudioContext || window.webkitAudioContext;
    const context = new AudioContext();
    let click_sound;


    const req = new XMLHttpRequest();
    req.responseType = 'arraybuffer';

    req.onload = function() {
        context.decodeAudioData(req.response, buffer => {
            click_sound = buffer;
        }, e => console.log(e));
    };

    req.open('GET', '/app/click.mp3', true);
    //req.open('GET', '/app/click.ogg', true);
    req.send();

    const playSound = function(buffer, gain) {
        gain = gain || 0.8;
        const source = context.createBufferSource();
        source.buffer = buffer;
        const gainNode = context.createGain();
        gainNode.gain.value = gain;
        source.connect(gainNode);
        gainNode.connect(context.destination);
        source.start(0);
    };
});
