<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'

import { greet, fibonacci } from '@jobinjia/lodash-wasm'
import {onMounted} from "vue";

function fibonacciTs(n: number): number {
  // 创建一个数组来存储 Fibonacci 数值
  const fib: number[] = new Array(n + 1).fill(0);

  // 基础情况
  fib[0] = 0; // Fibonacci(0) = 0
  if (n > 0) {
    fib[1] = 1; // Fibonacci(1) = 1
  }

  // 使用动态规划填充数组
  for (let i = 2; i <= n; i++) {
    fib[i] = fib[i - 1] + fib[i - 2];
  }

  return fib[n];
}

function test() {
  const testNum = 42
  console.time('rust version')
  const rr = fibonacci(testNum)
  console.log('rust data = ', rr)
  console.timeEnd('rust version')
  console.log('------------------')
  console.time('ts version')
  const tr = fibonacciTs(testNum)
  console.log('ts data = ', tr)
  console.timeEnd('ts version')
}

onMounted(() => {
  for(let i = 0; i < 10; i++) {
    console.log(`%c 第${i+1}次: `, 'background: blue; color: #fff')
    test()
  }
})
</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <HelloWorld msg="You did it!" />

      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;

    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
