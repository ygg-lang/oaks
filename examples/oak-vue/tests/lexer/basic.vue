<script setup lang="ts">
import { ref, computed, onMounted, useSlots, useAttrs, watch } from 'vue'

// Props definition
interface Props {
    msg: string
    initialCount?: number
    items?: string[]
}

const props = withDefaults(defineProps<Props>(), {
    initialCount: 0,
    items: () => []
})

// Emits
const emit = defineEmits<{
  (e: 'update:count', value: number): void
  (e: 'change', value: string): void
}>()

// State
const count = ref(props.initialCount)
const doubleCount = computed(() => count.value * 2)
const title = ref('Vue Component')

// Lifecycle
onMounted(() => {
    console.log(`Component mounted with msg: ${props.msg}`)
})

// Watchers
watch(count, (newVal, oldVal) => {
    emit('update:count', newVal)
    if (newVal > 10) {
        console.warn('Count is high')
    }
})

// Methods
function increment() {
    count.value++
}

function decrement() {
    count.value--
}

// Expose public methods
defineExpose({
    increment,
    count
})

const slots = useSlots()
const attrs = useAttrs()
</script>

<script lang="ts">
// Normal script block
export default {
    name: 'MyComponent',
    inheritAttrs: false
}
</script>

<template>
    <div class="card" :class="{ active: count > 0 }">
        <header>
            <h1>{{ msg }}</h1>
            <h2>{{ title }}</h2>
        </header>
        
        <main>
            <p v-bind="$attrs">
                Count is: {{ count }}
                Double is: {{ doubleCount }}
            </p>
            
            <div class="controls">
                <button type="button" @click="increment" :disabled="count >= 20">
                    Increment
                </button>
                <button type="button" @click.prevent="decrement">
                    Decrement
                </button>
            </div>
            
            <div v-if="count > 5" class="warning">
                Count is getting high!
            </div>
            <div v-else-if="count < 0" class="error">
                Count is negative!
            </div>
            <div v-else>
                Count is normal.
            </div>
            
            <!-- List rendering -->
            <ul>
                <li v-for="(item, index) in items" :key="index">
                    {{ index }} - {{ item }}
                </li>
                <li v-for="n in 5" :key="'num-' + n">
                    Number {{ n }}
                </li>
            </ul>
            
            <!-- Slots -->
            <slot name="header" :count="count">
                Default header content
            </slot>
            
            <div class="content">
                <slot>Default default slot content</slot>
            </div>
            
            <slot name="footer">
                <footer>Default footer</footer>
            </slot>
            
            <!-- Dynamic Components -->
            <component :is="count > 5 ? 'strong' : 'span'">
                Dynamic element
            </component>
            
            <!-- Teleport -->
            <Teleport to="body">
                <div v-if="count > 15" class="modal">
                    High count alert!
                </div>
            </Teleport>
            
            <!-- Suspense -->
            <Suspense>
                <template #default>
                    <AsyncComponent />
                </template>
                <template #fallback>
                    Loading...
                </template>
            </Suspense>
        </main>
    </div>
</template>

<style scoped lang="scss">
$primary-color: #42b883;

.card {
    border: 1px solid #ccc;
    padding: 20px;
    border-radius: 8px;
    
    &.active {
        border-color: $primary-color;
    }
}

.warning {
    color: orange;
    font-weight: bold;
}

.error {
    color: red;
}

:deep(.child-component) {
    background: #f0f0f0;
}

:global(body) {
    margin: 0;
}
</style>

<style module>
.red {
    color: red;
}
</style>
