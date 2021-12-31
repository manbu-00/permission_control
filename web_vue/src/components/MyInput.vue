
<script setup lang="ts">
import { defineComponent, DefineComponent, computed, ref, Ref, toRefs } from "vue";

const { modelValue } = defineProps<{
    modelValue: Map<string, string>,
}>()

const textA = computed(() => [...modelValue.entries()].map(([k, v]) => k + "-" + v).join(", "))
const val = ref("");
</script>


<template>
    <div>{{ textA }}</div>
    <button @click="modelValue.set(val, val)">add</button>
    <input type="text" v-model="val" />
</template>