<script lang="ts">
  import '../app.css';
  let { children } = $props();
</script>

<svelte:head>
  <title>CSAC Timetable Studio — Xếp Lịch Tập Tự Động</title>
</svelte:head>

{@render children()}
