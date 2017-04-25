import Kifu from './vue/Kifu.vue';
import KifuIndex from './vue/KifuIndex.vue';
import Vue from 'vue';
import VueRouter from 'vue-router';
import axios from 'axios';

Vue.use(VueRouter);

const Show = {
    name: 'show',
    template: '<div><kifu :kifu="kifu"></kifu><button v-on:click="fav" class="btn">fav</button></div>',
    data: function() {
        const data = {
            kifu: []
        };

        axios.get(`/kifu/${this.$route.params.id}`).then(res => {
            data.kifu = res.data;

            /* global WEBSOCKET_URL */ // see webpack.config.js
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
                const nn = jsonData[0];

                let value = jsonData[1].score.fields[0];
                const type = jsonData[1].score.variant;
                if (nn % 2) {
                    value = -value;
                }

                Vue.set(data.kifu[nn], 'value', value);
                Vue.set(data.kifu[nn], 'type', type);
                Vue.set(data.kifu[nn], 'pv', jsonData[1].moves);
            };
        });

        return data;
    },
    methods: {
        fav: function() {
            axios.post(`/kifu/fav/${this.$route.params.id}`, { fav: true }).then(() => {});
        }
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
            res.data.forEach(g => {
                g.on_click = () => {
                    router.push(`/kifu/${g.id}`);
                };
                g.name = g.id;
            });

            data.kifu = res.data;
        });

        return data;
    }
};

const ShougiWarsIndex = {
    template: '<kifu-index :kifu="kifu"></kifu-index>',
    components: {
        'kifu-index': KifuIndex
    },
    data: function() {
        const data = {
            kifu: []
        };

        axios.get(`/kifu/shougiwars/history/${this.$route.params.id}`).then(res => {
            res.data.forEach(g => {
                g.on_click = () => {
                    axios.get(`/kifu/shougiwars/game/${g.name}`).then(res => {
                        console.log(res);
                        router.push(`/kifu/${res.data.id}`);
                    });
                };
            });
            data.kifu = res.data;
        });

        return data;
    }
};

const router = new VueRouter({
    routes: [
    { path: '/kifu/', component: Index },
    { path: '/kifu/shougiwars/:id', component: ShougiWarsIndex },
    { path: '/kifu/:id', component: Show }
    ]
});
//          <li class="nav-item">
//             <router-link to="/kifu/" class="nav-link active">local</router-link>
//          </li>
//          <li class="nav-item">
//            <router-link to="/kifu/shougiwars/sunaemon0" class="nav-link">sunaemon0</router-link>
//          </li>
//          <li class="nav-item">
//            <router-link to="logoff" class="nav-link">login</router-link>
//          </li>
//        </ul>
//        <router-view></router-view>

new Vue({
    template: '<div><ul class="nav nav-tabs"><li v-for="tab in tabs" :class="{ \'nav-item\': true, active: false }" v-if="tab.show"><a href v-on:click="tab.on_click">{{tab.caption}}</a></li></ul><router-view/><div v-if="login">login!!</div></div>',
    router: router,
    data: function() {
        return {
            login: false,
            tabs: [
                {
                    caption: 'local',
                    on_click: () => router.push('/kifu/'),
                    maches: url => url === '/kifu/',
                    show: true
                },
                {
                    caption: 'sunaemon0',
                    on_click: () => router.push('/kifu/shougiwars/sunaemon0'),
                    maches: url => url === '/kifu/shougiwars/sunaemon0',
                    show: true
                },
                {
                    caption: 'logoff',
                    on_click: () => axios.post('/usres/logoff').then(() => this.login = false),
                    maches: () => false,
                    show: true
                },
                {
                    caption: 'login',
                    on_click: () => axios.post('/usres/login', { email: 'a', password: 'a' }).then(() => this.login = true),
                    maches: () => false,
                    show: true
                }
            ]
        };
    }
}).$mount('#app');

