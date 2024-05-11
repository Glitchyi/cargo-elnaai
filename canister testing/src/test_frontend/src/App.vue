<script setup>
import { ref, onMounted } from 'vue';
import { test_backend } from 'declarations/test_backend/index';
let datasetName = ref('default');
let className = ref('apple');
let images = ref([]);

const handleFileUpload = (event) => {
  const files = event.target.files;
  images.value = Array.from(files);
};

const createDataset = async () => {
  await test_backend.create_dataset(datasetName.value);
  if (images.value.length > 0) {
    for (let img of images.value) {
      let reader = new FileReader();
      reader.onload = async (e) => {
        let imgArrayBuffer = new Uint8Array(e.target.result);
        await test_backend.store_image(datasetName.value, imgArrayBuffer, className.value);
      };
      reader.readAsArrayBuffer(img);
    }
  }
};

let allImages = ref([]);
const getImages = async () => {
  allImages.value = await test_backend.get_images_from_dataset(datasetName.value, className.value);
};

onMounted(getImages);
</script>

<template>
  <main>
    <form action="#" @submit.prevent="createDataset">
      <label for="image">Upload images: &nbsp;</label>
      <input id="image" type="file" @change="handleFileUpload" multiple />
      <button type="submit">Create Dataset</button>
    </form>
    <div>
      <button @click="getImages">Get Images</button>
      <div v-for="(img, index) in allImages" :key="index">
        <img :src="URL.createObjectURL(new Blob([img], { type: 'image/*' }))" />
      </div>
    </div>
  </main>
</template>