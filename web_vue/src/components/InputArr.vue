
<script setup lang="ts">
import { computed, ref, Ref, toRefs, onUpdated } from "vue";

const { modelValue } = defineProps<{
    modelValue: string[],
}>()

const emits = defineEmits<{
    (emit: 'update:modelValue', val: string): void
}>()

const textA = computed(() => modelValue.join(", "))
const val = ref("");
function onclick() {
    // modelValue += val.value // Uncaught TypeError: Assignment to constant variable
    // emits("update:modelValue", val.value)
    // console.log(modelValue)
    // text2.value = modelValue
    modelValue[0] = val.value
}
function show() {
}
const text2 = ref("")
</script>


<template>
    <div>{{ textA}}</div>
    <button @click="onclick">add</button>
    <button @click="show">show</button>
    <input type="text" v-model="modelValue[0]" />
</template>