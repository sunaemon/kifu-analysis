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

