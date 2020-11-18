<script lang="ts">
import Header from "@/components/Header.vue";
export default {
    components: {
        Header,
    },
};
</script>

<template>
    <Header />
    <router-view />
</template>

<style lang="scss">
html,
body,
#app {
    height: 100%;
    overflow: auto;
    width: 100%;
    margin: 0;
    padding: 0;
    border: 0;
}

#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #fff;
    background: #bc2131;

    display: flex;
}

#welcome {
    margin-top: 200px;
    margin-left: 200px;

    h1 {
        font-size: 60px;

        .smaller {
            font-size: 40px;
            color: #f2b7bd;
        }

        .lead {
            font-size: 25px;
            color: #f2b7bd;
            margin-top: 0px;

            .font-normal {
                color: #ffffff;
            }
        }
    }
}
</style>
