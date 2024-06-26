<template>
    <div>
        <h1 class="text-blue-600 text-center text-4xl">All posts</h1>
        <p class=" text-center text-sm">click on post to delete</p>
        <div class="w-70 flex gap-5">
            <label>Value:</label>
            <input v-model="newPost" type="text">
            <button @click="addPost">Add post</button>
        </div>
        <div class="w-30 flex flex-row-reverse gap-5">
            <button class="float-right bg-blue-600 p-2 text-white rounded" @click="getPosts">Update</button>
            <button class="bg-blue-600 p-2 text-white rounded" @click="clearPosts">Clear</button>
        </div>
        <br>
        <div class="grid grid-cols-5 text-center gap-5">
            <div v-for="(post, index) in posts" class="drop-shadow-md bg-stone-300 p-5 overflow-auto">
                <p class=" cursor-pointer" @click="deletePost(index)">{{ post }}</p>
            </div>
        </div>
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
        },
        async deletePost(id) {
            await day2_backend.delete_post(id);
        }
    },
    async mounted() {
        this.getPosts();
    }
}
</script>