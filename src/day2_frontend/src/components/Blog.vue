<template>
    <div>
        <center>
            <label>New post value:</label>
            <input v-model="newPost" type="text">
            <button @click="addPost">Send post</button>
            <br>
            <button @click="getPosts">Update</button>
            <button @click="clearPosts">Clear</button>
            <br>
            {{ posts }}
        </center>
    </div>
</template>

<script>
import { day2_backend } from 'declarations/day2_backend/index';

export default {
    data() {
        return {
            posts: [],
            newPost: ""
        }
    },
    methods: {
        async getPosts() {
            this.posts = await day2_backend.read_posts();
        },
        async addPost() {
            await day2_backend.add_post(this.newPost);
        },
        async clearPosts() {
            await day2_backend.clear_posts();
        }
    }
}
</script>