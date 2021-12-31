
<script setup lang="ts">
import { computed, ref, Ref, toRefs, onUpdated } from "vue";

const { modelValue } = defineProps<{
    modelValue: string,
}>()

const emits = defineEmits<{
    (emit: 'update:modelValue', val: string): void
}>()

const textA = computed(() => modelValue + "  ;;")
const val = ref("");
function onclick() {
    // modelValue += val.value // Uncaught TypeError: Assignment to constant variable
    emits("update:modelValue", val.value)
    console.log(modelValue)
    text2.value = modelValue
}
function show() {
}
onUpdated(() => {
    console.log("update: ", modelValue)
})
const text2 = ref("")

const model_val = ref(modelValue)
</script>


<template>
    <div>{{ textA + text2}}</div>
    <button @click="onclick">add</button>
    <button @click="show">show</button>
    <input type="text" v-model="model_val" />
</template>