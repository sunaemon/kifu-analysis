import Kifu from './vue/Kifu.vue';
import KifuIndex from './vue/KifuIndex.vue';
import Vue from 'vue';
import VueRouter from 'vue-router';
import axios from 'axios';

Vue.use(VueRouter);

const Show = {
    name: 'show',
    template: '<kifu :kifu="kifu"></kifu>',
    data: function() {
        const data = {
            kifu: []
        };

        axios.get(`/kifu/${this.$route.params.id}`).then(res => {
            data.kifu = res.data;

            const connection = new WebSocket(WEBSOCKET_URL);
            connection.onopen = () => {
                console.log('connection opened');
                connection.send(this.$route.params.id);
            };
            connection.onerror = error => {
                console.log(`WebSocket Error ${error}`);
            };
            connection.onmessage = event => {
                const jsonData = JSON.parse(event.data);
                console.log(jsonData);
                const nn = jsonData[0];

                let value = jsonData[1].score.fields[0];
                const type = jsonData[1].score.variant;
                if (nn % 2) {
                    value = -value;
                }

                data.kifu[nn].value = value;
                data.kifu[nn].type = type;
                data.kifu[nn].pv = jsonData[1].moves;
            };
        });

        return data;
    },
    components: {
        kifu: Kifu
    }
};

const Index = {
    template: '<kifu-index :kifu="kifu"></kifu-index>',
    components: {
        'kifu-index': KifuIndex
    },
    data: function() {
        const data = {
            kifu: []
        };

        axios.get('/kifu/').then(res => {
            data.kifu = res.data;
        });

        return data;
    }
};

new Vue({
    router: new VueRouter({
        routes: [
        { path: '/kifu/', component: Index },
        { path: '/kifu/:id', component: Show }
        ]
    })
}).$mount('#app');

