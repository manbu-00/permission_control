
<script setup lang="ts">
import { computed, ref  } from "vue";
import { FieldType } from "./SqlFilter";
const {modelValue, type} =  defineProps<{
    modelValue: string | number,
    type: FieldType
}>()

const emits = defineEmits<{
    (eventKey: 'update:modelValue', val: string | number): void
}>()

const inputType = computed(() => typeof modelValue == "string" ? "text" : "number")
const val = ref(modelValue) // v-model 不能多层组件


</script>

<template>
    <select v-if="typeof type == 'object'" v-model="val" @change="$emit('update:modelValue', val)">
        <option v-for="(v, k) in type" :key="k" :value="v">{{ v }}</option>
    </select>
    <input v-else :type="inputType" v-model="val" @change="$emit('update:modelValue', val)">
</template>